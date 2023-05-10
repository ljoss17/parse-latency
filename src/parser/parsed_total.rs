use itertools::Itertools;
use serde_derive::Serialize;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

use crate::utils::*;
use crate::TimerInfo;

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
}

pub fn parse_total_infos_with_filter(infos: Vec<TimerInfo>) {
    let mut elapsed_values: HashMap<String, Vec<u128>> = HashMap::new();
    for info in infos {
        match elapsed_values.get(&info.name) {
            Some(entry) => {
                let mut new_entry = entry.clone();
                new_entry.push(info.elapsed);
                elapsed_values.insert(info.name.clone(), new_entry);
            }
            None => {
                elapsed_values.insert(info.name.clone(), vec![info.elapsed]);
            }
        }
    }

    let mut statistics = vec![];
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
            count: parsed_values.len() as u128,
            mean: compute_mean(&parsed_values),
            min: compute_min(&parsed_values),
            max: compute_max(&parsed_values),
            q10: compute_percentile(&parsed_values, 0.1),
            q25: compute_percentile(&parsed_values, 0.25),
            q75: compute_percentile(&parsed_values, 0.75),
            q90: compute_percentile(&parsed_values, 0.9),
            median: compute_percentile(&parsed_values, 0.5),
        };
        statistics.push(statistic);
    }

    let json_str = match serde_json::to_string_pretty(&statistics) {
        Ok(value) => value,
        Err(_) => format!("{statistics:?}"), // Fallback to debug printing
    };

    let file_name = Path::new("outputs/parsed_statistics.json");

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_name)
        .unwrap();

    if let Err(e) = writeln!(file, "{}", json_str) {
        println!("Couldn't write to file: {}", e);
    }
}
