use crate::AocSolution;
use std::{collections::HashSet, str::FromStr};

pub struct Day19;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Token {
    White,
    Blue,
    Black,
    Red,
    Green,
}

impl From<char> for Token {
    fn from(value: char) -> Self {
        match value {
            'w' => Token::White,
            'u' => Token::Blue,
            'b' => Token::Black,
            'r' => Token::Red,
            'g' => Token::Green,
            _ => panic!("invalid token"),
        }
    }
}

#[derive(Debug)]
struct Puzzle {
    available: HashSet<Vec<Token>>,
    max_available_len: usize,
    needed: Vec<Vec<Token>>,
}

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        let available: HashSet<Vec<Token>> = lines[0]
            .split(", ")
            .map(|s| s.chars().map(Token::from).collect())
            .collect();
        let max_available_len = available
            .iter()
            .map(|a| a.len())
            .fold(0, |a, len| a.max(len));
        let needed = lines[2..]
            .iter()
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.trim().chars().map(Token::from).collect())
            .collect();
        Ok(Puzzle {
            available,
            max_available_len,
            needed,
        })
    }
}

impl Puzzle {
    fn count_solutions(&self, needed: &[Token]) -> usize {
        let mut count_map = vec![0_usize; needed.len() + 1];
        count_map[0] = 1;
        for i in 0..needed.len() {
            let count = count_map[i];
            if count == 0 {
                continue;
            }
            for len in 1..=self.max_available_len {
                if i + len > needed.len() {
                    continue;
                }
                if !self.available.contains(&needed[i..i + len]) {
                    continue;
                }
                count_map[i + len] += count;
            }
        }
        *count_map.last().unwrap()
    }

    fn part1(&self) -> usize {
        self.needed
            .iter()
            .filter(|needed| self.count_solutions(needed) > 0)
            .count()
    }

    fn part2(&self) -> usize {
        self.needed
            .iter()
            .map(|needed| self.count_solutions(needed))
            .sum()
    }
}

impl AocSolution for Day19 {
    fn part1(&self, input: &str) -> String {
        let puzzle: Puzzle = input.parse().unwrap();
        puzzle.part1().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let puzzle: Puzzle = input.parse().unwrap();
        puzzle.part2().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"#;

    #[test]
    fn test_part1_example() {
        assert_eq!(Day19.part1(EXAMPLE), "6");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2024, 19).expect("Failed to get input");
        assert_eq!(Day19.part1(&input), "360");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(Day19.part2(EXAMPLE), "16");
    }

    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2024, 19).expect("Failed to get input");
        assert_eq!(Day19.part2(&input), "577474410989846");
    }
}
