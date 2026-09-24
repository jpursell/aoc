use crate::AocSolution;
use itertools::{Combinations, Itertools};
use rayon::prelude::*;
use std::str::FromStr;

pub struct Day12;

mod a {
    use super::*;

    #[derive(Clone, Copy, PartialEq, Debug)]
    pub enum Condition {
        Operational,
        Damaged,
        Unknown,
    }

    impl TryFrom<char> for Condition {
        type Error = &'static str;
        fn try_from(c: char) -> Result<Self, <Self as TryFrom<char>>::Error> {
            match c {
                '.' => Ok(Condition::Operational),
                '#' => Ok(Condition::Damaged),
                '?' => Ok(Condition::Unknown),
                _ => Err("Unknown char"),
            }
        }
    }

    #[derive(Debug)]
    pub struct SpringRecord {
        pub record: Vec<Condition>,
        pub groups: Vec<u32>,
    }

    pub struct SpringRecordIterator {
        record: Vec<Condition>,
        groups: Vec<u32>,
        combinations: Combinations<std::vec::IntoIter<usize>>,
    }

    impl SpringRecordIterator {
        fn new(record: &SpringRecord) -> SpringRecordIterator {
            let loc = record
                .record
                .iter()
                .enumerate()
                .filter(|(_, &c)| c == Condition::Unknown)
                .map(|(i, _)| i)
                .collect::<Vec<_>>();
            let groups = record.groups.clone();
            let record = record
                .record
                .iter()
                .map(|&c| {
                    if c == Condition::Unknown {
                        Condition::Operational
                    } else {
                        c
                    }
                })
                .collect::<Vec<_>>();
            let mut tot = groups.iter().map(|&x| x as usize).sum();
            tot -= record.iter().filter(|&&c| c == Condition::Damaged).count();
            let combinations = loc.into_iter().combinations(tot);

            SpringRecordIterator {
                record,
                groups,
                combinations,
            }
        }

        fn check(&self, condition: &Vec<Condition>) -> bool {
            let groups = condition
                .split(|&c| c == Condition::Operational)
                .map(|c| c.len())
                .filter(|&n| n > 0)
                .collect::<Vec<_>>();
            if groups.len() != self.groups.len() {
                return false;
            }
            groups
                .iter()
                .zip(self.groups.iter())
                .all(|(&x, &y)| x == y as usize)
        }
    }

    impl Iterator for SpringRecordIterator {
        type Item = Vec<Condition>;
        fn next(&mut self) -> Option<<Self as Iterator>::Item> {
            loop {
                let Some(loc) = self.combinations.next() else {
                    return None;
                };
                let mut out = self.record.clone();
                loc.iter().for_each(|&i| out[i] = Condition::Damaged);
                if self.check(&out) {
                    return Some(out);
                }
            }
        }
    }

    impl FromStr for SpringRecord {
        type Err = &'static str;
        fn from_str(line: &str) -> Result<Self, <Self as FromStr>::Err> {
            let (record, groups) = line.split_once(' ').unwrap();
            let record = record
                .chars()
                .map(|c| Condition::try_from(c).unwrap())
                .collect::<Vec<_>>();
            let groups = groups
                .split(',')
                .map(|num| num.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            Ok(SpringRecord { record, groups })
        }
    }

    impl SpringRecord {
        pub fn count_solutions(&self) -> usize {
            self.iter().count()
        }

        fn iter(&self) -> SpringRecordIterator {
            SpringRecordIterator::new(self)
        }
    }

    pub fn run(input: &str) -> usize {
        input
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.parse::<SpringRecord>().unwrap())
            .map(|sr| sr.count_solutions())
            .sum()
    }
}

mod b {
    use super::*;

    #[derive(Clone, Copy, PartialEq, Debug)]
    pub enum Condition {
        Opr,
        Dam,
        Unk,
    }

    impl TryFrom<char> for Condition {
        type Error = &'static str;
        fn try_from(c: char) -> Result<Self, <Self as TryFrom<char>>::Error> {
            match c {
                '.' => Ok(Condition::Opr),
                '#' => Ok(Condition::Dam),
                '?' => Ok(Condition::Unk),
                _ => Err("Unknown char"),
            }
        }
    }

    #[derive(Clone, Debug)]
    pub struct Solution {
        pub extra_space: usize,
        pub record_pos: usize,
        pub group_pos: usize,
    }

