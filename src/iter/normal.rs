use crate::slice_tracker::{LockGuard, LockGuardType, SliceTracker};
use crate::RawCandyCane;
use parking_lot::lock_api::RawRwLock;
use parking_lot::RawRwLock as RwLock;
use std::cell::UnsafeCell;
use std::ops::{Bound, RangeBounds, Deref, DerefMut};
use std::sync::Arc;
use crate::iter::ChunkVisit;
use std::mem::{ManuallyDrop, MaybeUninit};

pub(crate) trait Guard {}

impl<'a, R: RawRwLock> Guard for (Arc<LockGuard<'a, R>>, LockGuard<'a, R>) {}

pub struct Ref<'a, T: Sync> {
    item: &'a T,
    #[allow(dead_code)]
    live: Arc<dyn Guard + 'a>,
}

impl<'a, T: Sync> Deref for Ref<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.item
    }
}

pub struct Mut<'a, T: Send> {
    item: &'a mut T,
    #[allow(dead_code)]
    live: Arc<dyn Guard + 'a>,
}

impl<'a, T: Send> Deref for Mut<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.item
    }
}

impl<'a, T: Send> DerefMut for Mut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.item
    }
}

pub struct Raw<'a, T> {
    item: *mut T,
    live: Arc<dyn Guard + 'a>,
}

impl<'a, T> Raw<'a, T> {
    pub unsafe fn upgrade_ref(self) -> Ref<'a, T>
        where T: Sync {
        Ref {
            item: &*self.item,
            live: self.live
        }
    }

    pub unsafe fn upgrade_mut(self) -> Mut<'a, T>
        where T: Send {
        Mut {
            item: &mut *self.item,
            live: self.live
        }
    }
}

fn create_guard<'a, R: RawRwLock>(all_guard: Arc<LockGuard<'a, R>>, my_guard: LockGuard<'a, R>) -> Arc<dyn Guard + 'a>  {
    let temp_arc = Arc::new((all_guard, my_guard));
    let ptr = Arc::into_raw(temp_arc);
    let ptr = unsafe {
        &*ptr as &(dyn Guard + 'a) as *const (dyn Guard + 'a)
    };

    unsafe {
        Arc::from_raw(ptr)
    }
}

pub(crate) enum RawCandyCaneIter<'a, R: RawRwLock, T> {
    Built {
        slices: &'a [SliceTracker<R, T>],
        all_lock: Arc<LockGuard<'a, R>>,
        chunks_to_visit: Vec<ChunkVisit>,
    },
    Alive {
        slices: &'a [SliceTracker<R, T>],
        all_lock: Arc<LockGuard<'a, R>>,
        chunks_to_visit: Vec<ChunkVisit>,
        internal: std::slice::Iter<'a, UnsafeCell<T>>,
        chunk_guard: ManuallyDrop<Arc<dyn Guard + 'a>>,
    },
    Dead,
}

impl<'a, Lock: RawRwLock, T> RawCandyCaneIter<'a, Lock, T> {
    pub fn new_over<R: RangeBounds<usize>, const SLICES: usize>(
        range: R,
        buffer: &'a RawCandyCane<Lock, T, SLICES>,
    ) -> Self {
        let guard = buffer.lock_internal_for_read();

        let mut chunk_buffer = Vec::new();

        let start = range.start_bound();
        let end = range.end_bound();
        let len = buffer.len() - 1;

        let (start, end) = match (start, end) {
            (Bound::Excluded(&s), Bound::Excluded(&e)) => (s + 1, e + 1),
            (Bound::Excluded(&s), Bound::Included(&e)) => (s + 1, e),
            (Bound::Excluded(&s), Bound::Unbounded) => (s + 1, len),
            (Bound::Included(&s), Bound::Excluded(&e)) => (s, e + 1),
            (Bound::Included(&s), Bound::Included(&e)) => (s, e),
            (Bound::Included(&s), Bound::Unbounded) => (s, len),
            (Bound::Unbounded, Bound::Excluded(&e)) => (0, e + 1),
            (Bound::Unbounded, Bound::Included(&e)) => (0, e),
            (Bound::Unbounded, Bound::Unbounded) => (0, len),
        };

        assert!(start <= end);
        assert!(end <= len);

        let slices = if start == end {
            &[][..]
        } else {
            ChunkVisit::create_range(start, end, &mut chunk_buffer, &buffer)
        };

        Self::Built {
            slices,
            all_lock: Arc::new(guard),
            chunks_to_visit: chunk_buffer,
        }
    }

