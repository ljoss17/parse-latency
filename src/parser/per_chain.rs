use itertools::Itertools;
use serde_derive::Serialize;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

use crate::error::Error;
use crate::utils::*;
use crate::TimerInfo;

#[derive(Debug, Serialize)]
struct PerChainTimerStatistics {
    name: String,
    chain: String,
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

pub fn parse_per_chain_infos(infos: Vec<TimerInfo>, name_prefix: &String) -> Result<(), Error> {
    let mut elapsed_values: HashMap<(String, String), Vec<u128>> = HashMap::new();
    for info in infos {
        let entry_identifier = (info.name, info.src_chain);
        match elapsed_values.get(&entry_identifier) {
            Some(entry) => {
                let mut new_entry = entry.clone();
                new_entry.push(info.elapsed);
                elapsed_values.insert(entry_identifier.clone(), new_entry);
            }
            None => {
                elapsed_values.insert(entry_identifier.clone(), vec![info.elapsed]);
            }
        }
    }

    let mut statistics = vec![];
    for (name, chain) in elapsed_values.keys().sorted() {
        let mut values = elapsed_values[&(name.clone(), chain.clone())].clone();
        values.sort();
        let statistic = PerChainTimerStatistics {
            name: name.clone(),
            chain: chain.clone(),
            count: values.len() as u128,
            mean: compute_mean(&values),
            min: compute_min(&values),
            max: compute_max(&values),
            q10: compute_percentile(&values, 0.1),
            q25: compute_percentile(&values, 0.25),
            q75: compute_percentile(&values, 0.75),
            q90: compute_percentile(&values, 0.9),
            median: compute_percentile(&values, 0.5),
        };
        statistics.push(statistic);
    }

    let json_str = serde_json::to_string_pretty(&statistics).map_err(Error::serde_parse)?;

    let raw_file_name = format!("outputs/{name_prefix}_per_chain_statistics.json");
    let file_name = Path::new(&raw_file_name);

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_name)
        .map_err(Error::io)?;

    writeln!(file, "{}", json_str).map_err(Error::io)
}
