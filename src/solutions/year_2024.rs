use crate::AocSolution;
use counter::Counter;

// Day 1: Separate struct for each day
pub struct Day01;

impl Day01 {
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
}

impl AocSolution for Day01 {
    fn part1(&self, input: &str) -> String {
        let expected = "2164381";
        let ret = Day01::process_lists(Day01::extract_lists(input)).to_string();
        assert_eq!(ret, expected);
        ret
    }

    fn part2(&self, input: &str) -> String {
        let expected = "20719933";
        let ret = Day01::process_lists_2(Day01::extract_lists_2(input)).to_string();
        assert_eq!(ret, expected);
        ret
    }
}

// Map of all solutions for a given year
pub fn get_solutions() -> std::collections::HashMap<u8, Box<dyn AocSolution>> {
    let mut map: std::collections::HashMap<u8, Box<dyn AocSolution>> =
        std::collections::HashMap::new();
    // Register every day here
    map.insert(1, Box::new(Day01));
    // map.insert(2, Box::new(Day02));
    map
}
