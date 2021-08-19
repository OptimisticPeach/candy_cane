#![feature(test)]
extern crate test;

#[macro_use]
mod framework;

use framework::*;

tests_criterion! {
    [@(#candy_cane_stream)(#8), 1, iter_streaming_mut, 500000]: cc_test_1_chunk,
    [@(#candy_cane_stream)(#8), 16, iter_streaming_mut, 500000]: cc_test_16_chunk,
    [@(#candy_cane_stream)(#8), 976, iter_streaming_mut, 500000]: cc_test_per_page,
    [@(#candy_cane_stream)(#8), 1953, iter_streaming_mut, 500000]: cc_test_per_1_2page,
    [@(#candy_cane_stream)(#8), 3906, iter_streaming_mut, 500000]: cc_test_per_1_4page,
    [@(#candy_cane_stream)(#8), 7812, iter_streaming_mut, 500000]: cc_test_per_1_8page,

    [@(#vec)(#8), (.write()), 500000]: vec_naive_test,
}
