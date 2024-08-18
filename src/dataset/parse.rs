use crate::Dataset;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::time::Instant;
use tracing::info;
use zip::ZipArchive;

pub(crate) fn parse_datasets(files: &[&str]) -> anyhow::Result<Vec<Dataset>> {
    let start = Instant::now();
    let mut datasets = Vec::new();

    for dataset_filename in files {
        let dataset_file = File::open(PathBuf::from(format!(
            "{}/real-roaring-datasets/{}",
            env!("CARGO_MANIFEST_DIR"),
            dataset_filename
        )))?;

        let data = process_zipped_file(&dataset_file)?;

        datasets.push(Dataset::new(dataset_filename.to_string(), data));
    }

    let elapsed = start.elapsed().as_millis();

    info!("parsing all datasets took {} milliseconds", elapsed);

    Ok(datasets)
}

fn process_zipped_file(file: &File) -> anyhow::Result<Vec<BTreeSet<u32>>> {
    let mut zip_archive = ZipArchive::new(file)?;
    let num_of_file_in_archive = zip_archive.len();

    let mut data = Vec::new();

    for i in 0..num_of_file_in_archive {
        let zip_file = zip_archive.by_index(i)?;
        let buf_reader = BufReader::new(zip_file);

        let mut indices_of_set_bits = BTreeSet::new();
        for result in buf_reader.split(b',') {
            let bytes = result?;

            let index = String::from_utf8(bytes)?;
            let index = index.trim().parse::<u32>()?;
            indices_of_set_bits.insert(index);
        }

        data.push(indices_of_set_bits);
    }

    Ok(data)
}
