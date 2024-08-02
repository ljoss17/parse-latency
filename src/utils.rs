use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use crate::error::Error;
use crate::parser::utils::FailedEntry;

pub fn compute_mean(values: &[u128]) -> f64 {
    values.iter().sum::<u128>() as f64 / values.len() as f64
}

pub fn compute_total_minutes(values: &[u128]) -> f64 {
    values.iter().sum::<u128>() as f64 / 60000f64
}

pub fn compute_min(values: &[u128]) -> u128 {
    match values.iter().min() {
        Some(min) => *min,
        None => 0,
    }
}

pub fn compute_max(values: &[u128]) -> u128 {
    match values.iter().max() {
        Some(max) => *max,
        None => 0,
    }
}

pub fn compute_percentile(values: &[u128], percentile: f32) -> u128 {
    let mut sorted_values = values.to_vec();
    sorted_values.sort();
    let index = (((sorted_values.len() + 1) as f32) * percentile) - 1f32;
    sorted_values[index.floor() as usize]
}

pub fn std_deviation(values: &[u128], mean: f64) -> f64 {
    let mut variance = 0.0;
    for v in values.iter().map(|v| *v as f64) {
        let deviation = v - mean;
        variance += deviation.powf(2.0);
    }
    let variance = if values.len() == 1 {
        variance
    } else {
        variance / (values.len() - 1) as f64
    };

    variance.sqrt()
}

pub fn z_score(value: u128, mean: f64, std_dev: f64, threshold: f64) -> bool {
    if std_dev == 0.0 {
        return true;
    }
    let z_score = (value as f64 - mean) / std_dev;

    // Keep values inside the threshold
    if z_score.abs() < threshold {
        return true;
    }

    false
}

pub fn store_logs(name_prefix: &str, logs: Vec<FailedEntry>) -> Result<(), Error> {
    let raw_file_name = format!("outputs/{name_prefix}_logs");
    let file_name = Path::new(&raw_file_name);

    let json_str = serde_json::to_string_pretty(&logs).map_err(Error::serde_parse)?;

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_name)
        .map_err(Error::io)?;

    writeln!(file, "{}", json_str).map_err(Error::io)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_mean() {
        let values = [100, 25, 1455, 976, 15555, 26, 777, 134, 843, 271];

        let computed_mean = compute_mean(&values);
        let expected_mean = 2016.2;

        assert_eq!(computed_mean, expected_mean);
    }

    #[test]
    fn test_compute_total_minutes() {
        let values = [100, 100, 100, 100, 100, 100];

        let computed_mean = compute_total_minutes(&values);
        let expected_mean = 0.01;

        assert_eq!(computed_mean, expected_mean);
    }

    #[test]
    fn test_compute_min() {
        let values = [100, 25, 1455, 976, 15555, 26];

        let computed_mean = compute_min(&values);
        let expected_mean = 25;

        assert_eq!(computed_mean, expected_mean);
    }

    #[test]
    fn test_compute_max() {
        let values = [100, 25, 1455, 976, 15555, 26];

        let computed_mean = compute_max(&values);
        let expected_mean = 15555;

        assert_eq!(computed_mean, expected_mean);
    }

    #[test]
    fn test_compute_q10() {
        let values = [100, 25, 1455, 976, 15555, 26, 777, 134, 843, 271];

        let computed_mean = compute_percentile(&values, 0.1);
        let expected_mean = 25;

        assert_eq!(computed_mean, expected_mean);
    }

    #[test]
    fn test_compute_q25() {
        let values = [100, 25, 1455, 976, 15555, 26, 777, 134, 843, 271];

        let computed_mean = compute_percentile(&values, 0.25);
        let expected_mean = 26;

        assert_eq!(computed_mean, expected_mean);
    }

    #[test]
    fn test_compute_q50() {
        let values = [100, 25, 1455, 976, 15555, 26, 777, 134, 843, 271];

        let computed_mean = compute_percentile(&values, 0.5);
        let expected_mean = 271;

        assert_eq!(computed_mean, expected_mean);
    }

    #[test]
    fn test_compute_q75() {
        let values = [100, 25, 1455, 976, 15555, 26, 777, 134, 843, 271];

        let computed_mean = compute_percentile(&values, 0.75);
        let expected_mean = 976;

        assert_eq!(computed_mean, expected_mean);
    }

    #[test]
    fn test_compute_q90() {
        let values = [100, 25, 1455, 976, 15555, 26, 777, 134, 843, 271];

        let computed_mean = compute_percentile(&values, 0.9);
        let expected_mean = 1455;

        assert_eq!(computed_mean, expected_mean);
    }

    #[test]
    fn test_std_deviation() {
        let values = [3, 4, 4, 5, 6, 7, 7, 8, 8, 9];
        let mean = 6.1;

        let computed_mean = std_deviation(&values, mean);
        let expected_mean = 2.05;

        assert!((computed_mean - expected_mean).abs() < 1e-1);
    }

    #[test]
    fn test_z_score_pass() {
        let value = 55;
        let mean = 50.0;
        let std_dev = 5.0;

        let computed_mean = z_score(value, mean, std_dev, 3.0);

        assert!(computed_mean);
    }

    #[test]
    fn test_z_score_fail() {
        let value = 65;
        let mean = 50.0;
        let std_dev = 5.0;

        let computed_mean = z_score(value, mean, std_dev, 3.0);

        assert!(!computed_mean);
    }
}
