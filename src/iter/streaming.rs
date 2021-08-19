use crate::slice_tracker::{LockGuard, LockGuardType, SliceTracker};
use crate::RawCandyCane;
use parking_lot::lock_api::RawRwLock;
use parking_lot::RawRwLock as RwLock;
use std::cell::UnsafeCell;
use std::ops::{Bound, RangeBounds};
use super::ChunkVisit;
use crate::iter::ChunkVisitRange;
use std::collections::HashMap;
use atomic_deque::{AtomicDeque, Link};

#[inline]
#[cold]
fn cold() {}

fn id() -> String {
    format!("{:?}", std::thread::current().id())
}

pub struct RawCandyCaneIterStreaming<'a, R: RawRwLock, T, const N: usize> {
    slices: HashMap<usize, ChunkVisitRange>,
    slice_buffer: &'a AtomicDeque<SliceTracker<R>, N>,
    #[allow(dead_code)]
    all_lock: LockGuard<'a, R>,
    all_data: &'a [UnsafeCell<T>],
    internal: Option<(std::slice::Iter<'a, UnsafeCell<T>>, Link<SliceTracker<R>>)>,
}

impl<'a, Lock: RawRwLock, T, const N: usize> RawCandyCaneIterStreaming<'a, Lock, T, N> {
    pub fn new_over<R: RangeBounds<usize>>(
        range: R,
        buffer: &'a RawCandyCane<Lock, T, N>,
    ) -> Self {
        let guard = buffer.lock_internal_for_read();

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

        let (ranges, start_slice) = if start == end {
            (vec![], 0)
        } else {
            unsafe {
                ChunkVisit::create_range(start, end, &buffer)
            }
        };

        let slices = ranges.into_iter().enumerate().map(|(i, r)| (i + start_slice, r)).collect();
        // println!("Slices are {:?} on {}", &slices, id());
        Self {
            slices,
            slice_buffer: &buffer.slices,
            all_lock: guard,
            all_data: unsafe {
                &*buffer.data.get()
            },
            internal: None,
        }
    }

    pub fn next_raw(&mut self, lock_type: LockGuardType) -> Option<*mut T> {
        // println!("Running next");
        match self.internal.as_mut().and_then(|(iter, _)| iter.next()) {
            Some(x) => Some(x.get()),
            None => {
                cold();
                self
                    .internal
                    .take()
                    .map(|(_, link)| self.slice_buffer.deposit(link));

                if self.slices.len() == 0 {
                    // println!("Slices are empty on {}", id());
                    return None;
                }

                let mut range = None;

                let next_slice = self.slice_buffer.predicate_next_wait(|slice_tracker, index| {
                    // println!("Testing {} on {}", index, id());
                    if self.slices.contains_key(&index) {
                        // println!("Succeeded on {} on {}", index, id());
                        range = self.slices.remove(&index);
                        if slice_tracker.length == 0 {
                            // println!("Chunk length was 0 on {}", id());
                            false
                        } else {
                            true
                        }
                    } else {
                        false
                    }
                });

                if let (Some(range), Some(chunk)) = (range, next_slice) {
                    let slice = range.slice(chunk.start, chunk.length, &self.all_data);

                    let mut iter = slice.iter();

                    let next = iter.next();
                    if next.is_none() {
                        cold();
                    }
                    let item = match next {
                        Some(x) => x,
                        None => panic!(),
                    };

                    self.internal = Some((iter, chunk));
                    return Some(item.get());
                }

                // println!("Range and chunk were None on {}: {:?}", id(), &self.slices);
                None
            }
        }
    }
}

pub struct CandyCaneIterStreaming<'a, T: Sync, R: RawRwLock, const N: usize> {
    pub(crate) inner: RawCandyCaneIterStreaming<'a, R, T, N>,
}

impl<'a, T: Sync, R: RawRwLock, const N: usize> CandyCaneIterStreaming<'a, T, R, N> {
    #[inline]
    pub fn next(&mut self) -> Option<&T> {
        // SAFETY: The internal iterator should only
        // ever be called with `LockGuardType::Read`
        self.inner
            .next_raw(LockGuardType::Read)
            .map(|x| unsafe { &*x })
    }
}

pub struct CandyCaneIterStreamingMut<'a, T: Send, R: RawRwLock, const N: usize> {
    pub(crate) inner: RawCandyCaneIterStreaming<'a, R, T, N>,
}

impl<'a, T: Send, R: RawRwLock, const N: usize> CandyCaneIterStreamingMut<'a, T, R, N> {
    #[inline]
    pub fn next(&mut self) -> Option<&mut T> {
        // SAFETY: The internal iterator should only
        // ever be called with `LockGuardType::Write`
        self.inner
            .next_raw(LockGuardType::Write)
            .map(|x| unsafe { &mut *x })
    }
}
