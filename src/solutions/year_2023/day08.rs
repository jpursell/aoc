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

#[derive(Debug)]
struct History<'a> {
    nodes: Vec<&'a String>,
    instruction_len: usize,
    complete: bool,
}

impl<'a> History<'a> {
    fn new(instruction_len: usize) -> Self {
        History {
            nodes: Vec::new(),
            instruction_len,
            complete: false,
        }
    }
    fn from_map(map: &Map) -> Self {
        Self::new(map.instructions.len())
    }
    fn push(&mut self, node: &'a String) {
        self.nodes.push(node);
        self.update();
    }
    fn check(&self) -> bool {
        self.complete
    }
    fn cycle_len(&self) -> usize {
        self.nodes.len() - self.cycle_start() - 1
    }
    fn cycle_start(&self) -> usize {
        (self.nodes.len() - 1) % self.instruction_len
    }
    fn update(&mut self) {
        if self.nodes.len() < self.instruction_len + 1 {
            return;
        }
        let pos = self.cycle_start();
        let old_node = self.nodes[pos];
        let latest_node = *self.nodes.last().unwrap();
        let ret = old_node == latest_node;
        self.complete = ret;
    }
}

#[derive(Debug)]
struct Historian {
    pos: usize,
    cycle: usize,
    cycle_len: usize,
    end_pos: Vec<usize>,
}

impl Historian {
    fn new(history: History) -> Self {
        assert!(history.check());
        let cycle_len = history.cycle_len();
        let end_pos = history
            .nodes
            .iter()
            .enumerate()
            .filter(|(_num, key)| key.ends_with('Z'))
            .map(|(num, _key)| num)
            .collect::<Vec<usize>>();
        Historian {
            pos: 0,
            cycle: 0,
            cycle_len,
            end_pos,
        }
    }
}

impl<'a> TryFrom<History<'a>> for Historian {
    type Error = &'static str;
    fn try_from(hist: History<'a>) -> Result<Self, <Self as TryFrom<History<'a>>>::Error> {
        if !hist.check() {
            Err("Incomplete history")
        } else {
            Ok(Historian::new(hist))
        }
    }
}

impl Iterator for Historian {
    type Item = usize;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let ret = Some(self.end_pos[self.pos] + self.cycle_len * self.cycle);
        self.pos += 1;
        if self.pos == self.end_pos.len() {
            self.pos = 0;
            self.cycle += 1;
        }
        ret
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

    fn get_key_history<'a>(&'a self, key: &'a String) -> Historian {
        let mut pos = key;
        let mut history = History::from_map(self);
        history.push(pos);
        for instruction in self.instructions.iter().cycle() {
            pos = match instruction {
                Direction::L => &self.nodes[pos].0,
                Direction::R => &self.nodes[pos].1,
            };
            history.push(pos);
            if history.check() {
                break;
            }
        }
        Historian::try_from(history).unwrap()
    }

    fn count_steps_2(&self) -> u64 {
        let mut history = self
            .nodes
            .keys()
            .filter(|key| key.ends_with('A'))
            .map(|key| self.get_key_history(key))
            .collect::<Vec<_>>();
        let mut pos = history
            .iter_mut()
            .map(|h| h.next().unwrap())
            .collect::<Vec<_>>();
        loop {
            let max_pos = *pos.iter().max().unwrap();
            if pos.iter().all(|x| *x == max_pos) {
                break;
            }
            for (num, hist) in history.iter_mut().enumerate() {
                while pos[num] < max_pos {
                    pos[num] = hist.next().unwrap();
                }
            }
        }
        pos.into_iter().max().unwrap() as u64
    }
}

impl AocSolution for Day08 {
    fn part1(&self, input: &str) -> String {
        let map: Map = input.parse().unwrap();
        map.count_steps().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let map: Map = input.parse().unwrap();
        map.count_steps_2().to_string()
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
    #[ignore = "slow/infinite in aoc23"]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2023, 8).expect("Failed to get input");
        assert_eq!(Day08.part2(&input), "15299095336639");
    }
}
