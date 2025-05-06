pub fn euclidean_distance(a: &[f32], b: &[f32]) -> f64 {
    a.iter().zip(b.iter())
        .map(|(x, y)| ((*x - *y).powi(2)) as f64)
        .sum::<f64>()
        .sqrt()
}