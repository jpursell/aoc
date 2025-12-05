use std::ops::RangeInclusive;

use crate::AocSolution;

pub struct Day05;

struct Puzzle {
    ranges: Vec<RangeInclusive<u64>>,
    ids: Vec<u64>,
}

fn parse(input: &str) -> Puzzle {
    let empty_line_index = input
        .lines()
        .enumerate()
        .filter(|(_, x)| x.is_empty())
        .map(|(i, _)| i)
        .next()
        .expect("did not find empty line");
    let ids = input
        .lines()
        .skip(empty_line_index + 1)
        .map(|line| line.parse().expect("failed to parse id"))
        .collect();
    let ranges = input
        .lines()
        .take(empty_line_index)
        .map(|s| {
            let (low, high) = s.split_once('-').expect("failed to split range");
            low.parse().expect("failed to parse low")..=high.parse().expect("failed to parse high")
        })
        .collect();
    Puzzle { ranges, ids }
}

fn combine_ranges(mut ranges: Vec<RangeInclusive<u64>>) -> Vec<RangeInclusive<u64>> {
    // let mut puzzle = parse(input);
    let mut combined_ranges: Vec<RangeInclusive<u64>> = Vec::with_capacity(ranges.len());
    let mut next = None;
    while !ranges.is_empty() || next.is_some() {
        if next.is_none() {
            next = ranges.pop();
        }
        let mut matched = None;
        for (index, range) in combined_ranges.iter().enumerate() {
            let next_inner = next.as_ref().unwrap();
            if range.contains(next_inner.start()) || range.contains(next_inner.end()) {
                matched = Some(index);
                break;
            }
        }
        if let Some(index) = matched {
            // Combine and set next
            let (start0, end0) = combined_ranges.swap_remove(index).into_inner();
            let (start1, end1) = next.take().unwrap().into_inner();
            let start = start0.min(start1);
            let end = end0.max(end1);
            next = Some(start..=end);
        } else {
            // insert next and set next to None
            combined_ranges.push(next.take().unwrap());
        }
    }
    combined_ranges
}

impl AocSolution for Day05 {
    fn part1(&self, input: &str) -> String {
        let Puzzle { ranges, ids } = parse(input);
        let ranges = combine_ranges(ranges);
        ids.iter()
            .filter(|id| ranges.iter().any(|range| range.contains(id)))
            .count()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let Puzzle { ranges, ids: _ } = parse(input);
        let combined_ranges = combine_ranges(ranges);
        combined_ranges
            .into_iter()
            .map(|r| r.count() as u64)
            .sum::<u64>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_range() {
        let range: RangeInclusive<u64> = 1..=3;
        assert_eq!(range.clone().count(), 3);
        assert!(!range.contains(&0));
        assert!(range.contains(&1));
        assert!(range.contains(&2));
        assert!(range.contains(&3));
        assert!(!range.contains(&4));
    }
    #[test]
    fn test_part1_example() {
        assert_eq!(Day05.part1(EXAMPLE), "3");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2025, 5).expect("Failed to get input");
        assert_eq!(Day05.part1(&input), "712");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(Day05.part2(EXAMPLE), "14");
    }

    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2025, 5).expect("Failed to get input");
        let too_high = 338461571472310_u64;
        let result = Day05.part2(&input);
        let result_val: u64 = result.parse().unwrap();
        assert!(result_val < too_high);
        assert_eq!(Day05.part2(&input), "REPLACE_WITH_PART2_FULL_RESULT");
    }
}
