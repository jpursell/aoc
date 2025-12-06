use crate::AocSolution;
use ndarray::prelude::*;
use std::{cmp::Ordering, collections::BTreeSet, str::FromStr};

pub struct Day16;

#[derive(Debug, Clone, Copy)]
enum Token {
    Wall,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<Direction> for usize {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => 0,
            Direction::Right => 1,
            Direction::Down => 2,
            Direction::Left => 3,
        }
    }
}
impl From<usize> for Direction {
    fn from(value: usize) -> Self {
        match value {
            0 => Direction::Up,
            1 => Direction::Right,
            2 => Direction::Down,
            3 => Direction::Left,
            _ => panic!(),
        }
    }
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

#[derive(Debug)]
struct Puzzle {
    map: Array2<Token>,
    start: [usize; 2],
    end: [usize; 2],
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
                    '.' => Token::None,
                    '#' => Token::Wall,
                    'S' => {
                        start = [irow, icol];
                        Token::None
                    }
                    'E' => {
                        end = [irow, icol];
                        Token::None
                    }
                    _ => panic!(),
                });
            }
        }
        let map = Array2::from_shape_vec((nrows, ncols), map).unwrap();
        Ok(Puzzle { map, start, end })
    }
}

const DIRECTIONS_ALL: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

#[derive(Debug, PartialEq, Clone, Copy)]
enum Move {
    Forward,
    Clockwise,
    CounterClockwise,
}

impl Move {
    fn position_from(&self, position: &[usize; 3]) -> Option<[usize; 3]> {
        match self {
            Move::Forward => {
                let direction = Direction::from(position[2]);
                if (matches!(direction, Direction::Up) && position[0] == 0)
                    || (matches!(direction, Direction::Left) && position[1] == 0)
                {
                    None
                } else {
                    match direction {
                        Direction::Up => Some([position[0] - 1, position[1], position[2]]),
                        Direction::Down => Some([position[0] + 1, position[1], position[2]]),
                        Direction::Left => Some([position[0], position[1] - 1, position[2]]),
                        Direction::Right => Some([position[0], position[1] + 1, position[2]]),
                    }
                }
            }
            Move::Clockwise => Some([position[0], position[1], (position[2] + 1) % 4]),
            Move::CounterClockwise => {
                if position[2] == 0 {
                    Some([position[0], position[1], 3])
                } else {
                    Some([position[0], position[1], position[2] - 1])
                }
            }
        }
    }
}

const MOVES: [Move; 3] = [Move::Clockwise, Move::CounterClockwise, Move::Forward];

struct StatePart1 {
    scores: Array2<Option<usize>>,
    directions: Array2<Option<Direction>>,
    steps: usize,
}

struct StatePart2 {
    scores: Array3<Option<usize>>,
    previous: Array3<Vec<[usize; 3]>>,
}

impl Puzzle {
    fn process_part1(&self) -> usize {
        let mut state = StatePart1 {
            scores: Array2::<Option<usize>>::from_elem(self.map.raw_dim(), None),
            directions: Array2::<Option<Direction>>::from_elem(self.map.raw_dim(), None),
            steps: 0,
        };
        state.scores[self.start] = Some(0);
        state.directions[self.start] = Some(Direction::Right);
        let mut ends = [vec![self.start], Vec::new()];
        loop {
            let (e0, e1) = ends.split_at_mut(1);
            let (current, next) = if state.steps.is_multiple_of(2) {
                (&e0[0], &mut e1[0])
            } else {
                (&e1[0], &mut e0[0])
            };
            next.clear();
            if current.is_empty() {
                break;
            }
            for pos in current {
                next.append(&mut self.process_pos_part1(pos, &mut state));
            }
            state.steps += 1;
        }
        state.scores[self.end].unwrap()
    }
    fn process_pos_part1(&self, pos: &[usize; 2], state: &mut StatePart1) -> Vec<[usize; 2]> {
        let mut next_ends = Vec::new();
        for next_direction in self.find_possible_dirs_part1(pos) {
            let next_position = next_direction.position_from(pos).unwrap();
            let next_score = if next_direction == state.directions[*pos].unwrap() {
                state.scores[*pos].unwrap() + 1
            } else {
                state.scores[*pos].unwrap() + 1001
            };
            if let Some(score) = state.scores[next_position] {
                if next_score < score {
                    next_ends.push(next_position);
                    state.scores[next_position] = Some(next_score);
                    state.directions[next_position] = Some(next_direction);
                }
            } else {
                next_ends.push(next_position);
                state.scores[next_position] = Some(next_score);
                state.directions[next_position] = Some(next_direction);
            }
        }
        next_ends
    }
    fn find_possible_dirs_part1(&self, pos: &[usize; 2]) -> Vec<Direction> {
        let mut possible_directions = Vec::new();
        for next_direction in DIRECTIONS_ALL {
            if let Some(next_position) = next_direction.position_from(pos) {
                if let Some(token) = self.map.get(next_position) {
                    if !matches!(token, Token::Wall) {
                        possible_directions.push(next_direction);
                    }
                }
            }
        }
        possible_directions
    }

