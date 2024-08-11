use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::LazyLock;
use std::time::Instant;

use tracing::info;
use zip::ZipArchive;

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

pub static INSTANCE: LazyLock<Vec<Dataset>> =
    LazyLock::new(|| parse_datasets().expect("could not parse datasets"));

pub struct Dataset {
    name: String,
    data: Vec<Vec<u32>>,
}

impl Dataset {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn data(&self) -> &[Vec<u32>] {
        &self.data
    }
}

fn parse_datasets() -> anyhow::Result<Vec<Dataset>> {
    let start = Instant::now();
    let mut datasets = Vec::new();

    for dataset_filename in DATASET_FILENAMES {
        let dataset_file = File::open(PathBuf::from(format!(
            "{}/real-roaring-datasets/{}",
            env!("CARGO_MANIFEST_DIR"),
            dataset_filename
        )))?;

        let mut zip_archive = ZipArchive::new(dataset_file)?;
        let data = process_zip_archive(&mut zip_archive)?;

        datasets.push(Dataset {
            name: dataset_filename.to_string(),
            data,
        });
    }

    let elapsed = start.elapsed().as_millis();

    info!("parsing all datasets took {} milliseconds", elapsed);

    Ok(datasets)
}

fn process_zip_archive(zip: &mut ZipArchive<File>) -> anyhow::Result<Vec<Vec<u32>>> {
    let mut data = Vec::with_capacity(zip.len());

    for i in 0..zip.len() {
        let zip_file = zip.by_index(i)?;
        let buf_reader = BufReader::new(zip_file);

        let mut indices_of_set_bits = Vec::new();
        for result in buf_reader.split(b',') {
            let bytes = result?;

            let index = String::from_utf8(bytes)?;
            let index = index.trim().parse::<u32>()?;
            indices_of_set_bits.push(index);
        }

        data.push(indices_of_set_bits);
    }

    Ok(data)
}
