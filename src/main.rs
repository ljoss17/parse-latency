use serde_derive::Deserialize;
use serde_json::Value;
use std::fs::read_to_string;

use parser::per_chain::parse_per_chain_infos;
use parser::total::parse_total_infos;

use crate::parser::parsed_per_chain::parse_per_chain_infos_with_filter;
use crate::parser::parsed_total::parse_total_infos_with_filter;

pub mod parser;
pub mod utils;

#[derive(Clone, Debug, Deserialize)]
pub struct TimerInfo {
    name: String,
    src_chain: String,
    elapsed: u128,
    #[serde(flatten)]
    info: Value,
}

impl From<Value> for TimerInfo {
    fn from(mut value: Value) -> Self {
        let mut infos = value.as_object().unwrap().clone();
        let raw_elapsed = value["elapsed"].take();
        let elapsed = raw_elapsed.as_u64().unwrap() as u128;
        infos.remove("elapsed");

        let name = value["name"].take().to_string().replace('"', "");
        infos.remove("name");
        let src_chain = value["src_chain"].take().to_string().replace('"', "");
        infos.remove("src_chain");

        let info = serde_json::Value::from(infos);

        Self {
            name,
            src_chain,
            elapsed,
            info,
        }
    }
}

fn main() {
    let mut timer_infos: Vec<TimerInfo> = vec![];

    let input_path = std::env::args().nth(1).unwrap();
    let text = read_to_string(input_path).unwrap();

    let mut skip = 0;
    for raw_entry in text[1..].split_inclusive("}{") {
        let clean_entry = raw_entry.to_string()[0..raw_entry.len() - 2].to_string();
        let parsed_entry = format!("{{{clean_entry}}}");
        let raw_json = serde_json::from_str::<Value>(&parsed_entry);
        match raw_json {
            Ok(raw_json) => {
                let timer_info: TimerInfo = raw_json.into();
                timer_infos.push(timer_info);
            }
            Err(_) => {
                skip += 1;
            }
        }
    }

    println!("Skipped {skip} entries");

    parse_total_infos(timer_infos.clone());
    parse_per_chain_infos(timer_infos.clone());
    parse_total_infos_with_filter(timer_infos.clone());
    parse_per_chain_infos_with_filter(timer_infos);
}
