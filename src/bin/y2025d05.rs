use rand::{rngs::ThreadRng, Rng};
use std::{collections::BTreeSet, ops::RangeInclusive};

use aoc::solutions::year_2025::day05::combine_ranges;
fn random_range(rng: &mut ThreadRng, max: u64, maxrange: u64) -> RangeInclusive<u64> {
    let low: u64 = rng.random_range(0..=max);
    let range: u64 = rng.random_range(0..=maxrange);
    let high = low + range;
    low..=high
}

fn brute_force_count(ranges: &[RangeInclusive<u64>]) -> u64 {
    ranges
        .iter()
        .fold(BTreeSet::new(), |mut set, range| {
            set.append(&mut range.clone().collect::<BTreeSet<u64>>());
            set
        })
        .len() as u64
}

fn try_random(rng: &mut ThreadRng, max: u64, maxrange: u64, nranges: u64) -> Result<(), ()> {
    let ranges: Vec<RangeInclusive<u64>> = (0..nranges)
        .map(|_| random_range(rng, max, maxrange))
        .collect();

    let brute_count = brute_force_count(&ranges);
    let combined_ranges = combine_ranges(ranges.clone());
    let combined_count: u64 = combined_ranges
        .iter()
        .map(|range| range.clone().count() as u64)
        .sum();
    if combined_count != brute_count {
        println!("ranges:");
        for range in &ranges {
            println!("{:?}", range);
        }
        println!("combined ranges:");
        for range in &combined_ranges {
            println!("{:?} -> {}", range, range.clone().count());
        }
        println!(
            "combined count: {} != brute count: {}",
            combined_count, brute_count
        );
        let combined_brute = brute_force_count(&combined_ranges);
        println!("combined brute: {}", combined_brute);
        Err(())
    } else {
        Ok(())
    }
}

fn main() {
    let mut rng = rand::rng();
    let max = 100;
    let maxrange = 10;
    let nranges = 3;
    for i in 0..1000 {
        println!("try: {}", i);
        try_random(&mut rng, max, maxrange, nranges).expect("try failed")
    }
}
