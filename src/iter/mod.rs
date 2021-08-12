use parking_lot::lock_api::RawRwLock;
use crate::RawCandyCane;
use crate::slice_tracker::SliceTracker;
use std::sync::atomic::Ordering;
use std::cell::UnsafeCell;

pub mod normal;
pub mod streaming;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum ChunkVisitRange {
    All,
    /// `..self.end`
    First {
        end: usize,
    },
    /// `self.start..`
    Last {
        start: usize,
    },
    /// `self.start..self.end`
    Inside(usize, usize),
}

unsafe fn unsafe_cell_to_ref<T>(x: &[UnsafeCell<T>]) -> &[T] {
    std::slice::from_raw_parts(x.as_ptr().cast(), x.len())
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) struct ChunkVisit {
    chunk_id: usize,
    range: ChunkVisitRange,
}

impl ChunkVisit {
    pub fn create_range<'a, Lock: RawRwLock, T, const SLICES: usize>(
        start: usize,
        end: usize,
        slice_buffer: &mut Vec<Self>,
        buffer: &'a RawCandyCane<Lock, T, SLICES>,
    ) -> &'a [SliceTracker<Lock, T>] {
        let start_slice = buffer.calc_slice_index(start);
        let mut end_slice = buffer.calc_slice_index(end);
        assert!(end_slice >= start_slice);
        assert!(end_slice <= SLICES);

        let len_per_slice = buffer.len_per_slice.load(Ordering::Acquire);

        // Adjust if we are dealing with the last (and longer) slice.
        if end_slice == SLICES {
            end_slice = end_slice - 1;
        }

        if start_slice == end_slice {
            let slices = &buffer.slices[start_slice..=start_slice];
            let slice_offset = start_slice * len_per_slice;
            let start = start - slice_offset;
            let end = end - slice_offset;
            slice_buffer.push(ChunkVisit {
                chunk_id: 0,
                range: ChunkVisitRange::Inside(start, end),
            });
            return unsafe { unsafe_cell_to_ref(slices) };
        }

        let slices = &buffer.slices[start_slice..=end_slice];

        for (buffer_index, slice_index) in (start_slice..=end_slice).enumerate() {
            if slice_index == start_slice {
                let start_slice_offset = start_slice * len_per_slice;
                let start = start - start_slice_offset;
                slice_buffer.push(ChunkVisit {
                    chunk_id: buffer_index,
                    range: ChunkVisitRange::Last { start },
                });
            } else if slice_index == end_slice {
                let end_slice_offset = end_slice * len_per_slice;
                let end = end - end_slice_offset;
                slice_buffer.push(ChunkVisit {
                    chunk_id: buffer_index,
                    range: ChunkVisitRange::First { end },
                });
            } else {
                slice_buffer.push(ChunkVisit {
                    chunk_id: buffer_index,
                    range: ChunkVisitRange::All,
                })
            }
        }

        unsafe {
            // SAFETY: `UnsafeCell` is `repr(transparent)`.
            unsafe_cell_to_ref(slices)
        }
    }

    fn slice<'a, T>(&self, slice: &'a [T]) -> &'a [T] {
        match self.range {
            ChunkVisitRange::All => slice,
            ChunkVisitRange::Inside(s, e) => &slice[s..=e],
            ChunkVisitRange::First { end } => &slice[..=end],
            ChunkVisitRange::Last { start } => &slice[start..],
        }
    }
}
