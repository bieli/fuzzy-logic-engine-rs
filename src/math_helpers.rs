/*
It’s basically chopping up the interval [min, max] into evenly spaced points.

Example: linspace(0.0, 10.0, 6):

x-axis: 0                          10
        |---------------------------|
        ^    ^    ^    ^    ^    ^
        0    2    4    6    8   10

- The bar represents the interval from min=0 to max=10.
- The arrows ^ mark the 6 evenly spaced points.
- Step size = (10 - 0) / (6 - 1) = 2.

linspace function is just:

- Draw a line from min to max.
- Divide it into n-1 equal segments.
- Mark each division point, including the ends.
*/
pub fn linspace(min: f64, max: f64, n: usize) -> Vec<f64> {
    if n <= 1 {
        return vec![min];
    }
    let step = (max - min) / (n as f64 - 1.0);
    (0..n).map(|i| min + step * i as f64).collect()
}

/*
Visualizing the centroid

Imagine a fuzzy set shaped like a triangle:

 μ(x)
 1.0 |           /\
     |          /  \
     |         /    \
 0.5 |        /      \
     |       /    |   \
 0.0 |------/-----C----\---------
        a    b    ↑     c
                  |
              centroid

- The centroid C is the “center of gravity” of the fuzzy shape.
- If the fuzzy set is symmetric, the centroid is right in the middle.
- If the set is skewed, the centroid shifts toward the heavier side.
*/
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
