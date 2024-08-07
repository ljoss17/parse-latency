use error::Error;
use indicatif::ProgressBar;
use parser::extract::extract_data;
use parser::utils::FailedEntry;
use serde_derive::Deserialize;
use serde_json::Value;
use std::env;
use std::fs::read_to_string;
use utils::{create_progress_bar_template, store_logs};

use parser::per_chain::parse_per_chain_infos;
use parser::total::parse_total_infos;

pub mod error;
pub mod parser;
pub mod utils;

#[derive(Clone, Debug, Deserialize)]
pub struct TimerInfo {
    name: String,
    src_chain: String,
    elapsed: u128,
    #[allow(dead_code)]
    #[serde(flatten)]
    info: Value,
}

impl TryFrom<Value> for TimerInfo {
    type Error = Error;

    fn try_from(mut value: Value) -> Result<Self, Error> {
        let mut infos = value
            .as_object()
            .ok_or_else(|| Error::parse_to_object(value.clone()))?
            .clone();
        let raw_elapsed = value["elapsed"].take();
        let elapsed = raw_elapsed
            .as_u64()
            .ok_or_else(|| Error::parse_to_u64(raw_elapsed))? as u128;
        infos.remove("elapsed");

        let name = value["name"].take().to_string().replace('"', "");
        infos.remove("name");
        let src_chain = if let Some(src_chain) = value.get("src_chain") {
            src_chain.to_string().replace('"', "")
        } else {
            value["chain"].take().to_string().replace('"', "")
        };
        infos.remove("src_chain");

        let info = serde_json::Value::from(infos);

        Ok(Self {
            name,
            src_chain,
            elapsed,
            info,
        })
    }
}

fn main() -> Result<(), Error> {
    let name_prefix = env::var("NAME_PREFIX").unwrap_or_else(|_| "profiling".to_string());
    let mut timer_infos: Vec<TimerInfo> = vec![];

    let input_path = std::env::args()
        .nth(1)
        .ok_or_else(Error::missing_profiling_file)?;
    let text = read_to_string(input_path).map_err(Error::io)?;

    let entries: Vec<&str> = text[1..].split_inclusive("}\n{").collect();
    let mut failed_entries = Vec::new();

    // Initialise progress bar
    let pb = ProgressBar::new(entries.len() as u64);
    pb.set_style(create_progress_bar_template());
    pb.set_message("Extracting data from JSON file");

    for (line, raw_entry) in entries.into_iter().enumerate() {
        let clean_entry = raw_entry.to_string()[0..raw_entry.len() - 3].to_string();
        let parsed_entry = format!("{{{clean_entry}}}");
        let raw_json = serde_json::from_str::<Value>(&parsed_entry);
        match raw_json {
            Ok(raw_json) => {
                let timer_info: TimerInfo = raw_json.try_into()?;
                timer_infos.push(timer_info);
            }
            Err(e) => {
                let failed_entry = FailedEntry::new(line, raw_entry, e.to_string());
                failed_entries.push(failed_entry);
            }
        }
        pb.inc(1);
    }
    pb.set_message("Done");
    pb.finish();

    store_logs(&name_prefix, failed_entries)?;

    let extracted_data = extract_data(timer_infos);

    parse_total_infos(extracted_data.total(), &name_prefix)?;
    parse_per_chain_infos(extracted_data.per_chain(), &name_prefix)?;

    Ok(())
}