    impl Solution {
        pub fn new(sr: &SpringRecord) -> Solution {
            let extra_space =
                sr.record.len() - sr.groups.iter().sum::<usize>() - (sr.groups.len() - 1);
            Solution {
                extra_space,
                record_pos: 0,
                group_pos: 0,
            }
        }

        pub fn complete(&self, sr: &SpringRecord) -> bool {
            self.group_pos == sr.groups.len()
                && self.record_pos == sr.record.len()
                && self.extra_space == 0
        }

        pub fn push(&self, n_spaces: usize, sr: &SpringRecord) -> Option<Solution> {
            let mut s = self.clone();
            if s.extra_space < n_spaces {
                panic!()
            }
            s.extra_space -= n_spaces;
            for _ in 0..n_spaces {
                if s.record_pos == sr.record.len() {
                    return None;
                }
                if sr.record[s.record_pos] == Condition::Dam {
                    return None;
                }
                s.record_pos += 1;
            }
            if s.group_pos == sr.groups.len() {
                if s.complete(sr) {
                    return Some(s);
                } else {
                    return None;
                }
            }
            for _ in 0..sr.groups[s.group_pos] {
                if s.record_pos == sr.record.len() {
                    return None;
                }
                if sr.record[s.record_pos] == Condition::Opr {
                    return None;
                }
                s.record_pos += 1;
            }
            s.group_pos += 1;
            if s.complete(sr) {
                return Some(s);
            }
            if s.record_pos == sr.record.len() {
                return None;
            }
            if sr.record[s.record_pos] == Condition::Dam {
                return None;
            }
            s.record_pos += 1;
            if s.group_pos == sr.groups.len() {
                s.extra_space -= 1;
            }
            Some(s)
        }
    }

    #[derive(Debug)]
    pub struct SpringRecord {
        pub record: Vec<Condition>,
        pub groups: Vec<usize>,
    }

    impl FromStr for SpringRecord {
        type Err = &'static str;
        fn from_str(line: &str) -> Result<Self, <Self as FromStr>::Err> {
            let (record, groups) = line.split_once(' ').unwrap();
            let record = record
                .chars()
                .map(|c| Condition::try_from(c).unwrap())
                .collect::<Vec<_>>();
            let groups = groups
                .split(',')
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            Ok(SpringRecord::new(record, groups))
        }
    }

    impl SpringRecord {
        pub fn new(record: Vec<Condition>, groups: Vec<usize>) -> SpringRecord {
            SpringRecord { record, groups }
        }

        pub fn multiply(&self, count: usize) -> SpringRecord {
            let record = (0..count)
                .map(|_| self.record.clone())
                .collect::<Vec<_>>()
                .join(&Condition::Unk);
            let groups = (0..count)
                .map(|_| self.groups.clone())
                .collect::<Vec<_>>()
                .concat();
            SpringRecord { record, groups }
        }

        pub fn count_solutions(&self) -> usize {
            self.count_solutions_inner(&Solution::new(self))
        }

        fn count_solutions_inner(&self, solution: &Solution) -> usize {
            if solution.complete(self) {
                return 1;
            }
            let mut count = 0;
            for n_spaces in 0..=solution.extra_space {
                if let Some(s) = solution.push(n_spaces, self) {
                    count += self.count_solutions_inner(&s);
                }
            }
            count
        }
    }

    pub fn run(input: &str) -> usize {
        let records = input
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.parse::<SpringRecord>().unwrap().multiply(5))
            .collect::<Vec<_>>();
        records.par_iter().map(|sr| sr.count_solutions()).sum()
    }
}

impl AocSolution for Day12 {
    fn part1(&self, input: &str) -> String {
        a::run(input).to_string()
    }

    fn part2(&self, input: &str) -> String {
        b::run(input).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#;

    #[test]
    fn test_part1_example() {
        assert_eq!(Day12.part1(EXAMPLE), "21");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2023, 12).expect("Failed to get input");
        assert_eq!(Day12.part1(&input), "7191");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(Day12.part2(EXAMPLE), "525152");
    }

    #[test]
    #[ignore = "slow in aoc23 (>10 minutes with rayon)"]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2023, 12).expect("Failed to get input");
        assert_eq!(Day12.part2(&input), "6512849198636");
    }
}
