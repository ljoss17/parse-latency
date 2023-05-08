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

pub fn parse_total_infos(infos: Vec<TimerInfo>) {
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
    for timer in elapsed_values.iter_mut() {
        let name = timer.0.clone();
        timer.1.sort();
        let statistic = TimerStatistics {
            name,
            count: timer.1.len() as u128,
            mean: compute_mean(timer.1),
            min: compute_min(timer.1),
            max: compute_max(timer.1),
            q10: compute_percentile(timer.1, 0.1),
            q25: compute_percentile(timer.1, 0.25),
            q75: compute_percentile(timer.1, 0.75),
            q90: compute_percentile(timer.1, 0.9),
            median: compute_percentile(timer.1, 0.5),
        };
        statistics.push(statistic);
    }

    let json_str = match serde_json::to_string_pretty(&statistics) {
        Ok(value) => value,
        Err(_) => format!("{statistics:?}"), // Fallback to debug printing
    };

    let file_name = Path::new("outputs/statistics.json");

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
