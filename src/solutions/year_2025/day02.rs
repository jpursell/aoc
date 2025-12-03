use crate::AocSolution;
use std::collections::BTreeSet;

pub struct Day02;

fn find_doubles(low: &str, high: &str) -> BTreeSet<u64> {
    let low_val: u64 = low.trim().parse().unwrap();
    let high_val: u64 = high.trim().parse().unwrap();

    let nlow = low.len();
    let nhigh = high.len();

    let start_length = (nlow / 2).max(1);
    let end_length = (nhigh / 2).max(1);

    let mut bad_ids = BTreeSet::new();

    for length in start_length..=end_length {
        let start_val = 10_u64.pow((length - 1) as u32);
        let shift = start_val * 10;
        for val in start_val..start_val * 10 {
            let test = val + val * shift;
            if test >= low_val && test <= high_val {
                bad_ids.insert(test);
            }
        }
    }
    bad_ids
}

fn find_repeats(low: &str, high: &str) -> BTreeSet<u64> {
    let low_val: u64 = low.trim().parse().unwrap();
    let high_val: u64 = high.trim().parse().unwrap();

    let nlow = low.len();
    let nhigh = high.len();

    let start_length = (nlow / 2).max(1);
    let end_length = (nhigh / 2).max(1);

    let mut bad_ids = BTreeSet::new();

    for length in start_length..=end_length {
        let start_val = 10_u64.pow((length - 1) as u32);
        let shift = start_val * 10;
        for val in start_val..start_val * 10 {
            let test = val + val * shift;
            if test >= low_val && test <= high_val {
                bad_ids.insert(test);
            }
        }
    }
    bad_ids
}

impl AocSolution for Day02 {
    fn part1(&self, input: &str) -> String {
        input
            .split(",")
            .map(|s| s.split_once("-").unwrap())
            .map(|(low, high)| find_doubles(low, high))
            .fold(BTreeSet::new(), |mut acc, s| {
                acc.extend(s);
                acc
            })
            .iter()
            .sum::<u64>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        input
            .split(",")
            .map(|s| s.split_once("-").unwrap())
            .map(|(low, high)| find_repeats(low, high))
            .fold(BTreeSet::new(), |mut acc, s| {
                acc.extend(s);
                acc
            })
            .iter()
            .sum::<u64>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_count_range_998_1012() {
        let expected = BTreeSet::from([1010_u64]);
        assert_eq!(expected, find_doubles("998", "1012"));
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(Day02.part1(EXAMPLE), "1227775554");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2025, 2).expect("Failed to get input");
        assert_eq!(Day02.part1(&input), "23560874270");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(Day02.part2(EXAMPLE), "4174379265");
    }

    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2025, 2).expect("Failed to get input");
        assert_eq!(Day02.part2(&input), "REPLACE_WITH_PART2_FULL_RESULT");
    }
}
