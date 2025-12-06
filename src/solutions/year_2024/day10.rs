use crate::AocSolution;
use ndarray::prelude::*;
use std::{collections::BTreeSet, str::FromStr};

pub struct Day10;

#[derive(Debug)]
struct Puzzle {
    map: Array2<usize>,
}

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nrows = s.lines().count();
        let ncols = s.lines().next().unwrap().chars().count();
        let mut map = Vec::with_capacity(nrows * ncols);
        for line in s.lines() {
            for c in line.chars() {
                map.push(c.to_digit(10).unwrap() as usize);
            }
        }
        let map = Array2::from_shape_vec((nrows, ncols), map).unwrap();
        Ok(Puzzle { map })
    }
}

impl Puzzle {
    fn find_trail_ends(
        &self,
        pos: [usize; 2],
        value: usize,
        walked: &mut Array2<bool>,
    ) -> BTreeSet<[usize; 2]> {
        if value == 9 {
            return BTreeSet::from([pos]);
        }
        let mut ends = BTreeSet::new();
        let mut check_new_pos = |new_pos| {
            if !walked[new_pos] {
                let new_value = self.map[new_pos];
                if new_value == value + 1 {
                    walked[new_pos] = true;
                    ends.append(&mut self.find_trail_ends(new_pos, new_value, walked));
                    walked[new_pos] = false;
                }
            }
        };
        if pos[0] + 1 < self.map.shape()[0] {
            let new_pos = [pos[0] + 1, pos[1]];
            check_new_pos(new_pos);
        }
        if pos[0] > 0 {
            let new_pos = [pos[0] - 1, pos[1]];
            check_new_pos(new_pos);
        }
        if pos[1] + 1 < self.map.shape()[1] {
            let new_pos = [pos[0], pos[1] + 1];
            check_new_pos(new_pos);
        }
        if pos[1] > 0 {
            let new_pos = [pos[0], pos[1] - 1];
            check_new_pos(new_pos);
        }
        ends
    }

    fn count_trails(&self, pos: [usize; 2], value: usize, walked: &mut Array2<bool>) -> usize {
        if value == 9 {
            return 1;
        }
        let mut count = 0;
        let mut check_new_pos = |new_pos| {
            if !walked[new_pos] {
                let new_value = self.map[new_pos];
                if new_value == value + 1 {
                    walked[new_pos] = true;
                    count += self.count_trails(new_pos, new_value, walked);
                    walked[new_pos] = false;
                }
            }
        };
        if pos[0] + 1 < self.map.shape()[0] {
            let new_pos = [pos[0] + 1, pos[1]];
            check_new_pos(new_pos);
        }
        if pos[0] > 0 {
            let new_pos = [pos[0] - 1, pos[1]];
            check_new_pos(new_pos);
        }
        if pos[1] + 1 < self.map.shape()[1] {
            let new_pos = [pos[0], pos[1] + 1];
            check_new_pos(new_pos);
        }
        if pos[1] > 0 {
            let new_pos = [pos[0], pos[1] - 1];
            check_new_pos(new_pos);
        }
        count
    }
}

impl AocSolution for Day10 {
    fn part1(&self, input: &str) -> String {
        let puzzle: Puzzle = input.parse().unwrap();
        let mut walked = Array2::from_elem(puzzle.map.raw_dim(), false);
        puzzle
            .map
            .indexed_iter()
            .map(|(pos, start)| {
                if start != &0 {
                    0
                } else {
                    puzzle
                        .find_trail_ends([pos.0, pos.1], *start, &mut walked)
                        .len()
                }
            })
            .sum::<usize>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let puzzle: Puzzle = input.parse().unwrap();
        let mut walked = Array2::from_elem(puzzle.map.raw_dim(), false);
        puzzle
            .map
            .indexed_iter()
            .map(|(pos, start)| {
                if start != &0 {
                    0
                } else {
                    puzzle.count_trails([pos.0, pos.1], *start, &mut walked)
                }
            })
            .sum::<usize>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
"#;

    #[test]
    fn test_part1_example() {
        assert_eq!(Day10.part1(EXAMPLE), "36");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2024, 10).expect("Failed to get input");
        assert_eq!(Day10.part1(&input), "531");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(Day10.part2(EXAMPLE), "81");
    }

    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2024, 10).expect("Failed to get input");
        assert_eq!(Day10.part2(&input), "1210");
    }
}
