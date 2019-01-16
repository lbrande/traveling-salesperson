use rand::random;
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    for city_count in 0..=12 {
        println!("{} cities:", city_count);
        let mut cities = Vec::new();
        for _ in 0..city_count {
            cities.push(random());
        }
        let start = Instant::now();
        min_distance_dp(0, 1, &cities, &mut HashMap::new());
        println!(" DP took {} seconds", seconds_elapsed(start));
        let start = Instant::now();
        min_distance_naive(0, 1, &cities);
        println!(" Naive took {} seconds", seconds_elapsed(start));
    }
}

fn seconds_elapsed(instant: Instant) -> f64 {
    let elapsed_time = instant.elapsed();
    elapsed_time.as_secs() as f64 + f64::from(elapsed_time.subsec_millis()) / 1000.0
}

fn min_distance_dp(
    current: usize,
    visited: u64,
    cities: &[(f64, f64)],
    memo: &mut HashMap<(usize, u64), f64>,
) -> f64 {
    if let Some(&distance) = memo.get(&(current, visited)) {
        distance
    } else if visited == (1 << cities.len()) - 1 {
        distance(cities[current], cities[0])
    } else {
        let mut min_distance = std::f64::INFINITY;
        for i in 0..cities.len() {
            if (visited >> i) & 1 == 0 {
                let distance = min_distance_dp(i, visited | (1 << i), cities, memo)
                    + distance(cities[current], cities[i]);
                if distance < min_distance {
                    min_distance = distance;
                }
            }
        }
        memo.insert((current, visited), min_distance);
        min_distance
    }
}

fn min_distance_naive(current: usize, visited: u64, cities: &[(f64, f64)]) -> f64 {
    if visited == (1 << cities.len()) - 1 {
        distance(cities[current], cities[0])
    } else {
        let mut min_distance = std::f64::INFINITY;
        for i in 0..cities.len() {
            if (visited >> i) & 1 == 0 {
                let distance = min_distance_naive(i, visited | (1 << i), cities)
                    + distance(cities[current], cities[i]);
                if distance < min_distance {
                    min_distance = distance;
                }
            }
        }
        min_distance
    }
}

fn distance(pos0: (f64, f64), pos1: (f64, f64)) -> f64 {
    ((pos0.0 - pos1.0).powf(2.0) + (pos0.1 - pos1.1).powf(2.0)).sqrt()
}
