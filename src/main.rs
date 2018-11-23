use std::collections::HashMap;

fn min_distance_inner(
    current: usize,
    visited: u128,
    cities: &Vec<(f64, f64)>,
    memo: &mut HashMap<(usize, u128), f64>,
) -> f64 {
    if visited == (1 << cities.len()) - 1 {
        ((cities[current].0 - cities[0].0).powf(2.0) + (cities[current].1 - cities[0].1).powf(2.0))
            .sqrt()
    } else if let Some(&distance) = memo.get(&(current, visited)) {
        distance
    } else {
        let mut min_distance = std::f64::INFINITY;
        for i in 0..cities.len() {
            if (visited >> i) & 1 == 0 {
                let distance = min_distance_inner(i, visited | (1 << i), cities, memo)
                    + ((cities[current].0 - cities[i].0).powf(2.0)
                        + (cities[current].1 - cities[i].1).powf(2.0)).sqrt();
                if distance < min_distance {
                    min_distance = distance;
                }
            }
        }
        memo.insert((current, visited), min_distance);
        min_distance
    }
}

fn min_distance(cities: &Vec<(f64, f64)>) -> f64 {
    min_distance_inner(0, 1, cities, &mut HashMap::new())
}

fn main() {
    println!(
        "{}",
        min_distance(&vec![(0.0, 0.0), (-1.0, 0.0), (3.0, 0.0), (-6.0, 0.0)])
    );
}
