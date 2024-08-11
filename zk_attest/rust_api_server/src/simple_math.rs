

pub fn calculate_mean(numbers: &[f32]) -> f32 {
    let sum: f32 = numbers.iter().sum();
    sum / numbers.len() as f32
}

pub fn calculate_min(numbers: &[f32]) -> f32 {
    *numbers.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap()
}

pub fn calculate_max(numbers: &[f32]) -> f32 {
    *numbers.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap()
}

pub fn calculate_standard_deviation(numbers: &[f32], mean: f32) -> f32 {
    let variance: f32 = numbers.iter().map(|value| {
        let diff = mean - *value;
        diff * diff
    }).sum::<f32>() / numbers.len() as f32;
    variance.sqrt()
}