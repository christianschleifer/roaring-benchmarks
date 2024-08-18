use roaring_benchmarks::DATASETS;

#[test]
fn it_sets_up_all_datasets() {
    let datasets = &DATASETS;

    assert_eq!(datasets.len(), 8)
}
