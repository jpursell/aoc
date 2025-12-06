use crate::AocSolution;

#[derive(Debug)]
enum Op {
    Plus,
    Times,
}
fn solve_worksheet(input: &str) -> u64 {
    let nlines = input.lines().count();
    let numbers: Vec<Vec<u64>> = input
        .lines()
        .take(nlines - 1)
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect();
    let ops: Vec<Op> = input
        .lines()
        .next_back()
        .unwrap()
        .split_whitespace()
        .map(|s| match s {
            "*" => Op::Times,
            "+" => Op::Plus,
            _ => panic!("Got unknown operator"),
        })
        .collect();
    ops.iter()
        .enumerate()
        .map(|(i, op)| {
            numbers.iter().fold(
                match op {
                    Op::Plus => 0,
                    Op::Times => 1,
                },
                |acc, val| match op {
                    Op::Plus => acc + val[i],
                    Op::Times => acc * val[i],
                },
            )
        })
        .sum()
}

pub struct Day06;

impl AocSolution for Day06 {
    fn part1(&self, input: &str) -> String {
        solve_worksheet(input).to_string()
    }

    fn part2(&self, _input: &str) -> String {
        "Not implemented".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_part1_example() {
        assert_eq!(Day06.part1(EXAMPLE), "4277556");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2025, 6).expect("Failed to get input");
        assert_eq!(Day06.part1(&input), "6891729672676");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(Day06.part2(EXAMPLE), "REPLACE_WITH_PART2_EXAMPLE_RESULT");
    }

    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2025, 6).expect("Failed to get input");
        assert_eq!(Day06.part2(&input), "REPLACE_WITH_PART2_FULL_RESULT");
    }
}
