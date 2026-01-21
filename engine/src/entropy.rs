use std::collections::HashMap;

pub fn shannon_entropy(data: &str) -> f64 {
    let mut freq = HashMap::new();
    let len = data.len() as f64;

    for c in data.chars() {
        *freq.entry(c).or_insert(0) += 1;
    }

    freq.values()
        .map(|&count| {
            let p = count as f64 / len;
            -p * p.log2()
        })
        .sum()
}