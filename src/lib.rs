pub mod iter;
pub mod raw;
mod slice_tracker;

use crate::iter::streaming::{CandyCaneIterStreaming, CandyCaneIterStreamingMut};
use crate::raw::RawCandyCaneIterStreaming;
use crate::slice_tracker::{SliceTracker, LockGuard, LockGuardType};
use parking_lot::lock_api::RawRwLock;
use parking_lot::{Condvar, Mutex};
use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::ops::{RangeBounds, DerefMut, Deref};
use std::marker::PhantomData;
use std::sync::atomic::{AtomicUsize, Ordering};
use atomic_deque::AtomicDeque;
// use crate::iter::normal::{CandyCaneIter, RawCandyCaneIter, CandyCaneIterMut};

pub type CandyCane<T> = RawCandyCane<parking_lot::RawRwLock, T, 6>;

/// SAFETY: Every element in `from` must be initialized
unsafe fn assume_init_array<T, const LEN: usize>(from: [MaybeUninit<T>; LEN]) -> [T; LEN] {
    let val = std::mem::transmute_copy::<_, [T; LEN]>(&from);
    std::mem::forget(from);
    val
}

pub struct RawCandyCane<R: RawRwLock, T, const SLICES: usize> {
    data: UnsafeCell<Vec<UnsafeCell<T>>>,
    slices: AtomicDeque<SliceTracker<R>, SLICES>,
    /// Self explanatory (len / SLICES).
    len_per_slice: AtomicUsize,
    // SAFETY: `all_lock` must be boxed to ensure
    // that the pointers in the `SliceTracker`s
    // remain valid even after this `RawCandyCane`
    // is moved.
    all_lock: R,
    is_waiting_mut: Mutex<bool>,
    waiting_mut_wakeup: Condvar,
}

impl<R: RawRwLock, T, const SLICES: usize> RawCandyCane<R, T, SLICES> {
    pub fn new() -> Self {
        assert_ne!(SLICES, 0);

        let data = Vec::<UnsafeCell<T>>::new();

        let rwlock = R::INIT;

        // SAFETY: `MaybeUninit` does not require initialization.
        let mut slices: [MaybeUninit<SliceTracker<R>>; SLICES] =
            unsafe { MaybeUninit::uninit().assume_init() };

        for slice in &mut slices[..] {
            // SAFETY: None of the `SliceTracker`s will overlap
            // since they all have length 0.
            unsafe {
                *slice = MaybeUninit::new(SliceTracker::new(0, 0));
            }
        }

        // SAFETY: `slices` was initialized by the previous
        // `for` loop.
        let slices = unsafe { assume_init_array(slices) };

        Self {
            data: UnsafeCell::new(data),
            slices: AtomicDeque::new(slices),
            len_per_slice: AtomicUsize::new(0),
            all_lock: rwlock,
            is_waiting_mut: Mutex::new(false),
            waiting_mut_wakeup: Condvar::new(),
        }
    }

    pub fn from_vec(mut data: Vec<T>) -> Self {
        assert_ne!(SLICES, 0);

        if data.len() == 0 {
            return Self::new();
        }

        let data = {
            let len = data.len();
            let ptr = data.as_mut_ptr() as *mut UnsafeCell<T>;
            let cap = data.capacity();

            std::mem::forget(data);

            unsafe { Vec::from_raw_parts(ptr, len, cap) }
        };

        let rwlock = R::INIT;

        let (slices, per_slice) = Self::create_slices(&data);

        Self {
            data: UnsafeCell::new(data),
            len_per_slice: AtomicUsize::new(per_slice),
            slices: AtomicDeque::new(slices),
            all_lock: rwlock,
            is_waiting_mut: Mutex::new(false),
            waiting_mut_wakeup: Condvar::new(),
        }
    }

    pub fn len(&self) -> usize {
        let lock = self.lock_internal_for_read();

        // SAFETY: `all_lock` is shared currently,
        // and therefore no writes directly to the
        // vec should be occurring, making .len()
        // a safe operation.
        let len = unsafe { (&*self.data.get()).len() };

        drop(lock);

        len
    }

    pub fn write(&self) -> CandyCaneWriteGuard<R, T, SLICES> {
        *self.is_waiting_mut.lock() = true;
        let guard = LockGuard::lock(&self.all_lock, LockGuardType::Write);
        self.waiting_mut_wakeup.notify_all();

        let vec = self.data.get();
        let reconstructed_vec = unsafe {
            let ptr = (*vec).as_mut_ptr().cast::<T>();
            let len = (*vec).len();
            let cap = (*vec).capacity();

            Vec::from_raw_parts(ptr, len, cap)
        };

        CandyCaneWriteGuard {
            lock: guard,
            vec: reconstructed_vec,
            original: self,
            _phantom: PhantomData,
        }
    }