    fn process_part2(&self) -> usize {
        let map_shape = self.map.shape();
        let records_shape = [map_shape[0], map_shape[1], 4];
        let mut state = StatePart2 {
            scores: Array3::<Option<usize>>::from_elem(records_shape, None),
            previous: Array3::<Vec<[usize; 3]>>::from_elem(records_shape, Vec::with_capacity(3)),
        };
        let start = [self.start[0], self.start[1], Direction::Right.into()];
        state.scores[start] = Some(0);
        let mut ends = BTreeSet::from([start]);
        while !ends.is_empty() {
            let pos = ends.pop_first().unwrap();
            for next_pos in self.process_pos_part2(pos, &mut state) {
                ends.insert(next_pos);
            }
        }
        self.trace_back_part2(&state, &self.best_end_position_part2(&state).unwrap())
            .iter()
            .map(|x| [x[0], x[1]])
            .collect::<BTreeSet<[usize; 2]>>()
            .len()
    }
    fn best_end_position_part2(&self, state: &StatePart2) -> Option<[usize; 3]> {
        let mut lowest_score = None;
        let mut best_position = None;
        for i in 0..4 {
            let pos = [self.end[0], self.end[1], i];
            if let Some(score) = state.scores[pos] {
                if let Some(best) = lowest_score {
                    if score < best {
                        lowest_score = Some(score);
                        best_position = Some(pos);
                    }
                } else {
                    lowest_score = Some(score);
                    best_position = Some(pos);
                }
            }
        }
        best_position
    }
    fn trace_back_part2(&self, state: &StatePart2, pos: &[usize; 3]) -> BTreeSet<[usize; 3]> {
        let mut out = BTreeSet::from([*pos]);
        if pos[0] == self.start[0] && pos[1] == self.start[1] {
            return out;
        }
        for previous in &state.previous[*pos] {
            out.append(&mut self.trace_back_part2(state, previous));
        }
        out
    }
    fn process_pos_part2(&self, pos: [usize; 3], state: &mut StatePart2) -> Vec<[usize; 3]> {
        let mut next_ends = Vec::new();
        for (next_move, next_position) in self.find_possible_next_positions_part2(&pos) {
            let next_score = if matches!(next_move, Move::Forward) {
                state.scores[pos].unwrap() + 1
            } else {
                state.scores[pos].unwrap() + 1000
            };
            if let Some(existing_score) = state.scores[next_position] {
                match next_score.cmp(&existing_score) {
                    Ordering::Less => {
                        next_ends.push(next_position);
                        state.scores[next_position] = Some(next_score);
                        state.previous[next_position].clear();
                        state.previous[next_position].push(pos);
                    }
                    Ordering::Equal => {
                        state.previous[next_position].push(pos);
                    }
                    Ordering::Greater => {}
                }
            } else {
                next_ends.push(next_position);
                state.scores[next_position] = Some(next_score);
                state.previous[next_position].push(pos);
            }
        }
        next_ends
    }
    fn find_possible_next_positions_part2(&self, pos: &[usize; 3]) -> Vec<(Move, [usize; 3])> {
        let mut possible_positions = Vec::new();
        for next_move in MOVES {
            if let Some(next_position) = next_move.position_from(pos) {
                let next_position_2d = [next_position[0], next_position[1]];
                if let Some(token) = self.map.get(next_position_2d) {
                    if !matches!(token, Token::Wall) {
                        possible_positions.push((next_move, next_position));
                    }
                }
            }
        }
        possible_positions
    }
}

impl AocSolution for Day16 {
    fn part1(&self, input: &str) -> String {
        let puzzle: Puzzle = input.parse().unwrap();
        puzzle.process_part1().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let puzzle: Puzzle = input.parse().unwrap();
        puzzle.process_part2().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_A: &str = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#;
    const EXAMPLE_B: &str = r#"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"#;

    #[test]
    fn test_part1_a() {
        assert_eq!(Day16.part1(EXAMPLE_A), "7036");
    }

    #[test]
    fn test_part1_b() {
        assert_eq!(Day16.part1(EXAMPLE_B), "11048");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2024, 16).expect("Failed to get input");
        assert_eq!(Day16.part1(&input), "66404");
    }

    #[test]
    fn test_part2_a() {
        assert_eq!(Day16.part2(EXAMPLE_A), "45");
    }

    #[test]
    fn test_part2_b() {
        assert_eq!(Day16.part2(EXAMPLE_B), "64");
    }

    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2024, 16).expect("Failed to get input");
        assert_eq!(Day16.part2(&input), "433");
    }
}
