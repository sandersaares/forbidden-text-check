//! We keep the core functionality of generating/accessing the data set and searching it for
//! matches in this module, so it is compiled as a separate unit from the examples/benches.
//! This helps eliminate accidental compiler optimizations that risk randomly skewing the
//! results of our hot loop simply because something unrelated changed in the example .rs file.

use std::sync::LazyLock;

use region_cached::{RegionCachedExt, region_cached};
use region_local::{RegionLocalExt, region_local};

pub static FORBIDDEN_TEXTS: LazyLock<Vec<String>> = LazyLock::new(generate_forbidden_texts);

region_cached! {
    pub static FORBIDDEN_TEXTS_REGION_CACHED: Vec<String> = generate_forbidden_texts();
}

region_local! {
    pub static FORBIDDEN_TEXTS_REGION_LOCAL: Vec<String> = generate_forbidden_texts();
}

fn generate_forbidden_texts() -> Vec<String> {
    const ITEM_COUNT: usize = 1_000_000;

    let mut texts = Vec::with_capacity(ITEM_COUNT);

    // This will be in the hundreds of megabytes, which should be enough to not trivially fit in
    // even large L3 caches (though server systems can be rather creative these days).
    //
    // For our purposes, we just want to use a large data set for easy demonstration of
    // large data set effects (which in real world apps might be more "many smaller data sets"
    // that total a large amount of data).
    let mut next = u64::MAX;
    let stop = u64::MAX - ITEM_COUNT as u64;

    while next != stop {
        const MULTIPLIER: usize = 16;
        // Concatenate the number to itself many times, so we have "texts" that are realistically
        // long and unique, without having to bother with generating actual random data.
        let one = next.to_string();

        let mut s = String::with_capacity(one.len() * MULTIPLIER);
        for _ in 0..MULTIPLIER {
            s.push_str(&one);
        }

        texts.push(s);

        next -= 1;
    }

    texts
}

pub fn contains_forbidden_text_static(haystack: &str) -> bool {
    FORBIDDEN_TEXTS
        .iter()
        .any(|needle| haystack.contains(needle))
}

pub fn contains_forbidden_text_region_cached(haystack: &str) -> bool {
    FORBIDDEN_TEXTS_REGION_CACHED
        .with_cached(|needles| needles.iter().any(|needle| haystack.contains(needle)))
}

pub fn contains_forbidden_text_region_local(haystack: &str) -> bool {
    FORBIDDEN_TEXTS_REGION_LOCAL
        .with_local(|needles| needles.iter().any(|needle| haystack.contains(needle)))
}
