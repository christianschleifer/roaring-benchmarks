use roaring_benchmarks::DATASETS;

#[test]
fn it_computes_expected_results() {
    let datasets = &DATASETS;

    for dataset in datasets.iter() {
        let first_quartile = dataset.statistics().first_quartile().value();
        let second_quartile = dataset.statistics().second_quartile().value();
        let third_quartile = dataset.statistics().third_quartile().value();

        let mut first_quartile_results = Vec::with_capacity(dataset.raw_data().len());
        let mut second_quartile_results = Vec::with_capacity(dataset.raw_data().len());
        let mut third_quartile_results = Vec::with_capacity(dataset.raw_data().len());

        let bms = dataset.roaring_rs_bitmaps();

        for bm in bms {
            first_quartile_results.push(bm.contains(first_quartile));
            second_quartile_results.push(bm.contains(second_quartile));
            third_quartile_results.push(bm.contains(third_quartile));
        }

        assert_eq!(
            dataset.expected_results().contains_first_quartile_value(),
            first_quartile_results
        );
        assert_eq!(
            dataset.expected_results().contains_second_quartile_value(),
            second_quartile_results
        );
        assert_eq!(
            dataset.expected_results().contains_third_quartile_value(),
            third_quartile_results
        );
    }
}
