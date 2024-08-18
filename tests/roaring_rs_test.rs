use roaring::MultiOps;
use roaring_benchmarks::DATASETS;
use std::ops::{BitAnd, BitOr};

#[test]
fn it_computes_contains() {
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

#[test]
fn it_computes_successive_intersections() {
    let datasets = &DATASETS;

    for dataset in datasets.iter() {
        let mut results: Vec<Vec<_>> = Vec::with_capacity(dataset.raw_data().len());

        for i in 1..dataset.roaring_rs_bitmaps().len() {
            let bm1 = dataset.roaring_rs_bitmaps().get(i - 1).expect("data error");
            let bm2 = dataset.roaring_rs_bitmaps().get(i).expect("data error");

            results.push(bm1.bitand(bm2).iter().collect());
        }

        assert_eq!(
            dataset.expected_results().successive_intersections(),
            results
        );
    }
}

#[test]
fn it_computes_successive_unions() {
    let datasets = &DATASETS;

    for dataset in datasets.iter() {
        let mut results: Vec<Vec<_>> = Vec::with_capacity(dataset.raw_data().len());

        for i in 1..dataset.roaring_rs_bitmaps().len() {
            let bm1 = dataset.roaring_rs_bitmaps().get(i - 1).expect("data error");
            let bm2 = dataset.roaring_rs_bitmaps().get(i).expect("data error");

            results.push(bm1.bitor(bm2).iter().collect());
        }

        assert_eq!(dataset.expected_results().successive_unions(), results);
    }
}

#[test]
fn it_computes_collective_union() {
    let datasets = &DATASETS;

    for dataset in datasets.iter() {
        let result: Vec<_> = dataset.roaring_rs_bitmaps().union().iter().collect();

        assert_eq!(result, dataset.expected_results().collective_union())
    }
}