    pub fn next_raw(&mut self, lock_type: LockGuardType) -> Option<Raw<'a, T>> {
        match self {
            RawCandyCaneIter::Built {
                slices,
                all_lock,
                chunks_to_visit
            } => {
                let next = Self::try_find_next(slices, all_lock.clone(), chunks_to_visit, lock_type);
                let next = next.or_else(|| Self::wait_find_next(slices, all_lock.clone(), chunks_to_visit, lock_type));

                if let Some((mut iter, guard)) = next {
                    let slices = &**slices;
                    let all_lock = all_lock.clone();
                    let chunks_to_visit = std::mem::take(chunks_to_visit);

                    let next = iter.next().unwrap();

                    *self = RawCandyCaneIter::Alive {
                        slices,
                        all_lock,
                        chunks_to_visit,
                        internal: iter,
                        chunk_guard: ManuallyDrop::new(guard.clone()),
                    };

                    Some(Raw {
                        item: next.get(),
                        live: guard
                    })
                } else {
                    *self = RawCandyCaneIter::Dead;
                    None
                }
            },
            RawCandyCaneIter::Alive {
                slices,
                all_lock,
                chunks_to_visit,
                internal,
                chunk_guard
            } => {
                match internal.next() {
                    Some(next) => Some(Raw {
                        item: next.get(),
                        live: Arc::clone(&*chunk_guard),
                    }),
                    None => {
                        let chunk_guard = unsafe {
                            let ptr = chunk_guard as *mut ManuallyDrop<Arc<dyn Guard + 'a>>;
                            let ptr = ptr.cast::<MaybeUninit<ManuallyDrop<Arc<dyn Guard + 'a>>>>();
                            let ptr = &mut *ptr;

                            ManuallyDrop::drop(&mut *(ptr.as_mut_ptr()));

                            ptr
                        };

                        let next = Self::try_find_next(slices, all_lock.clone(), chunks_to_visit, lock_type);
                        let next = next.or_else(|| Self::wait_find_next(slices, all_lock.clone(), chunks_to_visit, lock_type));

                        if let Some((mut iter, guard)) = next {
                            let next = iter.next().unwrap();

                            chunk_guard.write(ManuallyDrop::new(guard.clone()));
                            *internal = iter;

                            Some(Raw {
                                item: next.get(),
                                live: guard
                            })
                        } else {
                            *self = RawCandyCaneIter::Dead;
                            None
                        }
                    }
                }
            },
            RawCandyCaneIter::Dead => None
        }
    }

    fn try_find_next(
        slices: &'a [SliceTracker<Lock, T>],
        all_lock: Arc<LockGuard<'a, Lock>>,
        chunks_to_visit: &mut Vec<ChunkVisit>,
        lock_type: LockGuardType,
    ) -> Option<(std::slice::Iter<'a, UnsafeCell<T>>, Arc<dyn Guard + 'a>)> {
        // println!("{:?} trying", std::thread::current().id());
        for index in (0..chunks_to_visit.len()).rev() {
            // println!("{:?} trying {} @ {}", std::thread::current().id(), index, chunks_to_visit[index].chunk_id);

            let chunk = &slices[chunks_to_visit[index].chunk_id];

            if let Some(guard) = chunk.try_lock(lock_type) {
                // println!("{:?} accept {} @ {}", std::thread::current().id(), index, chunks_to_visit[index].chunk_id);

                let slice = unsafe {
                    let slice =
                        std::slice::from_raw_parts(chunk.data.as_ptr(), chunk.length);
                    chunks_to_visit[index].slice(slice)
                };

                chunks_to_visit.remove(index);

                if slice.len() == 0 {
                    // println!("{:?} reject {} @ {}", std::thread::current().id(), index, old_chunk.chunk_id);
                    continue;
                }

                let iter = slice.iter();

                let guard = create_guard(all_lock, guard);

                return Some((iter, guard));
            }
        }

        None
    }

    fn wait_find_next(
        slices: &'a [SliceTracker<Lock, T>],
        all_lock: Arc<LockGuard<'a, Lock>>,
        chunks_to_visit: &mut Vec<ChunkVisit>,
        lock_type: LockGuardType,
    ) -> Option<(std::slice::Iter<'a, UnsafeCell<T>>, Arc<dyn Guard + 'a>)> {
        // println!("{:?} locking", std::thread::current().id());
        for index in (0..chunks_to_visit.len()).rev() {
            // println!("{:?} locking {} @ {}", std::thread::current().id(), index, chunks_to_visit[index].chunk_id);
            let chunk = &slices[chunks_to_visit[index].chunk_id];

            let guard = chunk.lock(lock_type);
            let slice = unsafe {
                let slice =
                    std::slice::from_raw_parts(chunk.data.as_ptr(), chunk.length);
                chunks_to_visit[index].slice(slice)
            };

            chunks_to_visit.remove(index);

            if slice.len() == 0 {
                // println!("{:?} reject {} @ {}", std::thread::current().id(), index, old_chunk.chunk_id);
                continue;
            }

            let iter = slice.iter();

            let guard = create_guard(all_lock, guard);

            return Some((iter, guard));
        }


        None
    }
}

pub struct CandyCaneIter<'a, T: Sync, R: RawRwLock = RwLock> {
    pub(crate) inner: RawCandyCaneIter<'a, R, T>,
}

impl<'a, T: Sync, R: RawRwLock> Iterator for CandyCaneIter<'a, T, R> {
    type Item = Ref<'a, T>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // SAFETY: The internal iterator should only
        // ever be called with `LockGuardType::Read`
        self.inner
            .next_raw(LockGuardType::Read)
            .map(|x| unsafe { x.upgrade_ref() })
    }
}

pub struct CandyCaneIterMut<'a, T: Send, R: RawRwLock = RwLock> {
    pub(crate) inner: RawCandyCaneIter<'a, R, T>,
}

impl<'a, T: Send, R: RawRwLock> Iterator for CandyCaneIterMut<'a, T, R> {
    type Item = Mut<'a, T>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // SAFETY: The internal iterator should only
        // ever be called with `LockGuardType::Write`
        self.inner
            .next_raw(LockGuardType::Write)
            .map(|x| unsafe { x.upgrade_mut() })
    }
}
