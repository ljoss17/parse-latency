use indicatif::ProgressBar;
use std::collections::HashMap;

use crate::utils::create_progress_bar_template;
use crate::TimerInfo;

pub struct ExtractedData {
    total: HashMap<String, Vec<u128>>,
    per_chain: HashMap<(String, String), Vec<u128>>,
}

impl ExtractedData {
    pub fn total(&self) -> &HashMap<String, Vec<u128>> {
        &self.total
    }

    pub fn per_chain(&self) -> &HashMap<(String, String), Vec<u128>> {
        &self.per_chain
    }
}

pub fn extract_data(infos: Vec<TimerInfo>) -> ExtractedData {
    // Initialise progress bar
    let pb = ProgressBar::new(infos.len() as u64);
    pb.set_style(create_progress_bar_template());
    pb.set_message("Extract data");

    let mut elapsed_values_total: HashMap<String, Vec<u128>> = HashMap::new();
    let mut elapsed_values_per_chain: HashMap<(String, String), Vec<u128>> = HashMap::new();
    for info in infos {
        // total
        match elapsed_values_total.get(&info.name) {
            Some(entry) => {
                let mut new_entry = entry.clone();
                new_entry.push(info.elapsed);
                elapsed_values_total.insert(info.name.clone(), new_entry);
            }
            None => {
                elapsed_values_total.insert(info.name.clone(), vec![info.elapsed]);
            }
        }
        // per chain
        let entry_identifier = (info.name, info.src_chain);
        match elapsed_values_per_chain.get(&entry_identifier) {
            Some(entry) => {
                let mut new_entry = entry.clone();
                new_entry.push(info.elapsed);
                elapsed_values_per_chain.insert(entry_identifier.clone(), new_entry);
            }
            None => {
                elapsed_values_per_chain.insert(entry_identifier.clone(), vec![info.elapsed]);
            }
        }
        pb.inc(1);
    }
    pb.set_message("Done");
    pb.finish();

    ExtractedData {
        total: elapsed_values_total,
        per_chain: elapsed_values_per_chain,
    }
}
