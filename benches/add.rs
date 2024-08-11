use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use croaring::Bitmap;
use roaring::MultiOps;

use roaring_benchmarks::DATASETS;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Union");

    let datasets = &DATASETS;

    for dataset in datasets.iter() {
        let croaring = dataset.to_croaring_bitmaps();
        let croaring: Vec<&Bitmap> = croaring.iter().collect();
        let roaring = dataset.to_roaring_bitmaps();

        group.bench_with_input(
            BenchmarkId::new("Croaring", dataset.name()),
            &croaring,
            |bencher, data| bencher.iter(|| croaring::Bitmap::fast_or(&data)),
        );

        group.bench_with_input(
            BenchmarkId::new("Roaring", dataset.name()),
            &roaring,
            |bencher, data| bencher.iter(|| data.union()),
        );
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
