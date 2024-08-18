use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use roaring_benchmarks::DATASETS;

fn contains(c: &mut Criterion) {
    let mut group = c.benchmark_group("contains");
    let datasets = &DATASETS;

    for dataset in datasets.iter() {
        let croaring = dataset.croaring_bitmaps();
        let croaring: Vec<_> = croaring.iter().collect();

        group.bench_with_input(
            BenchmarkId::new("croaring", dataset.name()),
            &(croaring, dataset.statistics()),
            |bencher, (bitmaps, statistics)| {
                bencher.iter(|| {
                    for bitmap in bitmaps {
                        let _first = bitmap.contains(statistics.first_quartile().value());
                        let _second = bitmap.contains(statistics.second_quartile().value());
                        let _third = bitmap.contains(statistics.third_quartile().value());
                    }
                })
            },
        );

        let roaring = dataset.roaring_rs_bitmaps();

        group.bench_with_input(
            BenchmarkId::new("roaring-rs", dataset.name()),
            &(roaring, dataset.statistics()),
            |bencher, (bitmaps, statistics)| {
                bencher.iter(|| {
                    for bitmap in *bitmaps {
                        let _first = bitmap.contains(statistics.first_quartile().value());
                        let _second = bitmap.contains(statistics.second_quartile().value());
                        let _third = bitmap.contains(statistics.third_quartile().value());
                    }
                })
            },
        );
    }
}

criterion_group!(benches, contains);
criterion_main!(benches);
