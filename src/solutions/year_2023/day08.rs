use crate::AocSolution;
use std::{collections::HashMap, str::FromStr};

pub struct Day08;

#[derive(Debug, Clone, Copy)]
enum Direction {
    L,
    R,
}

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'L' => Ok(Direction::L),
            'R' => Ok(Direction::R),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Map {
    instructions: Vec<Direction>,
    nodes: HashMap<String, (String, String)>,
}

impl FromStr for Map {
    type Err = ();

    fn from_str(val: &str) -> Result<Map, Self::Err> {
        let lines: Vec<&str> = val.lines().filter(|l| !l.trim().is_empty()).collect();
        let instructions = lines[0]
            .trim()
            .chars()
            .map(|c| Direction::try_from(c).unwrap())
            .collect();

        let nodes = lines[1..]
            .iter()
            .map(|line| {
                let (input, output) = line.split_once(" = ").unwrap();
                let output = output.trim();
                let output = &output[1..output.len() - 1];
                let (left_output, right_output) = output.split_once(", ").unwrap();
                (
                    input.trim().to_string(),
                    (
                        left_output.trim().to_string(),
                        right_output.trim().to_string(),
                    ),
                )
            })
            .collect();

        Ok(Map {
            instructions,
            nodes,
        })
    }
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a / gcd(a, b)) * b
    }
}

impl Map {
    fn count_steps(&self) -> u64 {
        let start = "AAA";
        let end = "ZZZ";
        let mut position = start;
        let mut steps = 0;
        for instruction in self.instructions.iter().cycle() {
            if position == end {
                break;
            }
            position = match instruction {
                Direction::L => &self.nodes[position].0,
                Direction::R => &self.nodes[position].1,
            };
            steps += 1;
        }
        steps
    }

    fn count_steps_part2(&self) -> u64 {
        let starts: Vec<&str> = self
            .nodes
            .keys()
            .filter(|key| key.ends_with('A'))
            .map(|s| s.as_str())
            .collect();

        let mut cycle_lengths = Vec::new();
        for start in starts {
            let mut pos = start;
            let mut steps = 0;
            for instruction in self.instructions.iter().cycle() {
                if pos.ends_with('Z') {
                    break;
                }
                pos = match instruction {
                    Direction::L => &self.nodes[pos].0,
                    Direction::R => &self.nodes[pos].1,
                };
                steps += 1;
            }
            cycle_lengths.push(steps);
        }

        cycle_lengths.into_iter().fold(1, lcm)
    }
}

impl AocSolution for Day08 {
    fn part1(&self, input: &str) -> String {
        let map: Map = input.parse().unwrap();
        map.count_steps().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let map: Map = input.parse().unwrap();
        map.count_steps_part2().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_A: &str = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;

    const EXAMPLE_B: &str = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;

    #[test]
    fn test_part1_example() {
        assert_eq!(Day08.part1(EXAMPLE_A), "6");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2023, 8).expect("Failed to get input");
        assert_eq!(Day08.part1(&input), "16343");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(Day08.part2(EXAMPLE_B), "6");
    }

    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2023, 8).expect("Failed to get input");
        assert_eq!(Day08.part2(&input), "15299095336639");
    }
}
