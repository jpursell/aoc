use crate::AocSolution;
use std::{
    collections::{hash_map::Entry, HashMap, VecDeque},
    str::FromStr,
};

pub struct Day22;

#[derive(Debug, Clone)]
struct Secret {
    value: usize,
}

impl Secret {
    fn mix(&mut self, num: usize) {
        self.value ^= num;
    }

    fn prune(&mut self) {
        self.value %= 16777216;
    }

    fn evolve(&mut self) {
        self.mix(self.value * 64);
        self.prune();
        self.mix(self.value / 32);
        self.prune();
        self.mix(self.value * 2048);
        self.prune();
    }

    fn make_sequence(&mut self, steps: usize) -> HashMap<[i8; 4], i8> {
        let mut out = HashMap::new();
        let mut prev = (self.value % 10) as i8;
        let mut deltas = VecDeque::new();
        for _ in 0..steps {
            self.evolve();
            let current = (self.value % 10) as i8;
            deltas.push_back(current - prev);
            if deltas.len() > 4 {
                deltas.pop_front();
            }
            prev = current;
            if deltas.len() < 4 {
                continue;
            }
            let key = [deltas[0], deltas[1], deltas[2], deltas[3]];
            match out.entry(key) {
                Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(current);
                }
                Entry::Occupied(_) => (),
            }
        }
        out
    }
}

#[derive(Debug, Clone)]
struct Puzzle {
    secrets: Vec<Secret>,
}

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let secrets = s
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| Secret {
                value: line.trim().parse::<usize>().unwrap(),
            })
            .collect();
        Ok(Puzzle { secrets })
    }
}

impl Puzzle {
    fn process_part1(&mut self, steps: usize) -> usize {
        for secret in &mut self.secrets {
            for _ in 0..steps {
                secret.evolve();
            }
        }
        self.secrets.iter().map(|x| x.value).sum()
    }

    fn process_part2(&mut self, steps: usize) -> usize {
        let mut total_bananas: HashMap<[i8; 4], usize> = HashMap::new();
        for secret in &mut self.secrets {
            let seq = secret.make_sequence(steps);
            for (key, val) in seq {
                *total_bananas.entry(key).or_default() += val as usize;
            }
        }
        total_bananas.values().copied().max().unwrap_or(0)
    }
}

impl AocSolution for Day22 {
    fn part1(&self, input: &str) -> String {
        let mut puzzle: Puzzle = input.parse().unwrap();
        puzzle.process_part1(2000).to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut puzzle: Puzzle = input.parse().unwrap();
        puzzle.process_part2(2000).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_A: &str = r#"1
10
100
2024"#;

    const EXAMPLE_B: &str = r#"1
2
3
2024"#;

    #[test]
    fn test_part1_example() {
        assert_eq!(Day22.part1(EXAMPLE_A), "37327623");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2024, 22).expect("Failed to get input");
        assert_eq!(Day22.part1(&input), "18525593556");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(Day22.part2(EXAMPLE_B), "23");
    }

    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2024, 22).expect("Failed to get input");
        assert_eq!(Day22.part2(&input), "2089");
    }
}
