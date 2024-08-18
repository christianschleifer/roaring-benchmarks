use std::collections::BTreeSet;
use std::ops::RangeInclusive;

pub struct Statistics {
    universe: RangeInclusive<u32>,
    first_quartile: Percentile,
    second_quartile: Percentile,
    third_quartile: Percentile,
}

impl Statistics {
    pub(crate) fn new(raw_data: &[BTreeSet<u32>]) -> Self {
        let universe = match raw_data.iter().flatten().max() {
            Some(upper_bound) => 0..=*upper_bound,
            None => panic!("expected at least one bitmap"),
        };

        let first_quartile = Percentile::new(&universe, 25);
        let second_quartile = Percentile::new(&universe, 50);
        let third_quartile = Percentile::new(&universe, 75);

        Self {
            universe,
            first_quartile,
            second_quartile,
            third_quartile,
        }
    }

    pub fn universe(&self) -> &RangeInclusive<u32> {
        &self.universe
    }

    pub fn first_quartile(&self) -> &Percentile {
        &self.first_quartile
    }

    pub fn second_quartile(&self) -> &Percentile {
        &self.second_quartile
    }

    pub fn third_quartile(&self) -> &Percentile {
        &self.third_quartile
    }
}

pub struct Percentile {
    percentile: u8,
    value: u32,
}

impl Percentile {
    fn new(universe: &RangeInclusive<u32>, percentile: u8) -> Self {
        Self {
            percentile,
            value: Self::compute_percentile(universe, percentile),
        }
    }

    fn compute_percentile(range: &RangeInclusive<u32>, percentile: u8) -> u32 {
        if percentile > 100 {
            panic!("percentile greater than 100");
        }

        let start = range.start();
        let end = range.end();

        let percentile = (start + ((end - start) * percentile as u32)) as f64 / 100_f64;

        percentile.floor() as u32
    }

    pub fn percentile(&self) -> u8 {
        self.percentile
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}