    pub fn into_inner(self) -> Vec<T> {
        // Sanity check
        *self.is_waiting_mut.lock() = true;
        LockGuard::try_lock(&self.all_lock, LockGuardType::Write).unwrap();

        let mut original = self.data.into_inner();
        let reconstructed_vec = unsafe {
            let ptr = original.as_mut_ptr().cast::<T>();
            let len = original.len();
            let cap = original.capacity();

            Vec::from_raw_parts(ptr, len, cap)
        };

        std::mem::forget(original);

        reconstructed_vec
    }

    pub(crate) fn reconstruct_chunks<'a>(&'a self, lock: &LockGuard<'a, R>) {
        assert!(self.ensure_my_write_guard(&lock));
        assert_eq!(self.slices.len(), SLICES);

        // SAFETY: We ensured that the lock we were given is for our lock.
        let data = unsafe { &*self.data.get() };

        let (slices, per_chunk) = Self::create_slices(&data);
        let mut slices = slices.map(|x| Some(x));

        let mut links = Vec::new();

        while let Some(mut link) = self.slices.next_try() {
            *link = slices[link.original_index()].take().unwrap();
            links.push(link);
        }

        links
            .into_iter()
            .for_each(|x| self.slices.deposit(x));

        // Does ordering matter here? Since nothing should
        // be reading this value right now.
        self.len_per_slice.store(per_chunk, Ordering::Release);
    }

    fn create_slices(data: &Vec<UnsafeCell<T>>) -> ([SliceTracker<R>; SLICES], usize) {
        // SAFETY: `MaybeUninit` does not require initialization.
        let mut slices: [MaybeUninit<SliceTracker<R>>; SLICES] =
            unsafe { MaybeUninit::uninit().assume_init() };

        let per_slice = data.len() / SLICES;
        let last_extra = data.len() % SLICES;
        let mut running_total = 0;

        for slice in &mut slices[..SLICES - 1] {
            // SAFETY: None of the `SliceTracker`s should overlap
            unsafe {
                *slice = MaybeUninit::new(SliceTracker::new(
                    running_total,
                    per_slice,
                ));
            }

            running_total += per_slice;
        }

        // SAFETY: None of the `SliceTracker`s should overlap with
        // the last one.
        unsafe {
            slices[SLICES - 1] = MaybeUninit::new(SliceTracker::new(
                running_total,
                per_slice + last_extra,
            ));
        }

        // SAFETY: `slices` was initialized by the previous
        // `for` loop.
        let slices = unsafe { assume_init_array(slices) };

        (slices, per_slice)
    }

    pub(crate) fn lock_internal_for_read(&self) -> LockGuard<'_, R> {
        let mut lock = self.is_waiting_mut.lock();
        while *lock {
            self.waiting_mut_wakeup.wait(&mut lock);
        }

        // This is safe because lock is still alive, and another thread
        // could not have already started to request
        LockGuard::lock(&self.all_lock, LockGuardType::Read)
    }

    pub(crate) fn calc_slice_index(&self, index: usize) -> usize {
        let len_per_slice = self.len_per_slice.load(Ordering::Acquire);
        if len_per_slice == 0 {
            SLICES
        } else {
            index / len_per_slice
        }
    }

    fn ensure_my_write_guard<'a>(&'a self, guard: &LockGuard<'a, R>) -> bool {
        matches!(guard.kind, LockGuardType::Write) &&
            std::ptr::eq(guard.rwlock as _, &self.all_lock as _)
    }
}

impl<R: RawRwLock, T: Sync, const SLICES: usize> RawCandyCane<R, T, SLICES> {
    pub fn iter_streaming(&self, range: impl RangeBounds<usize>) -> CandyCaneIterStreaming<'_, T, R, SLICES> {
        let internal = RawCandyCaneIterStreaming::new_over(range, self);
        CandyCaneIterStreaming { inner: internal }
    }

    // pub fn iter(&self, range: impl RangeBounds<usize>) -> CandyCaneIter<'_, T, R> {
    //     let internal = RawCandyCaneIter::new_over(range, self);
    //     CandyCaneIter { inner: internal }
    // }
}

