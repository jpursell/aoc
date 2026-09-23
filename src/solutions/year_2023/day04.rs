use crate::AocSolution;
use std::collections::HashSet;

pub struct Day04;

fn process_4a_line(line: &str) -> u32 {
    let (winners, nums) = line.split_once(" | ").unwrap();
    let mut set = HashSet::new();
    let (_, winners) = winners.split_once(": ").unwrap();
    for winner in winners.split_whitespace() {
        set.insert(winner.trim());
    }
    let mut score = 0_u32;
    for num in nums.split_whitespace() {
        if set.contains(num.trim()) {
            if score == 0 {
                score = 1;
            } else {
                score *= 2;
            }
        }
    }
    score
}

fn day_4a(input: &str) -> u32 {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(process_4a_line)
        .sum()
}

fn process_4b_line(line: &str) -> u32 {
    let (winners, nums) = line.split_once(" | ").unwrap();
    let mut set = HashSet::new();
    let (_, winners) = winners.split_once(": ").unwrap();
    for winner in winners.split_whitespace() {
        set.insert(winner.trim());
    }
    let mut matches = 0_u32;
    for num in nums.split_whitespace() {
        if set.contains(num.trim()) {
            matches += 1;
        }
    }
    matches
}

fn day_4b(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().filter(|l| !l.trim().is_empty()).collect();
    let mut card_counts = vec![1_u32; lines.len()];
    for (i, line) in lines.iter().enumerate() {
        let matches = process_4b_line(line) as usize;
        let count = card_counts[i];
        for j in (i + 1)..=(i + matches).min(lines.len() - 1) {
            card_counts[j] += count;
        }
    }
    card_counts.iter().sum()
}

impl AocSolution for Day04 {
    fn part1(&self, input: &str) -> String {
        day_4a(input).to_string()
    }

    fn part2(&self, input: &str) -> String {
        day_4b(input).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

    #[test]
    fn test_part1_example() {
        assert_eq!(Day04.part1(EXAMPLE), "13");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2023, 4).expect("Failed to get input");
        assert_eq!(Day04.part1(&input), "25174");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(Day04.part2(EXAMPLE), "30");
    }

    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2023, 4).expect("Failed to get input");
        assert_eq!(Day04.part2(&input), "6420979");
    }
}
