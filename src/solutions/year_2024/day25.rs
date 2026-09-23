use crate::AocSolution;
use std::str::FromStr;

pub struct Day25;

#[derive(Debug)]
struct Lock {
    heights: [usize; 5],
}

#[derive(Debug)]
struct Key {
    heights: [usize; 5],
}

impl Lock {
    fn overlap(&self, key: &Key) -> bool {
        self.heights
            .iter()
            .zip(key.heights.iter())
            .any(|(l, k)| l + k > 5)
    }
}

#[derive(Debug)]
struct Puzzle {
    locks: Vec<Lock>,
    keys: Vec<Key>,
}

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut locks = Vec::new();
        let mut keys = Vec::new();
        for block in s.replace("\r\n", "\n").split("\n\n") {
            let lines: Vec<&str> = block.lines().filter(|l| !l.trim().is_empty()).collect();
            if lines.is_empty() {
                continue;
            }
            let first_char = lines[0].chars().next().unwrap();
            let mut heights = [0; 5];
            for line in &lines {
                for (i, char) in line.chars().enumerate() {
                    if char == '#' {
                        heights[i] += 1;
                    }
                }
            }
            for h in &mut heights {
                *h -= 1;
            }
            match first_char {
                '#' => locks.push(Lock { heights }),
                '.' => keys.push(Key { heights }),
                _ => panic!("invalid block marker"),
            }
        }
        Ok(Puzzle { locks, keys })
    }
}

impl Puzzle {
    fn process(&self) -> usize {
        let mut out = 0;
        for lock in &self.locks {
            for key in &self.keys {
                if !lock.overlap(key) {
                    out += 1;
                }
            }
        }
        out
    }
}

impl AocSolution for Day25 {
    fn part1(&self, input: &str) -> String {
        let puzzle: Puzzle = input.parse().unwrap();
        puzzle.process().to_string()
    }

    fn part2(&self, _input: &str) -> String {
        "Merry Christmas!".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####"#;

    #[test]
    fn test_part1_example() {
        assert_eq!(Day25.part1(EXAMPLE), "3");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2024, 25).expect("Failed to get input");
        assert_eq!(Day25.part1(&input), "2854");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day25.part2(""), "Merry Christmas!");
    }
}
