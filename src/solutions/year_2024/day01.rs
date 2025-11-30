use crate::AocSolution;
use counter::Counter;

// Day 1: Separate struct for each day
pub struct Day01;

fn extract_lists(str: &str) -> [Vec<u32>; 2] {
    let count = str.lines().count();
    let mut vecs = [Vec::with_capacity(count), Vec::with_capacity(count)];
    for line in str.lines() {
        let (a, b) = line.split_once(" ").unwrap();
        let a: u32 = a.parse().unwrap();
        let b: u32 = b.trim().parse().unwrap();
        vecs[0].push(a);
        vecs[1].push(b);
    }
    vecs
}
fn process_lists(mut vecs: [Vec<u32>; 2]) -> u32 {
    vecs[0].sort();
    vecs[1].sort();
    let mut diff = 0;
    for (a, b) in vecs[0].iter().zip(vecs[1].iter()) {
        diff += a.abs_diff(*b);
    }
    diff
}
fn extract_lists_2(str: &str) -> [Vec<usize>; 2] {
    let count = str.lines().count();
    let mut vecs = [Vec::with_capacity(count), Vec::with_capacity(count)];
    for line in str.lines() {
        let (a, b) = line.split_once(" ").unwrap();
        let a: usize = a.parse().unwrap();
        let b: usize = b.trim().parse().unwrap();
        vecs[0].push(a);
        vecs[1].push(b);
    }
    vecs
}
fn process_lists_2(vecs: [Vec<usize>; 2]) -> usize {
    let counter = vecs[1].iter().collect::<Counter<_>>();
    let mut out = 0;
    for a in vecs[0].iter() {
        out += a * counter[a];
    }
    out
}

impl AocSolution for Day01 {
    fn part1(&self, input: &str) -> String {
        process_lists(extract_lists(input)).to_string()
    }

    fn part2(&self, input: &str) -> String {
        process_lists_2(extract_lists_2(input)).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_part1_example() {
        assert_eq!(Day01.part1(EXAMPLE), "11");
    }
    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2024, 1).expect("Failed to get input");
        assert_eq!(Day01.part1(&input), "2164381");
    }
    #[test]
    fn test_part2_example() {
        assert_eq!(Day01.part2(EXAMPLE), "31");
    }
    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2024, 1).expect("Failed to get input");
        assert_eq!(Day01 {}.part2(&input), "20719933");
    }
}
