use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use roaring_benchmarks::DATASETS;

fn contains(c: &mut Criterion) {
    let mut group = c.benchmark_group("contains");
    let datasets = &DATASETS;

    for dataset in datasets.iter() {
        let croaring_bms = dataset.croaring_bitmaps();
        let croaring_bms: Vec<_> = croaring_bms.iter().collect();

        let first_quartile_value = dataset.statistics().first_quartile().value();
        let second_quartile_value = dataset.statistics().second_quartile().value();
        let third_quartile_value = dataset.statistics().third_quartile().value();

        group.bench_with_input(
            BenchmarkId::new("croaring", dataset.name()),
            &(croaring_bms, (first_quartile_value, second_quartile_value, third_quartile_value)),
            |bencher, (bitmaps, (first_quartile_value, second_quartile_value, third_quartile_value))| {
                bencher.iter(|| {
                    for bitmap in bitmaps {
                        let _first = bitmap.contains(*first_quartile_value);
                        let _second = bitmap.contains(*second_quartile_value);
                        let _third = bitmap.contains(*third_quartile_value);
                    }
                })
            },
        );

        let roaring_rs_bms = dataset.roaring_rs_bitmaps();

        group.bench_with_input(
            BenchmarkId::new("roaring-rs", dataset.name()),
            &(roaring_rs_bms, (first_quartile_value, second_quartile_value, third_quartile_value)),
            |bencher, (bitmaps, (first_quartile_value, second_quartile_value, third_quartile_value))| {
                bencher.iter(|| {
                    for bitmap in *bitmaps {
                        let _first = bitmap.contains(*first_quartile_value);
                        let _second = bitmap.contains(*second_quartile_value);
                        let _third = bitmap.contains(*third_quartile_value);
                    }
                })
            },
        );
    }
}

criterion_group!(benches, contains);
criterion_main!(benches);
