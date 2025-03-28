//! We keep the core functionality of generating/accessing the data set and searching it for
//! matches in this module, so it is compiled as a separate unit from the examples/benches.
//! This helps eliminate accidental compiler optimizations that risk randomly skewing the
//! results of our hot loop simply because something unrelated changed in the example .rs file.

use std::sync::LazyLock;

use frozen_collections::{FzHashMap, MapQuery};
use region_cached::{RegionCachedExt, region_cached};
use region_local::{RegionLocalExt, region_local};

pub static FORBIDDEN_TEXTS: LazyLock<FzHashMap<String, bool>> =
    LazyLock::new(generate_forbidden_texts);

region_cached! {
    pub static FORBIDDEN_TEXTS_REGION_CACHED: FzHashMap<String, bool> = generate_forbidden_texts();
}

region_local! {
    pub static FORBIDDEN_TEXTS_REGION_LOCAL: FzHashMap<String, bool> = generate_forbidden_texts();
}

// For the sake of example convenience, our forbidden texts are titled
// with numbers from zero to ITEM_COUNT. This is a few GB of data.
const ITEM_COUNT: usize = 20_000_000;

/// Generates an example data set of forbidden texts, a map of text title to bool (is it forbidden).
fn generate_forbidden_texts() -> FzHashMap<String, bool> {
    let mut raw_data = Vec::with_capacity(ITEM_COUNT);

    for i in 0..ITEM_COUNT {
        let key = i.to_string();
        let value = matches!(i % 2, 0);
        raw_data.push((key, value));
    }

    FzHashMap::new(raw_data)
}

pub fn is_any_forbidden_text_static(titles: &str) -> bool {
    for title in titles.split(',') {
        if *FORBIDDEN_TEXTS.get(title).unwrap_or(&false) {
            return true;
        }
    }

    false
}

pub fn is_any_forbidden_text_region_cached(titles: &str) -> bool {
    FORBIDDEN_TEXTS_REGION_CACHED.with_cached(|texts| {
        for title in titles.split(',') {
            if *texts.get(title).unwrap_or(&false) {
                return true;
            }
        }

        false
    })
}

pub fn is_any_forbidden_text_region_local(titles: &str) -> bool {
    FORBIDDEN_TEXTS_REGION_LOCAL.with_local(|texts| {
        for title in titles.split(',') {
            if *texts.get(title).unwrap_or(&false) {
                return true;
            }
        }

        false
    })
}

fn get_random_title() -> String {
    rand::random_range(0..ITEM_COUNT).to_string()
}

// To make a lot of work for each HTTP request, as each individual lookup is fast and easy.
const BATCH_SIZE: usize = 10_000;

pub fn get_random_titles() -> String {
    (0..BATCH_SIZE)
        .map(|_| get_random_title())
        .collect::<Vec<_>>()
        .join(",")
}
