#![feature(bench_black_box)]

use std::sync::Arc;
use candy_cane::RawCandyCane;
use parking_lot::RawRwLock;
use std::hint::black_box;

fn main() {
    let data = (0..500000usize).collect::<Vec<_>>();
    for _ in 0..8000 {
        let data = data.clone();

        let cane = Arc::new(RawCandyCane::<RawRwLock, usize, 1>::from_vec(data));

        let mut thread_handles = Vec::with_capacity(8);

        let cane = black_box(&cane);

        for _ in 0..8 {
            let cane = black_box(cane.clone());
            thread_handles.push(std::thread::spawn(
                move || {
                    let mut iter = cane.iter_streaming_mut(..);

                    while let Some(val) = iter.next() {
                        black_box(*val);
                    }
                }
            ));
        }

        while let Some(t) = thread_handles.pop() {
            t.join().unwrap();
        }
    }
}
