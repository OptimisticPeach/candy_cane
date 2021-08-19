#![feature(test)]
extern crate test;

#[macro_use]
mod framework;

use framework::*;

// tests!(
//     // candy cane, no threads, iter.
//     [@(#candy_cane_iter)(no_threads), 1,  iter, 100,    cc_no_threads_iter_1_100],
//     [@(#candy_cane_iter)(no_threads), 4,  iter, 100,    cc_no_threads_iter_4_100],
//     [@(#candy_cane_iter)(no_threads), 8,  iter, 100,    cc_no_threads_iter_8_100],
//     [@(#candy_cane_iter)(no_threads), 10, iter, 100,    cc_no_threads_iter_10_100],
//
//     [@(#candy_cane_iter)(no_threads), 1,  iter, 1000,   cc_no_threads_iter_1_1000],
//     [@(#candy_cane_iter)(no_threads), 4,  iter, 1000,   cc_no_threads_iter_4_1000],
//     [@(#candy_cane_iter)(no_threads), 8,  iter, 1000,   cc_no_threads_iter_8_1000],
//     [@(#candy_cane_iter)(no_threads), 10, iter, 1000,   cc_no_threads_iter_10_1000],
//
//     [@(#candy_cane_iter)(no_threads), 1,  iter, 5000,   cc_no_threads_iter_1_5000],
//     [@(#candy_cane_iter)(no_threads), 4,  iter, 5000,   cc_no_threads_iter_4_5000],
//     [@(#candy_cane_iter)(no_threads), 8,  iter, 5000,   cc_no_threads_iter_8_5000],
//     [@(#candy_cane_iter)(no_threads), 10, iter, 5000,   cc_no_threads_iter_10_5000],
//
//     [@(#candy_cane_iter)(no_threads), 1,  iter, 10000,  cc_no_threads_iter_1_10000],
//     [@(#candy_cane_iter)(no_threads), 4,  iter, 10000,  cc_no_threads_iter_4_10000],
//     [@(#candy_cane_iter)(no_threads), 8,  iter, 10000,  cc_no_threads_iter_8_10000],
//     [@(#candy_cane_iter)(no_threads), 10, iter, 10000,  cc_no_threads_iter_10_10000],
//
//     [@(#candy_cane_iter)(no_threads), 1,  iter, 50000,  cc_no_threads_iter_1_50000],
//     [@(#candy_cane_iter)(no_threads), 4,  iter, 50000,  cc_no_threads_iter_4_50000],
//     [@(#candy_cane_iter)(no_threads), 8,  iter, 50000,  cc_no_threads_iter_8_50000],
//     [@(#candy_cane_iter)(no_threads), 10, iter, 50000,  cc_no_threads_iter_10_50000],
//
//     // candy cane, no threads, mut iter.
//     [@(#candy_cane_iter)(no_threads), 1,  iter_mut, 100,    cc_no_threads_iter_mut_1_100],
//     [@(#candy_cane_iter)(no_threads), 4,  iter_mut, 100,    cc_no_threads_iter_mut_4_100],
//     [@(#candy_cane_iter)(no_threads), 8,  iter_mut, 100,    cc_no_threads_iter_mut_8_100],
//     [@(#candy_cane_iter)(no_threads), 10, iter_mut, 100,    cc_no_threads_iter_mut_10_100],
//
//     [@(#candy_cane_iter)(no_threads), 1,  iter_mut, 1000,   cc_no_threads_iter_mut_1_1000],
//     [@(#candy_cane_iter)(no_threads), 4,  iter_mut, 1000,   cc_no_threads_iter_mut_4_1000],
//     [@(#candy_cane_iter)(no_threads), 8,  iter_mut, 1000,   cc_no_threads_iter_mut_8_1000],
//     [@(#candy_cane_iter)(no_threads), 10, iter_mut, 1000,   cc_no_threads_iter_mut_10_1000],
//
//     [@(#candy_cane_iter)(no_threads), 1,  iter_mut, 5000,   cc_no_threads_iter_mut_1_5000],
//     [@(#candy_cane_iter)(no_threads), 4,  iter_mut, 5000,   cc_no_threads_iter_mut_4_5000],
//     [@(#candy_cane_iter)(no_threads), 8,  iter_mut, 5000,   cc_no_threads_iter_mut_8_5000],
//     [@(#candy_cane_iter)(no_threads), 10, iter_mut, 5000,   cc_no_threads_iter_mut_10_5000],
//
//     [@(#candy_cane_iter)(no_threads), 1,  iter_mut, 10000,  cc_no_threads_iter_mut_1_10000],
//     [@(#candy_cane_iter)(no_threads), 4,  iter_mut, 10000,  cc_no_threads_iter_mut_4_10000],
//     [@(#candy_cane_iter)(no_threads), 8,  iter_mut, 10000,  cc_no_threads_iter_mut_8_10000],
//     [@(#candy_cane_iter)(no_threads), 10, iter_mut, 10000,  cc_no_threads_iter_mut_10_10000],
//
//     [@(#candy_cane_iter)(no_threads), 1,  iter_mut, 50000,  cc_no_threads_iter_mut_1_50000],
//     [@(#candy_cane_iter)(no_threads), 4,  iter_mut, 50000,  cc_no_threads_iter_mut_4_50000],
//     [@(#candy_cane_iter)(no_threads), 8,  iter_mut, 50000,  cc_no_threads_iter_mut_8_50000],
//     [@(#candy_cane_iter)(no_threads), 10, iter_mut, 50000,  cc_no_threads_iter_mut_10_50000],
//
//     // streaming
//     // candy cane, no threads, streaming_iter.
//     [@(#candy_cane_stream)(no_threads), 1,  iter_streaming, 100,    cc_no_threads_iter_streaming_1_100],
//     [@(#candy_cane_stream)(no_threads), 4,  iter_streaming, 100,    cc_no_threads_iter_streaming_4_100],
//     [@(#candy_cane_stream)(no_threads), 8,  iter_streaming, 100,    cc_no_threads_iter_streaming_8_100],
//     [@(#candy_cane_stream)(no_threads), 10, iter_streaming, 100,    cc_no_threads_iter_streaming_10_100],
//
//     [@(#candy_cane_stream)(no_threads), 1,  iter_streaming, 1000,   cc_no_threads_iter_streaming_1_1000],
//     [@(#candy_cane_stream)(no_threads), 4,  iter_streaming, 1000,   cc_no_threads_iter_streaming_4_1000],
//     [@(#candy_cane_stream)(no_threads), 8,  iter_streaming, 1000,   cc_no_threads_iter_streaming_8_1000],
//     [@(#candy_cane_stream)(no_threads), 10, iter_streaming, 1000,   cc_no_threads_iter_streaming_10_1000],
//
//     [@(#candy_cane_stream)(no_threads), 1,  iter_streaming, 5000,   cc_no_threads_iter_streaming_1_5000],
//     [@(#candy_cane_stream)(no_threads), 4,  iter_streaming, 5000,   cc_no_threads_iter_streaming_4_5000],
//     [@(#candy_cane_stream)(no_threads), 8,  iter_streaming, 5000,   cc_no_threads_iter_streaming_8_5000],
//     [@(#candy_cane_stream)(no_threads), 10, iter_streaming, 5000,   cc_no_threads_iter_streaming_10_5000],
//
//     [@(#candy_cane_stream)(no_threads), 1,  iter_streaming, 10000,  cc_no_threads_iter_streaming_1_10000],
//     [@(#candy_cane_stream)(no_threads), 4,  iter_streaming, 10000,  cc_no_threads_iter_streaming_4_10000],
//     [@(#candy_cane_stream)(no_threads), 8,  iter_streaming, 10000,  cc_no_threads_iter_streaming_8_10000],
//     [@(#candy_cane_stream)(no_threads), 10, iter_streaming, 10000,  cc_no_threads_iter_streaming_10_10000],
//
//     [@(#candy_cane_stream)(no_threads), 1,  iter_streaming, 50000,  cc_no_threads_iter_streaming_1_50000],
//     [@(#candy_cane_stream)(no_threads), 4,  iter_streaming, 50000,  cc_no_threads_iter_streaming_4_50000],
//     [@(#candy_cane_stream)(no_threads), 8,  iter_streaming, 50000,  cc_no_threads_iter_streaming_8_50000],
//     [@(#candy_cane_stream)(no_threads), 10, iter_streaming, 50000,  cc_no_threads_iter_streaming_10_50000],
//
//     // candy cane, no threads, mut iter.
//     [@(#candy_cane_stream)(no_threads), 1,  iter_streaming_mut, 100,    cc_no_threads_iter_streaming_mut_1_100],
//     [@(#candy_cane_stream)(no_threads), 4,  iter_streaming_mut, 100,    cc_no_threads_iter_streaming_mut_4_100],
//     [@(#candy_cane_stream)(no_threads), 8,  iter_streaming_mut, 100,    cc_no_threads_iter_streaming_mut_8_100],
//     [@(#candy_cane_stream)(no_threads), 10, iter_streaming_mut, 100,    cc_no_threads_iter_streaming_mut_10_100],
//
//     [@(#candy_cane_stream)(no_threads), 1,  iter_streaming_mut, 1000,   cc_no_threads_iter_streaming_mut_1_1000],
//     [@(#candy_cane_stream)(no_threads), 4,  iter_streaming_mut, 1000,   cc_no_threads_iter_streaming_mut_4_1000],
//     [@(#candy_cane_stream)(no_threads), 8,  iter_streaming_mut, 1000,   cc_no_threads_iter_streaming_mut_8_1000],
//     [@(#candy_cane_stream)(no_threads), 10, iter_streaming_mut, 1000,   cc_no_threads_iter_streaming_mut_10_1000],
//
//     [@(#candy_cane_stream)(no_threads), 1,  iter_streaming_mut, 5000,   cc_no_threads_iter_streaming_mut_1_5000],
//     [@(#candy_cane_stream)(no_threads), 4,  iter_streaming_mut, 5000,   cc_no_threads_iter_streaming_mut_4_5000],
//     [@(#candy_cane_stream)(no_threads), 8,  iter_streaming_mut, 5000,   cc_no_threads_iter_streaming_mut_8_5000],
//     [@(#candy_cane_stream)(no_threads), 10, iter_streaming_mut, 5000,   cc_no_threads_iter_streaming_mut_10_5000],
//
//     [@(#candy_cane_stream)(no_threads), 1,  iter_streaming_mut, 10000,  cc_no_threads_iter_streaming_mut_1_10000],
//     [@(#candy_cane_stream)(no_threads), 4,  iter_streaming_mut, 10000,  cc_no_threads_iter_streaming_mut_4_10000],
//     [@(#candy_cane_stream)(no_threads), 8,  iter_streaming_mut, 10000,  cc_no_threads_iter_streaming_mut_8_10000],
//     [@(#candy_cane_stream)(no_threads), 10, iter_streaming_mut, 10000,  cc_no_threads_iter_streaming_mut_10_10000],
//
//     [@(#candy_cane_stream)(no_threads), 1,  iter_streaming_mut, 50000,  cc_no_threads_iter_streaming_mut_1_50000],
//     [@(#candy_cane_stream)(no_threads), 4,  iter_streaming_mut, 50000,  cc_no_threads_iter_streaming_mut_4_50000],
//     [@(#candy_cane_stream)(no_threads), 8,  iter_streaming_mut, 50000,  cc_no_threads_iter_streaming_mut_8_50000],
//     [@(#candy_cane_stream)(no_threads), 10, iter_streaming_mut, 50000,  cc_no_threads_iter_streaming_mut_10_50000],
//
//     //=====================================================
//     // 2 threads
//     //=====================================================
//
//     // candy cane, no threads, iter.
//     [@(#candy_cane_iter)(#2), 1,  iter, 100,    cc_2_threads_iter_1_100],
//     [@(#candy_cane_iter)(#2), 4,  iter, 100,    cc_2_threads_iter_4_100],
//     [@(#candy_cane_iter)(#2), 8,  iter, 100,    cc_2_threads_iter_8_100],
//     [@(#candy_cane_iter)(#2), 10, iter, 100,    cc_2_threads_iter_10_100],
//
//     [@(#candy_cane_iter)(#2), 1,  iter, 1000,   cc_2_threads_iter_1_1000],
//     [@(#candy_cane_iter)(#2), 4,  iter, 1000,   cc_2_threads_iter_4_1000],
//     [@(#candy_cane_iter)(#2), 8,  iter, 1000,   cc_2_threads_iter_8_1000],
//     [@(#candy_cane_iter)(#2), 10, iter, 1000,   cc_2_threads_iter_10_1000],
//
//     [@(#candy_cane_iter)(#2), 1,  iter, 5000,   cc_2_threads_iter_1_5000],
//     [@(#candy_cane_iter)(#2), 4,  iter, 5000,   cc_2_threads_iter_4_5000],
//     [@(#candy_cane_iter)(#2), 8,  iter, 5000,   cc_2_threads_iter_8_5000],
//     [@(#candy_cane_iter)(#2), 10, iter, 5000,   cc_2_threads_iter_10_5000],
//
//     [@(#candy_cane_iter)(#2), 1,  iter, 10000,  cc_2_threads_iter_1_10000],
//     [@(#candy_cane_iter)(#2), 4,  iter, 10000,  cc_2_threads_iter_4_10000],
//     [@(#candy_cane_iter)(#2), 8,  iter, 10000,  cc_2_threads_iter_8_10000],
//     [@(#candy_cane_iter)(#2), 10, iter, 10000,  cc_2_threads_iter_10_10000],
//
//     [@(#candy_cane_iter)(#2), 1,  iter, 50000,  cc_2_threads_iter_1_50000],
//     [@(#candy_cane_iter)(#2), 4,  iter, 50000,  cc_2_threads_iter_4_50000],
//     [@(#candy_cane_iter)(#2), 8,  iter, 50000,  cc_2_threads_iter_8_50000],
//     [@(#candy_cane_iter)(#2), 10, iter, 50000,  cc_2_threads_iter_10_50000],
//
//     // candy cane, no 2, mut iter.
//     [@(#candy_cane_iter)(#2), 1,  iter_mut, 100,    cc_2_threads_iter_mut_1_100],
//     [@(#candy_cane_iter)(#2), 4,  iter_mut, 100,    cc_2_threads_iter_mut_4_100],
//     [@(#candy_cane_iter)(#2), 8,  iter_mut, 100,    cc_2_threads_iter_mut_8_100],
//     [@(#candy_cane_iter)(#2), 10, iter_mut, 100,    cc_2_threads_iter_mut_10_100],
//
//     [@(#candy_cane_iter)(#2), 1,  iter_mut, 1000,   cc_2_threads_iter_mut_1_1000],
//     [@(#candy_cane_iter)(#2), 4,  iter_mut, 1000,   cc_2_threads_iter_mut_4_1000],
//     [@(#candy_cane_iter)(#2), 8,  iter_mut, 1000,   cc_2_threads_iter_mut_8_1000],
//     [@(#candy_cane_iter)(#2), 10, iter_mut, 1000,   cc_2_threads_iter_mut_10_1000],
//     //
//     [@(#candy_cane_iter)(#2), 1,  iter_mut, 5000,   cc_2_threads_iter_mut_1_5000],
//     [@(#candy_cane_iter)(#2), 4,  iter_mut, 5000,   cc_2_threads_iter_mut_4_5000],
//     [@(#candy_cane_iter)(#2), 8,  iter_mut, 5000,   cc_2_threads_iter_mut_8_5000],
//     [@(#candy_cane_iter)(#2), 10, iter_mut, 5000,   cc_2_threads_iter_mut_10_5000],
//
//     [@(#candy_cane_iter)(#2), 1,  iter_mut, 10000,  cc_2_threads_iter_mut_1_10000],
//     [@(#candy_cane_iter)(#2), 4,  iter_mut, 10000,  cc_2_threads_iter_mut_4_10000],
//     [@(#candy_cane_iter)(#2), 8,  iter_mut, 10000,  cc_2_threads_iter_mut_8_10000],
//     [@(#candy_cane_iter)(#2), 10, iter_mut, 10000,  cc_2_threads_iter_mut_10_10000],
//
//     [@(#candy_cane_iter)(#2), 1,  iter_mut, 50000,  cc_2_threads_iter_mut_1_50000],
//     [@(#candy_cane_iter)(#2), 4,  iter_mut, 50000,  cc_2_threads_iter_mut_4_50000],
//     [@(#candy_cane_iter)(#2), 8,  iter_mut, 50000,  cc_2_threads_iter_mut_8_50000],
//     [@(#candy_cane_iter)(#2), 10, iter_mut, 50000,  cc_2_threads_iter_mut_10_50000],
//
//     // streaming
//     // candy 2, no 2, streaming_iter.
//     [@(#candy_cane_stream)(#2), 1,  iter_streaming, 100,    cc_2_threads_iter_streaming_1_100],
//     [@(#candy_cane_stream)(#2), 4,  iter_streaming, 100,    cc_2_threads_iter_streaming_4_100],
//     [@(#candy_cane_stream)(#2), 8,  iter_streaming, 100,    cc_2_threads_iter_streaming_8_100],
//     [@(#candy_cane_stream)(#2), 10, iter_streaming, 100,    cc_2_threads_iter_streaming_10_100],
//
//     [@(#candy_cane_stream)(#2), 1,  iter_streaming, 1000,   cc_2_threads_iter_streaming_1_1000],
//     [@(#candy_cane_stream)(#2), 4,  iter_streaming, 1000,   cc_2_threads_iter_streaming_4_1000],
//     [@(#candy_cane_stream)(#2), 8,  iter_streaming, 1000,   cc_2_threads_iter_streaming_8_1000],
//     [@(#candy_cane_stream)(#2), 10, iter_streaming, 1000,   cc_2_threads_iter_streaming_10_1000],
//
//     [@(#candy_cane_stream)(#2), 1,  iter_streaming, 5000,   cc_2_threads_iter_streaming_1_5000],
//     [@(#candy_cane_stream)(#2), 4,  iter_streaming, 5000,   cc_2_threads_iter_streaming_4_5000],
//     [@(#candy_cane_stream)(#2), 8,  iter_streaming, 5000,   cc_2_threads_iter_streaming_8_5000],
//     [@(#candy_cane_stream)(#2), 10, iter_streaming, 5000,   cc_2_threads_iter_streaming_10_5000],
//
//     [@(#candy_cane_stream)(#2), 1,  iter_streaming, 10000,  cc_2_threads_iter_streaming_1_10000],
//     [@(#candy_cane_stream)(#2), 4,  iter_streaming, 10000,  cc_2_threads_iter_streaming_4_10000],
//     [@(#candy_cane_stream)(#2), 8,  iter_streaming, 10000,  cc_2_threads_iter_streaming_8_10000],
//     [@(#candy_cane_stream)(#2), 10, iter_streaming, 10000,  cc_2_threads_iter_streaming_10_10000],
//
//     [@(#candy_cane_stream)(#2), 1,  iter_streaming, 50000,  cc_2_threads_iter_streaming_1_50000],
//     [@(#candy_cane_stream)(#2), 4,  iter_streaming, 50000,  cc_2_threads_iter_streaming_4_50000],
//     [@(#candy_cane_stream)(#2), 8,  iter_streaming, 50000,  cc_2_threads_iter_streaming_8_50000],
//     [@(#candy_cane_stream)(#2), 10, iter_streaming, 50000,  cc_2_threads_iter_streaming_10_50000],
//
//     // candy cane, no 2, mut iter.
//     [@(#candy_cane_stream)(#2), 1,  iter_streaming_mut, 100,    cc_2_threads_iter_streaming_mut_1_100],
//     [@(#candy_cane_stream)(#2), 4,  iter_streaming_mut, 100,    cc_2_threads_iter_streaming_mut_4_100],
//     [@(#candy_cane_stream)(#2), 8,  iter_streaming_mut, 100,    cc_2_threads_iter_streaming_mut_8_100],
//     [@(#candy_cane_stream)(#2), 10, iter_streaming_mut, 100,    cc_2_threads_iter_streaming_mut_10_100],
//
//     [@(#candy_cane_stream)(#2), 1,  iter_streaming_mut, 1000,   cc_2_threads_iter_streaming_mut_1_1000],
//     [@(#candy_cane_stream)(#2), 4,  iter_streaming_mut, 1000,   cc_2_threads_iter_streaming_mut_4_1000],
//     [@(#candy_cane_stream)(#2), 8,  iter_streaming_mut, 1000,   cc_2_threads_iter_streaming_mut_8_1000],
//     [@(#candy_cane_stream)(#2), 10, iter_streaming_mut, 1000,   cc_2_threads_iter_streaming_mut_10_1000],
//
//     [@(#candy_cane_stream)(#2), 1,  iter_streaming_mut, 5000,   cc_2_threads_iter_streaming_mut_1_5000],
//     [@(#candy_cane_stream)(#2), 4,  iter_streaming_mut, 5000,   cc_2_threads_iter_streaming_mut_4_5000],
//     [@(#candy_cane_stream)(#2), 8,  iter_streaming_mut, 5000,   cc_2_threads_iter_streaming_mut_8_5000],
//     [@(#candy_cane_stream)(#2), 10, iter_streaming_mut, 5000,   cc_2_threads_iter_streaming_mut_10_5000],
//
//     [@(#candy_cane_stream)(#2), 1,  iter_streaming_mut, 10000,  cc_2_threads_iter_streaming_mut_1_10000],
//     [@(#candy_cane_stream)(#2), 4,  iter_streaming_mut, 10000,  cc_2_threads_iter_streaming_mut_4_10000],
//     [@(#candy_cane_stream)(#2), 8,  iter_streaming_mut, 10000,  cc_2_threads_iter_streaming_mut_8_10000],
//     [@(#candy_cane_stream)(#2), 10, iter_streaming_mut, 10000,  cc_2_threads_iter_streaming_mut_10_10000],
//
//     [@(#candy_cane_stream)(#2), 1,  iter_streaming_mut, 50000,  cc_2_threads_iter_streaming_mut_1_50000],
//     [@(#candy_cane_stream)(#2), 4,  iter_streaming_mut, 50000,  cc_2_threads_iter_streaming_mut_4_50000],
//     [@(#candy_cane_stream)(#2), 8,  iter_streaming_mut, 50000,  cc_2_threads_iter_streaming_mut_8_50000],
//     [@(#candy_cane_stream)(#2), 10, iter_streaming_mut, 50000,  cc_2_threads_iter_streaming_mut_10_50000],
//
//     //===================================================
//     // 6 threads
//     //===================================================
//
//     // candy cane, no threads, iter.
//     [@(#candy_cane_iter)(#6), 1,  iter, 100,    cc_6_threads_iter_1_100],
//     [@(#candy_cane_iter)(#6), 4,  iter, 100,    cc_6_threads_iter_4_100],
//     [@(#candy_cane_iter)(#6), 8,  iter, 100,    cc_6_threads_iter_8_100],
//     [@(#candy_cane_iter)(#6), 10, iter, 100,    cc_6_threads_iter_10_100],
//
//     [@(#candy_cane_iter)(#6), 1,  iter, 1000,   cc_6_threads_iter_1_1000],
//     [@(#candy_cane_iter)(#6), 4,  iter, 1000,   cc_6_threads_iter_4_1000],
//     [@(#candy_cane_iter)(#6), 8,  iter, 1000,   cc_6_threads_iter_8_1000],
//     [@(#candy_cane_iter)(#6), 10, iter, 1000,   cc_6_threads_iter_10_1000],
//
//     [@(#candy_cane_iter)(#6), 1,  iter, 5000,   cc_6_threads_iter_1_5000],
//     [@(#candy_cane_iter)(#6), 4,  iter, 5000,   cc_6_threads_iter_4_5000],
//     [@(#candy_cane_iter)(#6), 8,  iter, 5000,   cc_6_threads_iter_8_5000],
//     [@(#candy_cane_iter)(#6), 10, iter, 5000,   cc_6_threads_iter_10_5000],
//
//     [@(#candy_cane_iter)(#6), 1,  iter, 10000,  cc_6_threads_iter_1_10000],
//     [@(#candy_cane_iter)(#6), 4,  iter, 10000,  cc_6_threads_iter_4_10000],
//     [@(#candy_cane_iter)(#6), 8,  iter, 10000,  cc_6_threads_iter_8_10000],
//     [@(#candy_cane_iter)(#6), 10, iter, 10000,  cc_6_threads_iter_10_10000],
//
//     [@(#candy_cane_iter)(#6), 1,  iter, 50000,  cc_6_threads_iter_1_50000],
//     [@(#candy_cane_iter)(#6), 4,  iter, 50000,  cc_6_threads_iter_4_50000],
//     [@(#candy_cane_iter)(#6), 8,  iter, 50000,  cc_6_threads_iter_8_50000],
//     [@(#candy_cane_iter)(#6), 10, iter, 50000,  cc_6_threads_iter_10_50000],
//
//     // candy cane, no threads,6mut iter.
//     [@(#candy_cane_iter)(#6), 1,  iter_mut, 100,    cc_6_threads_iter_mut_1_100],
//     [@(#candy_cane_iter)(#6), 4,  iter_mut, 100,    cc_6_threads_iter_mut_4_100],
//     [@(#candy_cane_iter)(#6), 8,  iter_mut, 100,    cc_6_threads_iter_mut_8_100],
//     [@(#candy_cane_iter)(#6), 10, iter_mut, 100,    cc_6_threads_iter_mut_10_100],
//
//     [@(#candy_cane_iter)(#6), 1,  iter_mut, 1000,   cc_6_threads_iter_mut_1_1000],
//     [@(#candy_cane_iter)(#6), 4,  iter_mut, 1000,   cc_6_threads_iter_mut_4_1000],
//     [@(#candy_cane_iter)(#6), 8,  iter_mut, 1000,   cc_6_threads_iter_mut_8_1000],
//     [@(#candy_cane_iter)(#6), 10, iter_mut, 1000,   cc_6_threads_iter_mut_10_1000],
//
//     [@(#candy_cane_iter)(#6), 1,  iter_mut, 5000,   cc_6_threads_iter_mut_1_5000],
//     [@(#candy_cane_iter)(#6), 4,  iter_mut, 5000,   cc_6_threads_iter_mut_4_5000],
//     [@(#candy_cane_iter)(#6), 8,  iter_mut, 5000,   cc_6_threads_iter_mut_8_5000],
//     [@(#candy_cane_iter)(#6), 10, iter_mut, 5000,   cc_6_threads_iter_mut_10_5000],
//
//     [@(#candy_cane_iter)(#6), 1,  iter_mut, 10000,  cc_6_threads_iter_mut_1_10000],
//     [@(#candy_cane_iter)(#6), 4,  iter_mut, 10000,  cc_6_threads_iter_mut_4_10000],
//     [@(#candy_cane_iter)(#6), 8,  iter_mut, 10000,  cc_6_threads_iter_mut_8_10000],
//     [@(#candy_cane_iter)(#6), 10, iter_mut, 10000,  cc_6_threads_iter_mut_10_10000],
//
//     [@(#candy_cane_iter)(#6), 1,  iter_mut, 50000,  cc_6_threads_iter_mut_1_50000],
//     [@(#candy_cane_iter)(#6), 4,  iter_mut, 50000,  cc_6_threads_iter_mut_4_50000],
//     [@(#candy_cane_iter)(#6), 8,  iter_mut, 50000,  cc_6_threads_iter_mut_8_50000],
//     [@(#candy_cane_iter)(#6), 10, iter_mut, 50000,  cc_6_threads_iter_mut_10_50000],
//
//     // streaming
//     // candy cane,6no threads,6streaming_iter.
//     [@(#candy_cane_stream)(#6), 1,  iter_streaming, 100,    cc_6_threads_iter_streaming_1_100],
//     [@(#candy_cane_stream)(#6), 4,  iter_streaming, 100,    cc_6_threads_iter_streaming_4_100],
//     [@(#candy_cane_stream)(#6), 8,  iter_streaming, 100,    cc_6_threads_iter_streaming_8_100],
//     [@(#candy_cane_stream)(#6), 10, iter_streaming, 100,    cc_6_threads_iter_streaming_10_100],
//
//     [@(#candy_cane_stream)(#6), 1,  iter_streaming, 1000,   cc_6_threads_iter_streaming_1_1000],
//     [@(#candy_cane_stream)(#6), 4,  iter_streaming, 1000,   cc_6_threads_iter_streaming_4_1000],
//     [@(#candy_cane_stream)(#6), 8,  iter_streaming, 1000,   cc_6_threads_iter_streaming_8_1000],
//     [@(#candy_cane_stream)(#6), 10, iter_streaming, 1000,   cc_6_threads_iter_streaming_10_1000],
//
//     [@(#candy_cane_stream)(#6), 1,  iter_streaming, 5000,   cc_6_threads_iter_streaming_1_5000],
//     [@(#candy_cane_stream)(#6), 4,  iter_streaming, 5000,   cc_6_threads_iter_streaming_4_5000],
//     [@(#candy_cane_stream)(#6), 8,  iter_streaming, 5000,   cc_6_threads_iter_streaming_8_5000],
//     [@(#candy_cane_stream)(#6), 10, iter_streaming, 5000,   cc_6_threads_iter_streaming_10_5000],
//
//     [@(#candy_cane_stream)(#6), 1,  iter_streaming, 10000,  cc_6_threads_iter_streaming_1_10000],
//     [@(#candy_cane_stream)(#6), 4,  iter_streaming, 10000,  cc_6_threads_iter_streaming_4_10000],
//     [@(#candy_cane_stream)(#6), 8,  iter_streaming, 10000,  cc_6_threads_iter_streaming_8_10000],
//     [@(#candy_cane_stream)(#6), 10, iter_streaming, 10000,  cc_6_threads_iter_streaming_10_10000],
//
//     [@(#candy_cane_stream)(#6), 1,  iter_streaming, 50000,  cc_6_threads_iter_streaming_1_50000],
//     [@(#candy_cane_stream)(#6), 4,  iter_streaming, 50000,  cc_6_threads_iter_streaming_4_50000],
//     [@(#candy_cane_stream)(#6), 8,  iter_streaming, 50000,  cc_6_threads_iter_streaming_8_50000],
//     [@(#candy_cane_stream)(#6), 10, iter_streaming, 50000,  cc_6_threads_iter_streaming_10_50000],
//
//     // candy cane, no threads,6mut iter.
//     [@(#candy_cane_stream)(#6), 1,  iter_streaming_mut, 100,    cc_6_threads_iter_streaming_mut_1_100],
//     [@(#candy_cane_stream)(#6), 4,  iter_streaming_mut, 100,    cc_6_threads_iter_streaming_mut_4_100],
//     [@(#candy_cane_stream)(#6), 8,  iter_streaming_mut, 100,    cc_6_threads_iter_streaming_mut_8_100],
//     [@(#candy_cane_stream)(#6), 10, iter_streaming_mut, 100,    cc_6_threads_iter_streaming_mut_10_100],
//
//     [@(#candy_cane_stream)(#6), 1,  iter_streaming_mut, 1000,   cc_6_threads_iter_streaming_mut_1_1000],
//     [@(#candy_cane_stream)(#6), 4,  iter_streaming_mut, 1000,   cc_6_threads_iter_streaming_mut_4_1000],
//     [@(#candy_cane_stream)(#6), 8,  iter_streaming_mut, 1000,   cc_6_threads_iter_streaming_mut_8_1000],
//     [@(#candy_cane_stream)(#6), 10, iter_streaming_mut, 1000,   cc_6_threads_iter_streaming_mut_10_1000],
//
//     [@(#candy_cane_stream)(#6), 1,  iter_streaming_mut, 5000,   cc_6_threads_iter_streaming_mut_1_5000],
//     [@(#candy_cane_stream)(#6), 4,  iter_streaming_mut, 5000,   cc_6_threads_iter_streaming_mut_4_5000],
//     [@(#candy_cane_stream)(#6), 8,  iter_streaming_mut, 5000,   cc_6_threads_iter_streaming_mut_8_5000],
//     [@(#candy_cane_stream)(#6), 10, iter_streaming_mut, 5000,   cc_6_threads_iter_streaming_mut_10_5000],
//
//     [@(#candy_cane_stream)(#6), 1,  iter_streaming_mut, 10000,  cc_6_threads_iter_streaming_mut_1_10000],
//     [@(#candy_cane_stream)(#6), 4,  iter_streaming_mut, 10000,  cc_6_threads_iter_streaming_mut_4_10000],
//     [@(#candy_cane_stream)(#6), 8,  iter_streaming_mut, 10000,  cc_6_threads_iter_streaming_mut_8_10000],
//     [@(#candy_cane_stream)(#6), 10, iter_streaming_mut, 10000,  cc_6_threads_iter_streaming_mut_10_10000],
//
//     [@(#candy_cane_stream)(#6), 1,  iter_streaming_mut, 50000,  cc_6_threads_iter_streaming_mut_1_50000],
//     [@(#candy_cane_stream)(#6), 4,  iter_streaming_mut, 50000,  cc_6_threads_iter_streaming_mut_4_50000],
//     [@(#candy_cane_stream)(#6), 8,  iter_streaming_mut, 50000,  cc_6_threads_iter_streaming_mut_8_50000],
//     [@(#candy_cane_stream)(#6), 10, iter_streaming_mut, 50000,  cc_6_threads_iter_streaming_mut_10_50000],
//
//     // VECS
//     ////////////////////////////////////////////////////////
//     // candy cane, no threads, iter.
//     [@(#vec)(no_threads), iter, 100,    vec_no_threads_iter_100],
//     [@(#vec)(no_threads), iter, 1000,   vec_no_threads_iter_1000],
//     [@(#vec)(no_threads), iter, 5000,   vec_no_threads_iter_5000],
//     [@(#vec)(no_threads), iter, 10000,  vec_no_threads_iter_10000],
//     [@(#vec)(no_threads), iter, 50000,  vec_no_threads_iter_50000],
//
//     [@(#vec)(no_threads), iter_mut, 100,    vec_no_threads_iter_mut_100],
//     [@(#vec)(no_threads), iter_mut, 1000,   vec_no_threads_iter_mut_1000],
//     [@(#vec)(no_threads), iter_mut, 5000,   vec_no_threads_iter_mut_5000],
//     [@(#vec)(no_threads), iter_mut, 10000,  vec_no_threads_iter_mut_10000],
//     [@(#vec)(no_threads), iter_mut, 50000,  vec_no_threads_iter_mut_50000],
//
//     [@(#vec)(#2), (.read()), 100,    vec_2_threads_iter_100],
//     [@(#vec)(#2), (.read()), 1000,   vec_2_threads_iter_1000],
//     [@(#vec)(#2), (.read()), 5000,   vec_2_threads_iter_5000],
//     [@(#vec)(#2), (.read()), 10000,  vec_2_threads_iter_10000],
//     [@(#vec)(#2), (.read()), 50000,  vec_2_threads_iter_50000],
//
//     [@(#vec)(#2), (.write()), 100,    vec_2_threads_iter_mut_100],
//     [@(#vec)(#2), (.write()), 1000,   vec_2_threads_iter_mut_1000],//
//     [@(#vec)(#2), (.write()), 5000,   vec_2_threads_iter_mut_5000],
//     [@(#vec)(#2), (.write()), 10000,  vec_2_threads_iter_mut_10000],
//     [@(#vec)(#2), (.write()), 50000,  vec_2_threads_iter_mut_50000],
//
//     [@(#vec)(#6), (.read()), 100,    vec_6_threads_iter_100],
//     [@(#vec)(#6), (.read()), 1000,   vec_6_threads_iter_1000],
//     [@(#vec)(#6), (.read()), 5000,   vec_6_threads_iter_5000],
//     [@(#vec)(#6), (.read()), 10000,  vec_6_threads_iter_10000],
//     [@(#vec)(#6), (.read()), 50000,  vec_6_threads_iter_50000],
//
//     [@(#vec)(#6), (.write()), 100,    vec_6_threads_iter_mut_100],
//     [@(#vec)(#6), (.write()), 1000,   vec_6_threads_iter_mut_1000],
//     [@(#vec)(#6), (.write()), 5000,   vec_6_threads_iter_mut_5000],
//     [@(#vec)(#6), (.write()), 10000,  vec_6_threads_iter_mut_10000],
//     [@(#vec)(#6), (.write()), 50000,  vec_6_threads_iter_mut_50000],
// );
