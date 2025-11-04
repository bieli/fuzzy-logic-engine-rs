pub fn linspace(min: f64, max: f64, n: usize) -> Vec<f64> {
    if n <= 1 {
        return vec![min];
    }
    let step = (max - min) / (n as f64 - 1.0);
    (0..n).map(|i| min + step * i as f64).collect()
}

pub fn centroid(xs: &[f64], mus: &[f64]) -> f64 {
    let mut num = 0.0;
    let mut den = 0.0;
    for (x, mu) in xs.iter().zip(mus.iter()) {
        num += x * mu;
        den += mu;
    }
    if den == 0.0 {
        xs.get(xs.len() / 2).copied().unwrap_or(0.0)
    } else {
        num / den
    }
}
