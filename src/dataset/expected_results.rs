use crate::dataset::statistics::Statistics;
use std::collections::BTreeSet;

pub struct ExpectedResults {
    contains_first_quartile_value: Vec<bool>,
    contains_second_quartile_value: Vec<bool>,
    contains_third_quartile_value: Vec<bool>,
    successive_intersections: Vec<Vec<u32>>,
    successive_unions: Vec<Vec<u32>>,
    collective_union: Vec<u32>,
}

impl ExpectedResults {
    pub(crate) fn new(raw_data: &[BTreeSet<u32>], statistics: &Statistics) -> Self {
        let contains_first_quartile_value: Vec<_> = raw_data
            .iter()
            .map(|bm| bm.contains(&statistics.first_quartile().value()))
            .collect();

        let contains_second_quartile_value: Vec<_> = raw_data
            .iter()
            .map(|bm| bm.contains(&statistics.second_quartile().value()))
            .collect();

        let contains_third_quartile_value: Vec<_> = raw_data
            .iter()
            .map(|bm| bm.contains(&statistics.third_quartile().value()))
            .collect();

        let mut successive_intersections: Vec<Vec<u32>> = Vec::with_capacity(raw_data.len() - 1);
        let mut successive_unions: Vec<Vec<u32>> = Vec::with_capacity(raw_data.len() - 1);

        for i in 1..raw_data.len() {
            let bm1 = raw_data.get(i - 1).expect("data error");
            let bm2 = raw_data.get(i).expect("data error");

            let intersection = bm1.intersection(bm2);
            successive_intersections.push(intersection.copied().collect());

            let union = bm1.union(bm2);
            successive_unions.push(union.copied().collect());
        }

        let collective_union: BTreeSet<u32> = raw_data.iter().flatten().copied().collect();
        let collective_union: Vec<_> = collective_union.into_iter().collect();

        Self {
            contains_first_quartile_value,
            contains_second_quartile_value,
            contains_third_quartile_value,
            successive_intersections,
            successive_unions,
            collective_union,
        }
    }

    pub fn contains_first_quartile_value(&self) -> &[bool] {
        &self.contains_first_quartile_value
    }

    pub fn contains_second_quartile_value(&self) -> &[bool] {
        &self.contains_second_quartile_value
    }

    pub fn contains_third_quartile_value(&self) -> &[bool] {
        &self.contains_third_quartile_value
    }

    pub fn successive_intersections(&self) -> &[Vec<u32>] {
        &self.successive_intersections
    }

    pub fn successive_unions(&self) -> &[Vec<u32>] {
        &self.successive_unions
    }

    pub fn collective_union(&self) -> &[u32] {
        &self.collective_union
    }
}
