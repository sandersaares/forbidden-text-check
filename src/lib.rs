use std::sync::LazyLock;

use region_cached::{RegionCachedExt, region_cached};
use region_local::{RegionLocalExt, region_local};

pub static ILLEGAL_NUMBERS: LazyLock<Vec<String>> = LazyLock::new(generate_illegal_numbers);

region_cached! {
    pub static ILLEGAL_NUMBERS_REGION_CACHED: Vec<String> = generate_illegal_numbers();
}

region_local! {
    pub static ILLEGAL_NUMBERS_REGION_LOCAL: Vec<String> = generate_illegal_numbers();
}

fn generate_illegal_numbers() -> Vec<String> {
    const ILLEGAL_NUMBER_START: usize = 5_000_000;
    const ILLEGAL_NUMBER_COUNT: usize = 25_000_000;

    let mut numbers = Vec::with_capacity(ILLEGAL_NUMBER_COUNT);

    // This will be some hundreds of megabytes, which should be enough to not trivially fit in
    // even large L3 caches (though server systems can be rather creative these days).
    //
    // For our purposes, we just want to use a large data set for easy demonstration of
    // large data set effects (which in real world apps might be more "many smaller data sets"
    // that total a large amount of data).
    for i in ILLEGAL_NUMBER_START..(ILLEGAL_NUMBER_START + ILLEGAL_NUMBER_COUNT) {
        numbers.push(i.to_string());
    }

    numbers
}

pub fn contains_illegal_numbers(payload: &str) -> bool {
    ILLEGAL_NUMBERS
        .iter()
        .any(|number| payload.contains(number))
}

pub fn contains_illegal_numbers_region_cached(payload: &str) -> bool {
    ILLEGAL_NUMBERS_REGION_CACHED
        .with_cached(|numbers| numbers.iter().any(|number| payload.contains(number)))
}

pub fn contains_illegal_numbers_region_local(payload: &str) -> bool {
    ILLEGAL_NUMBERS_REGION_LOCAL
        .with_local(|numbers| numbers.iter().any(|number| payload.contains(number)))
}
