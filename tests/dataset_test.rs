use roaring_benchmarks::DATASETS;

#[test]
fn test() {
    let datasets = &DATASETS;

    assert_eq!(datasets.len(), 8)
}
