#![allow(unused_macros, unused_attributes)]
#![feature(test)]
extern crate test;
pub use test::{Bencher, black_box};
pub use candy_cane::RawCandyCane;
pub use parking_lot::RawRwLock;
pub use std::sync::Arc;
pub use parking_lot::RwLock;
pub use criterion::{Criterion, criterion_group, criterion_main};

pub fn make_data<const LEN: usize>() -> Vec<usize> {
    (0..LEN)
        .collect()
}

macro_rules! create_test {
    (@(#candy_cane_iter)(no_threads), $chunks:literal, $iter_func:ident, $datalen:literal, $name:ident) => {
        // #[bench]
        // fn $name(b: &mut Bencher) {
        //     let data = make_data::<$datalen>();
        //
        //     let cane = RawCandyCane::<RawRwLock, usize, $chunks>::from_vec(data);
        //
        //     b.iter(|| {
        //         let cane = black_box(&cane);
        //         cane.$iter_func(..)
        //             .for_each(|val| drop(black_box(*val)));
        //     });
        // }
    };

    (@(#candy_cane_stream)(no_threads), $chunks:literal, $iter_func:ident, $datalen:literal, $name:ident) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let data = make_data::<$datalen>();

            let cane = RawCandyCane::<RawRwLock, usize, $chunks>::from_vec(data);

            b.iter(|| {
                let cane = black_box(&cane);
                let mut iter = cane.$iter_func(..);

                while let Some(val) = iter.next() {
                    black_box(*val);
                }
            });
        }
    };

    (@(#candy_cane_iter)(#$threads:expr), $chunks:literal, $iter_func:ident, $datalen:literal, $name:ident) => {
        // #[bench]
        // fn $name(b: &mut Bencher) {
        //     let data = make_data::<$datalen>();
        //
        //     let cane = Arc::new(RawCandyCane::<RawRwLock, usize, $chunks>::from_vec(data));
        //
        //     let mut thread_handles = Vec::with_capacity($threads);
        //
        //     b.iter(move || {
        //         let cane = black_box(&cane);
        //
        //         for _ in 0..$threads {
        //             let cane = cane.clone();
        //             thread_handles.push(std::thread::spawn(
        //                 move || {
        //                     cane.$iter_func(..)
        //                         .for_each(|x| drop(black_box(*x)));
        //                 }
        //             ));
        //         }
        //
        //         while let Some(t) = thread_handles.pop() {
        //             t.join().unwrap();
        //         }
        //     });
        // }
    };

    (@(#candy_cane_stream)(#$threads:expr), $chunks:literal, $iter_func:ident, $datalen:literal, $name:ident) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let data = make_data::<$datalen>();

            let cane = Arc::new(RawCandyCane::<RawRwLock, usize, $chunks>::from_vec(data));

            let mut thread_handles = Vec::with_capacity($threads);

            b.iter(move || {
                let cane = black_box(&cane);

                for _ in 0..$threads {
                    let cane = cane.clone();
                    thread_handles.push(std::thread::spawn(
                        move || {
                            let mut iter = cane.$iter_func(..);

                            while let Some(val) = iter.next() {
                                black_box(*val);
                            }
                        }
                    ));
                }

                while let Some(t) = thread_handles.pop() {
                    t.join().unwrap();
                }
            });
        }
    };

    (@(#vec)(no_threads), $iter_func:ident, $datalen:literal, $name:ident) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let mut data = make_data::<$datalen>();

            b.iter(|| {
                let data = black_box(&mut data);
                data.$iter_func()
                    .for_each(|val| drop(black_box(*val)));
            });
        }
    };

    (@(#vec)(#$threads:expr), ($($acquire_func:tt)*), $datalen:literal, $name:ident) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let data = make_data::<$datalen>();

            let data = Arc::new(RwLock::new(data));

            let mut thread_handles = Vec::with_capacity($threads);

            b.iter(move || {
                let data = black_box(&data);

                for _ in 0..$threads {
                    let data = data.clone();
                    thread_handles.push(std::thread::spawn(
                        move || {
                            let guard = data$($acquire_func)*;

                            guard.iter()
                                .for_each(|x| drop(black_box(*x)));
                        }
                    ));
                }

                while let Some(t) = thread_handles.pop() {
                    t.join().unwrap();
                }
            });
        }
    };
}


macro_rules! tests {
    ($([$($args:tt)+]),+$(,)?) => {
        $(
            create_test!($($args)+);
        )+
    }
}

macro_rules! create_test_criterion {
    (@(#candy_cane_iter)(no_threads), $chunks:literal, $iter_func:ident, $datalen:literal, $name:ident) => {
        // #[bench]
        // fn $name(b: &mut Bencher) {
        //     let data = make_data::<$datalen>();
        //
        //     let cane = RawCandyCane::<RawRwLock, usize, $chunks>::from_vec(data);
        //
        //     b.iter(|| {
        //         let cane = black_box(&cane);
        //         cane.$iter_func(..)
        //             .for_each(|val| drop(black_box(*val)));
        //     });
        // }
    };

    (@(#candy_cane_stream)(no_threads), $chunks:literal, $iter_func:ident, $datalen:literal, $name:ident) => {
        fn $name(c: &mut Criterion) {
            let data = make_data::<$datalen>();

            let cane = RawCandyCane::<RawRwLock, usize, $chunks>::from_vec(data);

            c.bench_function(stringify!($name), move |b| b.iter(|| {
                let cane = black_box(&cane);
                let mut iter = cane.$iter_func(..);

                while let Some(val) = iter.next() {
                    black_box(*val);
                }
            }));
        }
    };

    (@(#candy_cane_iter)(#$threads:expr), $chunks:literal, $iter_func:ident, $datalen:literal, $name:ident) => {
        // #[bench]
        // fn $name(b: &mut Bencher) {
        //     let data = make_data::<$datalen>();
        //
        //     let cane = Arc::new(RawCandyCane::<RawRwLock, usize, $chunks>::from_vec(data));
        //
        //     let mut thread_handles = Vec::with_capacity($threads);
        //
        //     b.iter(move || {
        //         let cane = black_box(&cane);
        //
        //         for _ in 0..$threads {
        //             let cane = cane.clone();
        //             thread_handles.push(std::thread::spawn(
        //                 move || {
        //                     cane.$iter_func(..)
        //                         .for_each(|x| drop(black_box(*x)));
        //                 }
        //             ));
        //         }
        //
        //         while let Some(t) = thread_handles.pop() {
        //             t.join().unwrap();
        //         }
        //     });
        // }
    };

    (@(#candy_cane_stream)(#$threads:expr), $chunks:literal, $iter_func:ident, $datalen:literal, $name:ident) => {
        fn $name(c: &mut Criterion) {
            let data = make_data::<$datalen>();

            let cane = Arc::new(RawCandyCane::<RawRwLock, usize, $chunks>::from_vec(data));

            let mut thread_handles = Vec::with_capacity($threads);

            c.bench_function(stringify!($name), |b| b.iter(|| {
                let cane = black_box(&cane);

                for _ in 0..$threads {
                    let cane = cane.clone();
                    thread_handles.push(std::thread::spawn(
                        move || {
                            let mut iter = cane.$iter_func(..);

                            while let Some(val) = iter.next() {
                                black_box(*val);
                            }
                        }
                    ));
                }

                while let Some(t) = thread_handles.pop() {
                    t.join().unwrap();
                }
            }));
        }
    };

    (@(#vec)(no_threads), $iter_func:ident, $datalen:literal, $name:ident) => {
        fn $name(c: &mut Criterion) {
            let mut data = make_data::<$datalen>();

            c.bench_function(stringify!($name), |b| b.iter(|| {
                let data = black_box(&mut data);
                data.$iter_func()
                    .for_each(|val| drop(black_box(*val)));
            }));
        }
    };

    (@(#vec)(#$threads:expr), ($($acquire_func:tt)*), $datalen:literal, $name:ident) => {
        fn $name(c: &mut Criterion) {
            let data = make_data::<$datalen>();

            let data = Arc::new(RwLock::new(data));

            let mut thread_handles = Vec::with_capacity($threads);

            c.bench_function(stringify!($name), move |b| b.iter(|| {
                let data = black_box(&data);

                for _ in 0..$threads {
                    let data = data.clone();
                    thread_handles.push(std::thread::spawn(
                        move || {
                            let guard = data$($acquire_func)*;

                            guard.iter()
                                .for_each(|x| drop(black_box(*x)));
                        }
                    ));
                }

                while let Some(t) = thread_handles.pop() {
                    t.join().unwrap();
                }
            }));
        }
    };
}

macro_rules! tests_criterion {
    ($([$($args:tt)+]: $name:ident),+$(,)?) => {
        $(
            create_test_criterion!($($args)+, $name);
        )+

        criterion_group!(benches, $($name),+);
        criterion_main!(benches);
    }
}
