use crate::AocSolution;
use ndarray::prelude::*;
use std::str::FromStr;

pub struct Day20;

#[derive(Debug, Clone, Copy)]
enum Token {
    Wall,
    Track,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn position_from(&self, position: &[usize; 2]) -> Option<[usize; 2]> {
        if (matches!(self, Direction::Up) && position[0] == 0)
            || (matches!(self, Direction::Left) && position[1] == 0)
        {
            None
        } else {
            match self {
                Direction::Up => Some([position[0] - 1, position[1]]),
                Direction::Down => Some([position[0] + 1, position[1]]),
                Direction::Left => Some([position[0], position[1] - 1]),
                Direction::Right => Some([position[0], position[1] + 1]),
            }
        }
    }
}

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

#[derive(Debug)]
struct Puzzle {
    map: Array2<Token>,
    start: [usize; 2],
    end: [usize; 2],
    steps_map: Array2<Option<usize>>,
    positions: Vec<[usize; 2]>,
}

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nrows = s.lines().count();
        let ncols = s.lines().next().unwrap().chars().count();
        let mut map = Vec::with_capacity(nrows * ncols);
        let mut start = [0, 0];
        let mut end = [0, 0];
        for (irow, line) in s.lines().enumerate() {
            for (icol, c) in line.chars().enumerate() {
                map.push(match c {
                    '.' => Token::Track,
                    '#' => Token::Wall,
                    'S' => {
                        start = [irow, icol];
                        Token::Track
                    }
                    'E' => {
                        end = [irow, icol];
                        Token::Track
                    }
                    _ => panic!("unexpected character in puzzle map"),
                });
            }
        }
        let map = Array2::from_shape_vec((nrows, ncols), map).unwrap();
        let steps = Array2::from_elem(map.raw_dim(), None);
        Ok(Puzzle {
            map,
            start,
            end,
            steps_map: steps,
            positions: Vec::new(),
        })
    }
}

impl Puzzle {
    fn solve_steps(&mut self) {
        self.steps_map.iter_mut().for_each(|x| *x = None);
        self.steps_map[self.start] = Some(0);
        let mut pos = self.start;
        let mut steps = 0;
        self.positions.push(pos);
        while pos != self.end {
            pos = self.find_next_pos(&pos);
            self.positions.push(pos);
            steps += 1;
            self.steps_map[pos] = Some(steps);
        }
    }

    fn find_next_pos(&self, pos: &[usize; 2]) -> [usize; 2] {
        let mut next_pos = None;
        for next_direction in DIRECTIONS {
            let potential_next_pos = next_direction.position_from(pos);
            if potential_next_pos.is_none() {
                continue;
            }
            let potential_next_pos = potential_next_pos.unwrap();
            let next_token = self.map.get(potential_next_pos);
            if next_token.is_none() {
                continue;
            }
            if matches!(next_token.unwrap(), Token::Wall) {
                continue;
            }
            if self.steps_map[potential_next_pos].is_some() {
                continue;
            }
            if next_pos.is_some() {
                panic!("branching track not supported");
            }
            next_pos = Some(potential_next_pos);
        }
        next_pos.unwrap()
    }

    fn find_cheats_part1(&self) -> Vec<usize> {
        let mut cheats = Vec::new();
        for pos in &self.positions {
            for direction in DIRECTIONS {
                let cheat_pos = direction.position_from(pos);
                if cheat_pos.is_none() {
                    continue;
                }
                let cheat_pos = direction.position_from(&cheat_pos.unwrap());
                if cheat_pos.is_none() {
                    continue;
                }
                let cheat_steps = self.steps_map.get(cheat_pos.unwrap());
                if cheat_steps.is_none() || cheat_steps.unwrap().is_none() {
                    continue;
                }
                let cheat_steps = cheat_steps.unwrap().unwrap();
                let current_steps = self.steps_map[*pos].unwrap();
                if cheat_steps > current_steps + 2 {
                    let improvement = cheat_steps - (current_steps + 2);
                    cheats.push(improvement);
                }
            }
        }
        cheats
    }

    fn find_cheats_part2(&self, radius: i64) -> Vec<usize> {
        let mut cheats = Vec::new();
        for pos in &self.positions {
            for drow in -radius..=radius {
                let cheat_pos_row = pos[0] as i64 + drow;
                if cheat_pos_row < 0 {
                    continue;
                }
                for dcol in -radius..=radius {
                    let cheat_pos_col = pos[1] as i64 + dcol;
                    if cheat_pos_col < 0 {
                        continue;
                    }
                    let cheat_length = drow.abs() + dcol.abs();
                    if cheat_length > radius {
                        continue;
                    }
                    let cheat_length = cheat_length as usize;
                    let cheat_pos = [cheat_pos_row as usize, cheat_pos_col as usize];

                    let cheat_steps = self.steps_map.get(cheat_pos);
                    if cheat_steps.is_none() || cheat_steps.unwrap().is_none() {
                        continue;
                    }
                    let cheat_steps = cheat_steps.unwrap().unwrap();
                    let current_steps = self.steps_map[*pos].unwrap();
                    if cheat_steps > current_steps + cheat_length {
                        let improvement = cheat_steps - (current_steps + cheat_length);
                        cheats.push(improvement);
                    }
                }
            }
        }
        cheats
    }
}

impl AocSolution for Day20 {
    fn part1(&self, input: &str) -> String {
        let mut puzzle: Puzzle = input.parse().unwrap();
        let min_saving = if puzzle.map.shape()[0] < 50 { 20 } else { 100 };
        puzzle.solve_steps();
        let cheats = puzzle.find_cheats_part1();
        cheats
            .iter()
            .filter(|&x| *x >= min_saving)
            .count()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut puzzle: Puzzle = input.parse().unwrap();
        let min_saving = if puzzle.map.shape()[0] < 50 { 50 } else { 100 };
        puzzle.solve_steps();
        let cheats = puzzle.find_cheats_part2(20);
        cheats
            .iter()
            .filter(|&x| *x >= min_saving)
            .count()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"#;

    #[test]
    fn test_part1_example() {
        assert_eq!(Day20.part1(EXAMPLE), "5");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2024, 20).expect("Failed to get input");
        assert_eq!(Day20.part1(&input), "1346");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(Day20.part2(EXAMPLE), "285");
    }

    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2024, 20).expect("Failed to get input");
        assert_eq!(Day20.part2(&input), "985482");
    }
}
