pub fn compute_mean(values: &Vec<u128>) -> f64 {
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

pub fn compute_percentile(values: &Vec<u128>, percentile: f32) -> u128 {
    let index = (((values.len() + 1) as f32) * percentile) - 1f32;
    values[index.floor() as usize]
}

pub fn std_deviation(values: &[u128], mean: f64) -> f64 {
    let mut variance = 0.0;
    for v in values.iter() {
        let deviation = *v as f64 - mean;
        variance += deviation.powf(2.0);
    }
    let variance = variance / values.len() as f64;

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
