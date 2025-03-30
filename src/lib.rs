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
    const ITEM_COUNT: usize = 4_000_000;

    let mut texts = Vec::with_capacity(ITEM_COUNT);

    // This will be in the low gigabytes, which should be enough to not trivially fit
    // in even large L3 caches (though server systems can be rather capable these days).
    //
    // For our purposes, we just want to use a large data set for easy demonstration of
    // large data set effects (which in real world apps might be more "many smaller data sets"
    // that total a large amount of data).
    let mut next = u64::MAX;
    let stop = u64::MAX - ITEM_COUNT as u64;

    while next != stop {
        const MULTIPLIER: usize = 32;
        // Concatenate the number to itself many times, so we have "texts" that are realistically
        // long and unique, without having to bother with generating actual random data. We want
        // large data sets that take up a lot of memory (even or especially if that memory is not
        // always accessed, since real world algorithms do not just iterate from 0 to N)
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

pub fn is_forbidden_text_static(s: &str) -> bool {
    FORBIDDEN_TEXTS
        .iter()
        .any(|candidate| s.starts_with(candidate))
}

pub fn is_forbidden_text_region_cached(s: &str) -> bool {
    FORBIDDEN_TEXTS_REGION_CACHED
        .with_cached(|texts| texts.iter().any(|candidate| s.starts_with(candidate)))
}

pub fn is_forbidden_text_region_local(s: &str) -> bool {
    FORBIDDEN_TEXTS_REGION_LOCAL
        .with_local(|texts| texts.iter().any(|candidate| s.starts_with(candidate)))
}

/// Our web service checks different variants of the input string concurrently. This generates
/// concurrent load stress and makes the scenario a bit more realistic because most web services
/// do things like spawning async tasks and awaiting them, which is what we are doing with these.
pub fn into_variants(s: String) -> Vec<String> {
    // The output will contain:
    // - The original string (s)
    // - The original string reversed.
    // - The original string uppercased.
    // - The original string lowercased.
    // - The reversed string uppercased.
    // - The reversed string lowercased.
    // - The original string with the first word removed.
    // - The reversed string with the first word removed.

    let mut variants = Vec::with_capacity(8);

    let first_word_removed: String = s
        .split_whitespace()
        .skip(1)
        .collect::<Vec<&str>>()
        .join(" ");

    variants.push(if first_word_removed.is_empty() {
        s.clone()
    } else {
        first_word_removed
    });

    variants.push(s.to_uppercase());

    variants.push(s.to_lowercase());

    let reversed: String = s.chars().rev().collect();

    variants.push(reversed.to_uppercase());

    variants.push(reversed.to_lowercase());

    let reversed_first_word_removed: String = reversed
        .split_whitespace()
        .skip(1)
        .collect::<Vec<&str>>()
        .join(" ");

    variants.push(if reversed_first_word_removed.is_empty() {
        reversed.clone()
    } else {
        reversed_first_word_removed
    });

    variants.push(reversed);

    variants.push(s);

    variants
}
