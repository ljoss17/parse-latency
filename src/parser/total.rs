use indicatif::ProgressBar;
use itertools::Itertools;
use serde_derive::Serialize;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

use crate::error::Error;
use crate::utils::*;

#[derive(Debug, Serialize)]
struct TimerStatistics {
    name: String,
    count: u128,
    mean: f64,
    min: u128,
    max: u128,
    q10: u128,
    q25: u128,
    median: u128,
    q75: u128,
    q90: u128,
    total: f64,
}

pub fn parse_total_infos(
    elapsed_values: &HashMap<String, Vec<u128>>,
    name_prefix: &String,
) -> Result<(), Error> {
    // Initialise progress bar
    let pb = ProgressBar::new(elapsed_values.keys().len() as u64);
    pb.set_style(create_progress_bar_template());
    pb.set_message("Parse total infos");

    let mut statistics = vec![];
    let mut filtered_statistics = vec![];

    for name in elapsed_values.keys().sorted() {
        let mut values = elapsed_values[name].clone();
        values.sort();

        let mean = compute_mean(&values);
        let std_dev = std_deviation(&values, mean);
        let parsed_values: Vec<u128> = values
            .iter()
            .filter(|v| z_score(**v, mean, std_dev, 3.0))
            .cloned()
            .collect();

        let statistic = TimerStatistics {
            name: name.clone(),
            count: values.len() as u128,
            mean,
            min: compute_min(&values),
            max: compute_max(&values),
            q10: compute_percentile(&values, 0.1),
            q25: compute_percentile(&values, 0.25),
            q75: compute_percentile(&values, 0.75),
            q90: compute_percentile(&values, 0.9),
            median: compute_percentile(&values, 0.5),
            total: compute_total_minutes(&values),
        };
        statistics.push(statistic);

        let filtered_statistic = TimerStatistics {
            name: name.clone(),
            count: parsed_values.len() as u128,
            mean: compute_mean(&parsed_values),
            min: compute_min(&parsed_values),
            max: compute_max(&parsed_values),
            q10: compute_percentile(&parsed_values, 0.1),
            q25: compute_percentile(&parsed_values, 0.25),
            q75: compute_percentile(&parsed_values, 0.75),
            q90: compute_percentile(&parsed_values, 0.9),
            median: compute_percentile(&parsed_values, 0.5),
            total: compute_total_minutes(&parsed_values),
        };
        filtered_statistics.push(filtered_statistic);
        pb.inc(1);
    }
    pb.set_message("Done");
    pb.finish();

    let json_str = serde_json::to_string_pretty(&statistics).map_err(Error::serde_parse)?;

    let raw_file_name = format!("outputs/{name_prefix}_statistics.json");
    let file_name = Path::new(&raw_file_name);

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_name)
        .map_err(Error::io)?;

    writeln!(file, "{}", json_str).map_err(Error::io)?;

    let json_str = serde_json::to_string_pretty(&statistics).map_err(Error::serde_parse)?;

    let raw_file_name = format!("outputs/{name_prefix}_parsed_statistics.json");
    let file_name = Path::new(&raw_file_name);

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_name)
        .map_err(Error::io)?;

    writeln!(file, "{}", json_str).map_err(Error::io)
}