impl<R: RawRwLock, T: Send, const SLICES: usize> RawCandyCane<R, T, SLICES> {
    pub fn iter_streaming_mut(&self, range: impl RangeBounds<usize>) -> CandyCaneIterStreamingMut<'_, T, R, SLICES> {
        let internal = RawCandyCaneIterStreaming::new_over(range, self);
        CandyCaneIterStreamingMut { inner: internal }
    }

    // pub fn iter_mut(&self, range: impl RangeBounds<usize>) -> CandyCaneIterMut<'_, T, R> {
    //     let internal = RawCandyCaneIter::new_over(range, self);
    //     CandyCaneIterMut { inner: internal }
    // }
}

unsafe impl<R: RawRwLock, T, const SLICES: usize> Sync for RawCandyCane<R, T, SLICES> {}
unsafe impl<R: RawRwLock, T, const SLICES: usize> Send for RawCandyCane<R, T, SLICES> {}

pub struct CandyCaneWriteGuard<'a, R: RawRwLock, T, const SLICES: usize> {
    lock: LockGuard<'a, R>,
    original: &'a RawCandyCane<R, T, SLICES>,
    vec: Vec<T>,
    _phantom: PhantomData<&'a mut Vec<UnsafeCell<T>>>
}

impl<'a, R: RawRwLock, T, const SLICES: usize> Deref for CandyCaneWriteGuard<'a, R, T, SLICES> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.vec
    }
}

impl<'a, R: RawRwLock, T, const SLICES: usize> DerefMut for CandyCaneWriteGuard<'a, R, T, SLICES> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.vec
    }
}

impl<'a, R: RawRwLock, T, const SLICES: usize> Drop for CandyCaneWriteGuard<'a, R, T, SLICES> {
    fn drop(&mut self) {
        let reconstructed_vec = unsafe {
            let ptr = self.vec.as_mut_ptr().cast::<UnsafeCell<T>>();
            let len = self.vec.len();
            let cap = self.vec.capacity();

            Vec::from_raw_parts(ptr, len, cap)
        };

        std::mem::forget(std::mem::take(&mut self.vec));

        self.original.ensure_my_write_guard(&self.lock);
        unsafe {
            *self.original.data.get() = reconstructed_vec;
        }

        self.original.reconstruct_chunks(&self.lock);
    }
}

#[cfg(test)]
mod unit_tests {
    use crate::RawCandyCane;
    use hushed_panic::hush_this_test;
    use parking_lot::RawRwLock;
    use parking_lot::lock_api::RawRwLock as RRwlock;
    use std::sync::Arc;

    #[test]
    fn new() {
        RawCandyCane::<RawRwLock, (), 1>::new();
        RawCandyCane::<RawRwLock, u8, 1>::new();
        RawCandyCane::<RawRwLock, (), 100>::new();
        RawCandyCane::<RawRwLock, u8, 100>::new();
    }

    #[test]
    fn from_vec() {
        let unit_vec = vec![(); 90];
        let u8_vec = vec![0u8; 90];
        RawCandyCane::<RawRwLock, (), 1>::from_vec(unit_vec.clone());
        RawCandyCane::<RawRwLock, u8, 1>::from_vec(u8_vec.clone());
        RawCandyCane::<RawRwLock, (), 100>::from_vec(unit_vec);
        RawCandyCane::<RawRwLock, u8, 100>::from_vec(u8_vec);
    }

    #[test]
    #[should_panic]
    fn zero_slices() {
        let _x = hush_this_test();
        RawCandyCane::<RawRwLock, (), 0>::new();
    }

    fn make_data() -> Vec<usize> {
        (0..4000).collect()
    }

    fn iter_and_add<R: RRwlock, const SLICES: usize>(candy_cane: &RawCandyCane<R, usize, SLICES>) {
        let mut iter = candy_cane.iter_streaming(..);
        let mut sum = 0;
        let mut count = 0;
        let mut last = 0;
        while let Some(item) = iter.next() {
            sum += *item;
            count += 1;
            last = *item;
        }

        assert_eq!(count, 4000);
        assert_eq!(sum, (3999 * 4000) / 2);

        // let (sum, count) = candy_cane
        //     .iter(..)
        //     .fold((0, 0), |(sum, count), val| (sum + *val, count + 1));

        assert_eq!(count, 4000);
        assert_eq!(sum, (3999 * 4000) / 2);
    }

