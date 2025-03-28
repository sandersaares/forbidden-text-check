//! For easy profiling and comparison of the different ways to do the checking.
//!
//! Ideally the different "check_*" example variants have no differences because
//! all the action is happening on the same thread, so the optimization do not help.

use std::hint::black_box;

use forbidden_text_check::{get_random_titles, is_any_forbidden_text_static};

const ITERATION_COUNT: u64 = 1_000;

fn main() {
    for _ in 0..ITERATION_COUNT {
        // The black box ensures that the operation is not optimized away due to unused result.
        black_box(is_any_forbidden_text_static(&get_random_titles()));
    }
}
