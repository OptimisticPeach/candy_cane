use crate::slice_tracker::{LockGuard, LockGuardType, SliceTracker};
use crate::RawCandyCane;
use parking_lot::lock_api::RawRwLock;
use parking_lot::RawRwLock as RwLock;
use std::cell::UnsafeCell;
use std::ops::{Bound, RangeBounds};
use super::ChunkVisit;

pub struct RawCandyCaneIterStreaming<'a, R: RawRwLock, T> {
    slices: &'a [SliceTracker<R, T>],
    #[allow(dead_code)]
    all_lock: LockGuard<'a, R>,
    pub(crate) chunks_to_visit: Vec<ChunkVisit>,
    internal: Option<(std::slice::Iter<'a, UnsafeCell<T>>, LockGuard<'a, R>)>,
}

impl<'a, Lock: RawRwLock, T> RawCandyCaneIterStreaming<'a, Lock, T> {
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

        Self {
            slices,
            all_lock: guard,
            chunks_to_visit: chunk_buffer,
            internal: None,
        }
    }

    pub fn next_raw(&mut self, lock_type: LockGuardType) -> Option<*mut T> {
        // println!("Running next");
        match self.internal.as_mut().and_then(|(iter, _)| iter.next()) {
            Some(x) => Some(x.get()),
            None => {
                drop(self.internal.take());
                // First, we try looking for a free chunk to access.
                for index in (0..self.chunks_to_visit.len()).rev() {
                    let chunk = &self.slices[self.chunks_to_visit[index].chunk_id];
                    // println!("{:?} trying {} @ {}", std::thread::current().id(), index, self.chunks_to_visit[index].chunk_id);
                    if let Some(guard) = chunk.try_lock(lock_type) {
                        // println!("{:?} try_lock-ed on {} @ {}", std::thread::current().id(), index, self.chunks_to_visit[index].chunk_id);
                        let slice = unsafe {
                            let slice =
                                std::slice::from_raw_parts(chunk.data.as_ptr(), chunk.length);
                            self.chunks_to_visit[index].slice(slice)
                        };
                        let mut iter = slice.iter();
                        self.chunks_to_visit.remove(index);
                        let item = match iter.next() {
                            Some(x) => x,
                            None => continue,
                        };

                        // println!("{:?} using {}", std::thread::current().id(), index);

                        self.internal = Some((iter, guard));
                        return Some(item.get());
                    }
                }

                // If all of them are occupied, we simply wait on
                // the next available one.
                while let Some(chunk) = self.chunks_to_visit.pop() {
                    // println!("{:?} locking on {}", std::thread::current().id(), chunk.chunk_id);

                    let tracker = &self.slices[chunk.chunk_id];
                    let guard = tracker.lock(lock_type);

                    let slice = unsafe {
                        let slice =
                            std::slice::from_raw_parts(tracker.data.as_ptr(), tracker.length);
                        chunk.slice(slice)
                    };
                    let mut iter = slice.iter();
                    let item = match iter.next() {
                        Some(x) => x,
                        None => continue,
                    };

                    self.internal = Some((iter, guard));
                    return Some(item.get());
                }

                None
            }
        }
    }
}

pub struct CandyCaneIterStreaming<'a, T: Sync, R: RawRwLock = RwLock> {
    pub(crate) inner: RawCandyCaneIterStreaming<'a, R, T>,
}

impl<'a, T: Sync, R: RawRwLock> CandyCaneIterStreaming<'a, T, R> {
    #[inline]
    pub fn next(&mut self) -> Option<&T> {
        // SAFETY: The internal iterator should only
        // ever be called with `LockGuardType::Read`
        self.inner
            .next_raw(LockGuardType::Read)
            .map(|x| unsafe { &*x })
    }
}

pub struct CandyCaneIterStreamingMut<'a, T: Send, R: RawRwLock = RwLock> {
    pub(crate) inner: RawCandyCaneIterStreaming<'a, R, T>,
}

impl<'a, T: Send, R: RawRwLock> CandyCaneIterStreamingMut<'a, T, R> {
    #[inline]
    pub fn next(&mut self) -> Option<&mut T> {
        // SAFETY: The internal iterator should only
        // ever be called with `LockGuardType::Write`
        self.inner
            .next_raw(LockGuardType::Write)
            .map(|x| unsafe { &mut *x })
    }
}