    fn assure_final_state<R: RRwlock, const SLICES: usize>(candy_cane: &RawCandyCane<R, usize, SLICES>) {
        assert!(candy_cane.all_lock.try_lock_exclusive());
        unsafe {
            candy_cane.all_lock.unlock_exclusive();
        }
    }

    #[test]
    fn iterate_1_single_threaded() {
        let data = make_data();

        let candy_cane = RawCandyCane::<RawRwLock, _, 1>::from_vec(data);

        iter_and_add(&candy_cane);
        assure_final_state(&candy_cane);
    }

    #[test]
    fn iterate_3_single_threaded() {
        let data = make_data();

        let candy_cane = RawCandyCane::<RawRwLock, _, 3>::from_vec(data);

        iter_and_add(&candy_cane);
        assure_final_state(&candy_cane);
    }

    #[test]
    fn iterate_1_multi_threaded() {
        let data = make_data();

        let candy_cane = Arc::new(RawCandyCane::<RawRwLock, _, 1>::from_vec(data));

        let threads = (0..4)
            .map(|_| {
                let clone = Arc::clone(&candy_cane);
                std::thread::spawn(move || {
                    // println!("Created {:?}", std::thread::current().id());
                    iter_and_add(&*clone);
                    // println!("Finished {:?}", std::thread::current().id());
                })
            })
            .collect::<Vec<_>>();

        threads
            .into_iter()
            .for_each(|x| x.join().unwrap());

        assure_final_state(&candy_cane);
    }

    #[test]
    fn iterate_3_multi_threaded() {
        let data = make_data();

        let candy_cane = Arc::new(RawCandyCane::<RawRwLock, _, 3>::from_vec(data));

        let threads = (0..7)
            .map(|_| {
                let clone = Arc::clone(&candy_cane);
                std::thread::spawn(move || {
                    iter_and_add(&*clone);
                })
            })
            .collect::<Vec<_>>();

        threads
            .into_iter()
            .for_each(|x| x.join().unwrap());

        assure_final_state(&candy_cane);
    }

    fn iter_and_add_mut<R: RRwlock, const SLICES: usize>(candy_cane: &RawCandyCane<R, usize, SLICES>) {
        let mut iter = candy_cane.iter_streaming_mut(..);
        let mut sum = 0;
        let mut count = 0;
        while let Some(item) = iter.next() {
            sum += *item;
            count += 1;
        }

        assert_eq!(count, 4000);
        assert_eq!(sum, (3999 * 4000) / 2);

        // let (sum, count) = candy_cane
        //     .iter_mut(..)
        //     .fold((0, 0), |(sum, count), val| (sum + *val, count + 1));

        assert_eq!(count, 4000);
        assert_eq!(sum, (3999 * 4000) / 2);
    }

    #[test]
    fn iterate_1_single_threaded_mut() {
        let data = make_data();

        let candy_cane = RawCandyCane::<RawRwLock, _, 1>::from_vec(data);

        iter_and_add_mut(&candy_cane);
        assure_final_state(&candy_cane);
    }

    #[test]
    fn iterate_3_single_threaded_mut() {
        let data = make_data();

        let candy_cane = RawCandyCane::<RawRwLock, _, 3>::from_vec(data);

        iter_and_add_mut(&candy_cane);
        assure_final_state(&candy_cane);
    }

    #[test]
    fn iterate_1_multi_threaded_mut() {
        let data = make_data();

        let candy_cane = Arc::new(RawCandyCane::<RawRwLock, _, 1>::from_vec(data));

        let threads = (0..4)
            .map(|_| {
                let clone = Arc::clone(&candy_cane);
                std::thread::spawn(move || {
                    iter_and_add_mut(&*clone);
                })
            })
            .collect::<Vec<_>>();

        threads
            .into_iter()
            .for_each(|x| x.join().unwrap());

        assure_final_state(&candy_cane);
    }

    #[test]
    fn iterate_3_multi_threaded_mut() {
        let data = make_data();

        let candy_cane = Arc::new(RawCandyCane::<RawRwLock, _, 3>::from_vec(data));

        let threads = (0..7)
            .map(|_| {
                let clone = Arc::clone(&candy_cane);
                std::thread::spawn(move || {
                    // println!("{:?} Started", std::thread::current().id());
                    iter_and_add_mut(&*clone);
                    // println!("{:?} Ended", std::thread::current().id());
                })
            })
            .collect::<Vec<_>>();

        threads
            .into_iter()
            .for_each(|x| x.join().unwrap());

        assure_final_state(&candy_cane);
    }
}
