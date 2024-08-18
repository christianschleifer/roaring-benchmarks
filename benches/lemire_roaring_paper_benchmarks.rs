//! This harness implements the benchmarks described in
//! [`Consistently faster and smaller compressed bitmaps with Roaring`](https://arxiv.org/pdf/1603.06549).
//!
//! More specifically, the benchmarks implemented here are described in section 6.6 `Performance of
//! Queries in the Java Heap`.

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use croaring::Bitmap;
use roaring::MultiOps;
use roaring_benchmarks::DATASETS;
use std::ops::{BitAnd, BitOr};

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

fn successive_intersections(c: &mut Criterion) {
    let mut group = c.benchmark_group("successive_intersections");
    let datasets = &DATASETS;

    for dataset in datasets.iter() {
        let croaring_bms = dataset.croaring_bitmaps();
        let croaring_bms: Vec<_> = croaring_bms.iter().collect();

        group.bench_with_input(
            BenchmarkId::new("croaring", dataset.name()),
            &croaring_bms,
            |bencher, bitmaps| {
                bencher.iter(|| {
                    for i in 1..bitmaps.len() {
                        let bm1 = bitmaps.get(i - 1).expect("data error");
                        let bm2 = bitmaps.get(i).expect("data error");

                        let _ = bm1.and(bm2);
                    }
                })
            },
        );

        let roaring_rs_bms = dataset.roaring_rs_bitmaps();

        group.bench_with_input(
            BenchmarkId::new("roaring-rs", dataset.name()),
            &roaring_rs_bms,
            |bencher, bitmaps| {
                bencher.iter(|| {
                    for i in 1..bitmaps.len() {
                        let bm1 = bitmaps.get(i - 1).expect("data error");
                        let bm2 = bitmaps.get(i).expect("data error");

                        let _ = bm1.bitand(bm2);
                    }
                })
            },
        );
    }
}

fn successive_unions(c: &mut Criterion) {
    let mut group = c.benchmark_group("successive_unions");
    let datasets = &DATASETS;

    for dataset in datasets.iter() {
        let croaring_bms = dataset.croaring_bitmaps();
        let croaring_bms: Vec<_> = croaring_bms.iter().collect();

        group.bench_with_input(
            BenchmarkId::new("croaring", dataset.name()),
            &croaring_bms,
            |bencher, bitmaps| {
                bencher.iter(|| {
                    for i in 1..bitmaps.len() {
                        let bm1 = bitmaps.get(i - 1).expect("data error");
                        let bm2 = bitmaps.get(i).expect("data error");

                        let _ = bm1.or(bm2);
                    }
                })
            },
        );

        let roaring_rs_bms = dataset.roaring_rs_bitmaps();

        group.bench_with_input(
            BenchmarkId::new("roaring-rs", dataset.name()),
            &roaring_rs_bms,
            |bencher, bitmaps| {
                bencher.iter(|| {
                    for i in 1..bitmaps.len() {
                        let bm1 = bitmaps.get(i - 1).expect("data error");
                        let bm2 = bitmaps.get(i).expect("data error");

                        let _ = bm1.bitor(bm2);
                    }
                })
            },
        );
    }
}

fn collective_union(c: &mut Criterion) {
    let mut group = c.benchmark_group("collective_union");
    let datasets = &DATASETS;

    for dataset in datasets.iter() {
        let croaring_bms = dataset.croaring_bitmaps();
        let croaring_bms: Vec<_> = croaring_bms.iter().collect();

        group.bench_with_input(
            BenchmarkId::new("croaring", dataset.name()),
            &croaring_bms,
            |bencher, bitmaps| {
                bencher.iter(|| {
                    let _ = Bitmap::fast_or(bitmaps);
                })
            },
        );

        let roaring_rs_bms = dataset.roaring_rs_bitmaps();

        group.bench_with_input(
            BenchmarkId::new("roaring-rs", dataset.name()),
            &roaring_rs_bms,
            |bencher, bitmaps| {
                bencher.iter(|| {
                    let _ = bitmaps.union();
                })
            },
        );
    }
}

criterion_group!(
    benches,
    contains,
    successive_intersections,
    successive_unions,
    collective_union
);
criterion_main!(benches);
