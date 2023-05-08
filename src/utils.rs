use serde_json::Value;

pub fn compute_mean(values: &Vec<u128>) -> f64 {
    values.iter().sum::<u128>() as f64 / values.len() as f64
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

pub fn compute_percentile(values: &mut Vec<u128>, percentile: f32) -> u128 {
    let index = (((values.len() + 1) as f32) * percentile) - 1f32;
    values[index.floor() as usize]
}