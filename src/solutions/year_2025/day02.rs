use crate::AocSolution;
use std::collections::BTreeSet;

pub struct Day02;

fn count_range(low: &str, high: &str) -> BTreeSet<u64> {
    let nlow = low.len();
    let nhigh = high.len();
    let nmax = nhigh / 2;
    let low_val: u64 = low.parse().unwrap();
    let high_val: u64 = high.parse().unwrap();
    let mut bad_ids = BTreeSet::new();
    let length = (nlow / 2).max(1);
    let mut start = 10_u64.pow(length as u32);

    loop {
        let pattern = start.to_string();
        let length = pattern.len();
        if pattern.len() > nmax {
            break;
        }
        for target_length in nlow..=nhigh {
            if target_length % length != 0 {
                continue;
            }
            let nrep = target_length / length;
            if nrep != 2 {
                continue;
            }
            let value: u64 = pattern.repeat(nrep).parse().unwrap();
            if value >= low_val && value <= high_val {
                bad_ids.insert(value);
            }
        }
        start += 1;
    }
    bad_ids
}

fn process(input: &str) -> u64 {
    input
        .split(",")
        .map(|s| s.split_once("-").unwrap())
        .map(|(low, high)| count_range(low, high))
        .fold(BTreeSet::new(), |mut acc, s| {
            acc.extend(s);
            acc
        })
        .iter()
        .sum()
}

impl AocSolution for Day02 {
    fn part1(&self, input: &str) -> String {
        process(input).to_string()
    }

    fn part2(&self, _input: &str) -> String {
        "Not implemented".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_count_range_998_1012() {
        let expected = BTreeSet::from([1010_u64]);
        assert_eq!(expected, count_range("998", "1012"));
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(Day02.part1(EXAMPLE), "1227775554");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2025, 2).expect("Failed to get input");
        assert_eq!(Day02.part1(&input), "REPLACE_WITH_PART1_FULL_RESULT");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(Day02.part2(EXAMPLE), "REPLACE_WITH_PART2_EXAMPLE_RESULT");
    }

    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2025, 2).expect("Failed to get input");
        assert_eq!(Day02.part2(&input), "REPLACE_WITH_PART2_FULL_RESULT");
    }
}
