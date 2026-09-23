use std::collections::HashMap;

use crate::AocSolution;

pub struct Day12;

fn count_arrangements(
    pattern: &[u8],
    groups: &[usize],
    memo: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    if groups.is_empty() {
        return if pattern.contains(&b'#') { 0 } else { 1 };
    }
    if pattern.is_empty() {
        return 0;
    }

    let key = (pattern.len(), groups.len());
    if let Some(&ans) = memo.get(&key) {
        return ans;
    }

    let mut total = 0;
    let c = pattern[0];

    // Branch 1: treat current as '.'
    if c == b'.' || c == b'?' {
        total += count_arrangements(&pattern[1..], groups, memo);
    }

    // Branch 2: treat current as start of group '#'
    if c == b'#' || c == b'?' {
        let g = groups[0];
        if pattern.len() >= g && !pattern[..g].contains(&b'.') {
            if pattern.len() == g {
                total += count_arrangements(&[], &groups[1..], memo);
            } else if pattern[g] != b'#' {
                total += count_arrangements(&pattern[g + 1..], &groups[1..], memo);
            }
        }
    }

    memo.insert(key, total);
    total
}

fn solve_line_part1(line: &str) -> u64 {
    let (pattern, groups_str) = line.split_once(' ').unwrap();
    let groups: Vec<usize> = groups_str
        .split(',')
        .filter_map(|s| s.parse::<usize>().ok())
        .collect();
    let mut memo = HashMap::new();
    count_arrangements(pattern.as_bytes(), &groups, &mut memo)
}

fn solve_line_part2(line: &str) -> u64 {
    let (pattern_str, groups_str) = line.split_once(' ').unwrap();
    let pattern = (0..5).map(|_| pattern_str).collect::<Vec<_>>().join("?");
    let single_groups: Vec<usize> = groups_str
        .split(',')
        .filter_map(|s| s.parse::<usize>().ok())
        .collect();
    let groups: Vec<usize> = (0..5).flat_map(|_| single_groups.clone()).collect();
    let mut memo = HashMap::new();
    count_arrangements(pattern.as_bytes(), &groups, &mut memo)
}

impl AocSolution for Day12 {
    fn part1(&self, input: &str) -> String {
        let sum: u64 = input
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(solve_line_part1)
            .sum();
        sum.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let sum: u64 = input
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(solve_line_part2)
            .sum();
        sum.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

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
    fn test_part2_full() {
        let input = crate::get_input_for_day(2023, 12).expect("Failed to get input");
        assert_eq!(Day12.part2(&input), "6512849198636");
    }
}
