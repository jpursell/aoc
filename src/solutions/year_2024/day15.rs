use crate::AocSolution;
use ndarray::prelude::*;
use std::str::FromStr;

pub struct Day15;

// Common Direction enum
#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn position_from(&self, position: &[usize; 2]) -> Option<[usize; 2]> {
        if matches!(self, Direction::Up) && position[0] == 0
            || matches!(self, Direction::Left) && position[1] == 0
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

// Part 1 specific code
#[derive(Debug, Clone, Copy)]
enum TokenPart1 {
    Wall,
    Box,
    None,
}

#[derive(Debug)]
struct PuzzlePart1 {
    map: Array2<TokenPart1>,
    robot: [usize; 2],
    directions: Vec<Direction>,
}

impl FromStr for PuzzlePart1 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (map_lines, direction_lines) = {
            let mut map_lines = Vec::new();
            let mut direction_lines = Vec::new();
            let mut getting_map = true;
            for line in s.lines() {
                if line.is_empty() {
                    getting_map = false;
                    continue;
                }
                if getting_map {
                    map_lines.push(line);
                } else {
                    direction_lines.push(line);
                }
            }
            (map_lines, direction_lines)
        };

        let (map, robot) = {
            let nrows = map_lines.len();
            let ncols = map_lines[0].len();
            let mut map = Vec::with_capacity(nrows * ncols);
            let mut robot = [0, 0];
            for (irow, line) in map_lines.iter().enumerate() {
                for (icol, c) in line.chars().enumerate() {
                    map.push(match c {
                        '#' => TokenPart1::Wall,
                        'O' => TokenPart1::Box,
                        '.' => TokenPart1::None,
                        '@' => {
                            robot = [irow, icol];
                            TokenPart1::None
                        }
                        _ => panic!(),
                    });
                }
            }
            (Array2::from_shape_vec((nrows, ncols), map).unwrap(), robot)
        };

        let mut directions = Vec::new();
        for line in direction_lines {
            for c in line.chars() {
                directions.push(match c {
                    '>' => Direction::Right,
                    '<' => Direction::Left,
                    '^' => Direction::Up,
                    'v' => Direction::Down,
                    _ => panic!(),
                });
            }
        }

        Ok(PuzzlePart1 {
            map,
            robot,
            directions,
        })
    }
}

impl PuzzlePart1 {
    fn process(&self) -> usize {
        let mut map = self.map.to_owned();
        let mut robot = self.robot;
        for direction in &self.directions {
            if PuzzlePart1::can_move(map.view(), &robot, direction) {
                PuzzlePart1::do_move(map.view_mut(), &robot, direction);
                robot = direction.position_from(&robot).unwrap();
            }
        }
        map.indexed_iter()
            .map(|(pos, x)| match x {
                TokenPart1::Box => pos.0 * 100 + pos.1,
                _ => 0,
            })
            .sum()
    }

    fn can_move(map: ArrayView2<TokenPart1>, position: &[usize; 2], direction: &Direction) -> bool {
        let new_position = direction.position_from(position).unwrap();
        match map.get(new_position) {
            Some(TokenPart1::None) => true,
            Some(TokenPart1::Wall) => false,
            Some(TokenPart1::Box) => PuzzlePart1::can_move(map, &new_position, direction),
            None => false,
        }
    }

    fn do_move(mut map: ArrayViewMut2<TokenPart1>, position: &[usize; 2], direction: &Direction) {
        let new_position = direction.position_from(position).unwrap();
        if let Some(&TokenPart1::Box) = map.get(new_position) {
            PuzzlePart1::do_move(map.view_mut(), &new_position, direction);
        }
        if let Some(&TokenPart1::Box) = map.get(*position) {
            map[new_position] = TokenPart1::Box;
            map[*position] = TokenPart1::None;
        }
    }
}

// Part 2 specific code
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TokenPart2 {
    Wall,
    BoxLeft,
    BoxRight,
    None,
}

#[derive(Debug)]
struct PuzzlePart2 {
    map: Array2<TokenPart2>,
    robot: [usize; 2],
    directions: Vec<Direction>,
}

impl FromStr for PuzzlePart2 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (map_lines, direction_lines) = {
            let mut map_lines = Vec::new();
            let mut direction_lines = Vec::new();
            let mut getting_map = true;
            for line in s.lines() {
                if line.is_empty() {
                    getting_map = false;
                    continue;
                }
                if getting_map {
                    map_lines.push(line);
                } else {
                    direction_lines.push(line);
                }
            }
            (map_lines, direction_lines)
        };

        let (map, robot) = {
            let nrows = map_lines.len();
            let ncols = map_lines[0].len() * 2;
            let mut map = Vec::with_capacity(nrows * ncols);
            let mut robot = [0, 0];
            for (irow, line) in map_lines.iter().enumerate() {
                for (icol, c) in line.chars().enumerate() {
                    match c {
                        '#' => {
                            map.push(TokenPart2::Wall);
                            map.push(TokenPart2::Wall);
                        }
                        'O' => {
                            map.push(TokenPart2::BoxLeft);
                            map.push(TokenPart2::BoxRight);
                        }
                        '.' => {
                            map.push(TokenPart2::None);
                            map.push(TokenPart2::None);
                        }
                        '@' => {
                            robot = [irow, icol * 2];
                            map.push(TokenPart2::None);
                            map.push(TokenPart2::None);
                        }
                        _ => panic!(),
                    }
                }
            }
            (Array2::from_shape_vec((nrows, ncols), map).unwrap(), robot)
        };

        let mut directions = Vec::new();
        for line in direction_lines {
            for c in line.chars() {
                directions.push(match c {
                    '>' => Direction::Right,
                    '<' => Direction::Left,
                    '^' => Direction::Up,
                    'v' => Direction::Down,
                    _ => panic!(),
                });
            }
        }

        Ok(PuzzlePart2 {
            map,
            robot,
            directions,
        })
    }
}

