use crate::dataset::parse::parse_datasets;
use croaring::Bitmap;
use expected_results::ExpectedResults;
use roaring::RoaringBitmap;
use statistics::Statistics;
use std::collections::BTreeSet;
use std::sync::LazyLock;

pub(crate) mod expected_results;
mod parse;
pub(crate) mod statistics;

const DATASET_FILENAMES: &[&str] = &[
    "census-income.zip",
    "census-income_srt.zip",
    "census1881.zip",
    "census1881_srt.zip",
    "weather_sept_85.zip",
    "weather_sept_85_srt.zip",
    "wikileaks-noquotes.zip",
    "wikileaks-noquotes_srt.zip",
];

pub static DATASETS: LazyLock<Vec<Dataset>> =
    LazyLock::new(|| parse_datasets(DATASET_FILENAMES).expect("could not parse datasets"));

pub struct Dataset {
    name: String,
    raw_data: Vec<BTreeSet<u32>>,
    croaring_bitmaps: Vec<Bitmap>,
    roaring_rs_bitmaps: Vec<RoaringBitmap>,
    statistics: Statistics,
    expected_results: ExpectedResults,
}

impl Dataset {
    pub(crate) fn new(name: String, raw_data: Vec<BTreeSet<u32>>) -> Self {
        let statistics = Statistics::new(&raw_data);
        let expected_results = ExpectedResults::new(&raw_data, &statistics);

        let croaring_bitmaps = raw_data
            .iter()
            .map(|set| Bitmap::from(set.iter().copied().collect::<Vec<_>>().as_slice()))
            .collect();

        let roaring_rs_bitmaps = raw_data.iter().map(RoaringBitmap::from_iter).collect();

        Self {
            name,
            raw_data,
            croaring_bitmaps,
            roaring_rs_bitmaps,
            statistics,
            expected_results,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn raw_data(&self) -> &[BTreeSet<u32>] {
        &self.raw_data
    }

    pub fn croaring_bitmaps(&self) -> &[Bitmap] {
        &self.croaring_bitmaps
    }

    pub fn roaring_rs_bitmaps(&self) -> &[RoaringBitmap] {
        &self.roaring_rs_bitmaps
    }

    pub fn statistics(&self) -> &Statistics {
        &self.statistics
    }

    pub fn expected_results(&self) -> &ExpectedResults {
        &self.expected_results
    }
}
