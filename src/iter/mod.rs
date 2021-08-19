use parking_lot::lock_api::RawRwLock;
use crate::RawCandyCane;
use crate::slice_tracker::SliceTracker;
use std::sync::atomic::Ordering;
use std::cell::UnsafeCell;

// pub mod normal;
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
    // SAFETY: Assumes we have an active read lock for the candy cane.
    pub unsafe fn create_range<Lock: RawRwLock, T, const SLICES: usize>(
        start: usize,
        end: usize,
        buffer: &RawCandyCane<Lock, T, SLICES>,
    ) -> (Vec<ChunkVisitRange>, usize) {
        let start_slice = buffer.calc_slice_index(start);
        let mut end_slice = buffer.calc_slice_index(end);
        assert!(end_slice >= start_slice);
        assert!(end_slice <= SLICES);

        // Adjust if we are dealing with the last (and longer) slice.
        if end_slice == SLICES {
            end_slice = end_slice - 1;
        }

        if start_slice == end_slice {
            let range = ChunkVisitRange::Inside(start, end);
            return (vec![range], start_slice);
        }

        let ranges = (start_slice..=end_slice)
            .map(|slice_index| {
                if slice_index == start_slice {
                    ChunkVisitRange::Last { start }
                } else if slice_index == end_slice {
                    ChunkVisitRange::First { end }
                } else {
                    ChunkVisitRange::All
                }
            })
            .collect();

        (ranges, start_slice)
    }
}

impl ChunkVisitRange {
    pub(crate) fn slice<'a, T>(&self, start: usize, length: usize, slice: &'a [T]) -> &'a [T] {
        match *self {
            ChunkVisitRange::All => &slice[start..start + length],
            ChunkVisitRange::Inside(s, e) => &slice[s..=e],
            ChunkVisitRange::First { end: r_end } => &slice[start..=r_end],
            ChunkVisitRange::Last { start: r_start } => &slice[r_start..start + length],
        }
    }
}
