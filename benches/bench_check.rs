use std::hint::black_box;

use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use forbidden_text_check::*;
use frozen_collections::Len;
use region_cached::RegionCachedExt;
use region_local::RegionLocalExt;

criterion_group!(benches, entrypoint);
criterion_main!(benches);

fn entrypoint(c: &mut Criterion) {
    let mut g = c.benchmark_group("check");

    // Touch each of the data sets to ensure they are loaded into memory.
    black_box(FORBIDDEN_TEXTS.len());
    black_box(FORBIDDEN_TEXTS_REGION_CACHED.with_cached(|x| x.len()));
    black_box(FORBIDDEN_TEXTS_REGION_LOCAL.with_local(|x| x.len()));

    // The data set is huge, so let's not be greedy.
    g.sample_size(10);

    g.bench_function("static", |b| {
        b.iter_batched_ref(
            get_random_titles,
            |titles| is_any_forbidden_text_static(titles),
            BatchSize::SmallInput,
        );
    });

    g.bench_function("region_cached", |b| {
        b.iter_batched_ref(
            get_random_titles,
            |titles| is_any_forbidden_text_region_cached(titles),
            BatchSize::SmallInput,
        );
    });

    g.bench_function("region_local", |b| {
        b.iter_batched_ref(
            get_random_titles,
            |titles| is_any_forbidden_text_region_local(titles),
            BatchSize::SmallInput,
        );
    });

    g.finish();
}