impl PuzzlePart2 {
    fn process(&self) -> usize {
        let mut map = self.map.to_owned();
        let mut robot = self.robot;
        for direction in &self.directions {
            if PuzzlePart2::can_move(map.view(), &robot, direction) {
                PuzzlePart2::do_move(map.view_mut(), &robot, direction);
                robot = direction.position_from(&robot).unwrap();
            }
        }
        map.indexed_iter()
            .map(|(pos, x)| {
                if *x == TokenPart2::BoxLeft {
                    pos.0 * 100 + pos.1
                } else {
                    0
                }
            })
            .sum()
    }

    fn can_move(map: ArrayView2<TokenPart2>, position: &[usize; 2], direction: &Direction) -> bool {
        let Some(new_position) = direction.position_from(position) else {
            return false;
        };
        match map.get(new_position) {
            Some(TokenPart2::None) => true,
            Some(TokenPart2::Wall) => false,
            Some(TokenPart2::BoxLeft) => match direction {
                Direction::Left => false,
                Direction::Right => {
                    let second_position = direction.position_from(&new_position).unwrap();
                    PuzzlePart2::can_move(map, &second_position, direction)
                }
                Direction::Up | Direction::Down => {
                    let second_position = Direction::Right.position_from(&new_position).unwrap();
                    PuzzlePart2::can_move(map, &new_position, direction)
                        && PuzzlePart2::can_move(map, &second_position, direction)
                }
            },
            Some(TokenPart2::BoxRight) => match direction {
                Direction::Right => false,
                Direction::Left => {
                    let second_position = direction.position_from(&new_position).unwrap();
                    PuzzlePart2::can_move(map, &second_position, direction)
                }
                Direction::Up | Direction::Down => {
                    let second_position = Direction::Left.position_from(&new_position).unwrap();
                    PuzzlePart2::can_move(map, &new_position, direction)
                        && PuzzlePart2::can_move(map, &second_position, direction)
                }
            },
            None => false,
        }
    }

    fn do_move(mut map: ArrayViewMut2<TokenPart2>, position: &[usize; 2], direction: &Direction) {
        let new_position = direction.position_from(position).unwrap();
        let mut second_position_opt = None;
        if let Some(token) = map.get(new_position).cloned() {
            match token {
                TokenPart2::BoxLeft => {
                    if matches!(direction, Direction::Up | Direction::Down) {
                        second_position_opt =
                            Some(Direction::Right.position_from(&new_position).unwrap());
                    }
                    PuzzlePart2::do_move(map.view_mut(), &new_position, direction);
                    if let Some(second_pos) = second_position_opt {
                        PuzzlePart2::do_move(map.view_mut(), &second_pos, direction);
                    }
                }
                TokenPart2::BoxRight => {
                    if matches!(direction, Direction::Up | Direction::Down) {
                        second_position_opt =
                            Some(Direction::Left.position_from(&new_position).unwrap());
                    }
                    PuzzlePart2::do_move(map.view_mut(), &new_position, direction);
                    if let Some(second_pos) = second_position_opt {
                        PuzzlePart2::do_move(map.view_mut(), &second_pos, direction);
                    }
                }
                _ => {}
            }
        }
        if let Some(token) = map.get(*position).cloned() {
            if token == TokenPart2::BoxLeft || token == TokenPart2::BoxRight {
                map[new_position] = token;
                map[*position] = TokenPart2::None;
            }
        }
    }
}

impl AocSolution for Day15 {
    fn part1(&self, input: &str) -> String {
        let puzzle = input.parse::<PuzzlePart1>().unwrap();
        puzzle.process().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let puzzle = input.parse::<PuzzlePart2>().unwrap();
        puzzle.process().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_SMALL: &str = r#"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"#;

    const EXAMPLE: &str = r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#;

    #[test]
    fn test_part1_small() {
        assert_eq!(Day15.part1(EXAMPLE_SMALL), "2028");
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(Day15.part1(EXAMPLE), "10092");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2024, 15).expect("Failed to get input");
        assert_eq!(Day15.part1(&input), "1495147");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(Day15.part2(EXAMPLE), "9021");
    }

    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2024, 15).expect("Failed to get input");
        assert_eq!(Day15.part2(&input), "1524905");
    }
}
