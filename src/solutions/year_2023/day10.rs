use std::collections::BTreeSet;
use std::str::FromStr;

use crate::AocSolution;

pub struct Day10;

#[derive(Ord, PartialOrd, Eq, Clone, Copy, Debug, PartialEq)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    const ALL: [Direction; 4] = [Direction::N, Direction::E, Direction::S, Direction::W];
}

fn opposite_direction(direction: &Direction) -> Direction {
    match direction {
        Direction::N => Direction::S,
        Direction::E => Direction::W,
        Direction::S => Direction::N,
        Direction::W => Direction::E,
    }
}

fn get_directions(c: &char) -> Option<(Direction, Direction)> {
    match c {
        '|' => Some((Direction::N, Direction::S)),
        '-' => Some((Direction::E, Direction::W)),
        'L' => Some((Direction::N, Direction::E)),
        'J' => Some((Direction::N, Direction::W)),
        '7' => Some((Direction::S, Direction::W)),
        'F' => Some((Direction::S, Direction::E)),
        '.' => None,
        _ => panic!("got unexpected val {}", c),
    }
}

fn tile_supports_dir(tile: &char, direction: &Direction) -> bool {
    let Some((d1, d2)) = get_directions(tile) else {
        return false;
    };
    d1 == *direction || d2 == *direction
}

fn can_go_to_tile(tile: &char, direction: &Direction) -> bool {
    if *tile == 'S' {
        return true;
    }
    tile_supports_dir(tile, &opposite_direction(direction))
}

struct Maze {
    nrows: usize,
    ncols: usize,
    map: Vec<Vec<char>>,
}

impl FromStr for Maze {
    type Err = &'static str;
    fn from_str(input: &str) -> Result<Self, <Self as FromStr>::Err> {
        let map = input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        assert!(map.iter().all(|line| line.len() == map[0].len()));
        Ok(Maze {
            nrows: map.len(),
            ncols: map[0].len(),
            map,
        })
    }
}

struct MazeWalker<'a> {
    maze: &'a Maze,
    pos: (usize, usize),
    last_direction: Option<Direction>,
}

impl<'a> MazeWalker<'a> {
    fn new(maze: &'a Maze) -> Self {
        MazeWalker {
            maze,
            pos: maze.find_start().expect("No start!"),
            last_direction: None,
        }
    }

    fn get_tile(&self, direction: &Direction) -> Option<char> {
        match direction {
            Direction::N => {
                if self.pos.0 == 0 {
                    return None;
                }
                Some(self.maze.map[self.pos.0 - 1][self.pos.1])
            }
            Direction::E => {
                if self.pos.1 == self.maze.ncols - 1 {
                    return None;
                }
                Some(self.maze.map[self.pos.0][self.pos.1 + 1])
            }
            Direction::S => {
                if self.pos.0 == self.maze.nrows - 1 {
                    return None;
                }
                Some(self.maze.map[self.pos.0 + 1][self.pos.1])
            }
            Direction::W => {
                if self.pos.1 == 0 {
                    return None;
                }
                Some(self.maze.map[self.pos.0][self.pos.1 - 1])
            }
        }
    }

    fn can_move(&self, direction: &Direction) -> bool {
        let Some(tile) = self.get_tile(direction) else {
            return false;
        };
        can_go_to_tile(&tile, direction)
    }

    fn advance_position(&mut self, direction: &Direction) {
        self.pos = match direction {
            Direction::N => (self.pos.0 - 1, self.pos.1),
            Direction::E => (self.pos.0, self.pos.1 + 1),
            Direction::S => (self.pos.0 + 1, self.pos.1),
            Direction::W => (self.pos.0, self.pos.1 - 1),
        };
        self.last_direction = Some(*direction);
    }

    fn sub_start(&self) -> char {
        assert_eq!(self.last_direction, None);
        let dirs = Direction::ALL
            .into_iter()
            .filter(|d| self.can_move(d))
            .collect::<BTreeSet<Direction>>();
        assert_eq!(2, dirs.len());
        if dirs.contains(&Direction::N) {
            if dirs.contains(&Direction::E) {
                return 'L';
            } else if dirs.contains(&Direction::W) {
                return 'J';
            } else if dirs.contains(&Direction::S) {
                return '|';
            }
        } else if dirs.contains(&Direction::S) {
            if dirs.contains(&Direction::E) {
                return 'F';
            } else if dirs.contains(&Direction::W) {
                return '7';
            }
        } else if dirs.contains(&Direction::E) && dirs.contains(&Direction::W) {
            return '-';
        }
        panic!("Unable to determine start tile");
    }

    fn make_move(&mut self) -> Option<()> {
        match self.last_direction {
            Some(ld) => {
                let tile = &self.maze.map[self.pos.0][self.pos.1];
                let directions = get_directions(tile).expect("Not on a valid tile!");
                let back = opposite_direction(&ld);
                let direction = if directions.0 == back {
                    directions.1
                } else if directions.1 == back {
                    directions.0
                } else {
                    panic!(
                        "directions {:?} from tile {:?} did not match back {:?}",
                        directions, tile, back
                    )
                };
                assert!(self.can_move(&direction));
                self.advance_position(&direction);
                if self.maze.map[self.pos.0][self.pos.1] == 'S' {
                    return None;
                }
            }
            None => {
                for dir in Direction::ALL {
                    if self.can_move(&dir) {
                        self.advance_position(&dir);
                        break;
                    }
                }
            }
        }
        Some(())
    }
}

impl Maze {
    fn find_start(&self) -> Result<(usize, usize), &'static str> {
        for (irow, row) in self.map.iter().enumerate() {
            for (icol, col) in row.iter().enumerate() {
                if *col == 'S' {
                    return Ok((irow, icol));
                }
            }
        }
        Err("No start")
    }

    fn sub_start(&self) -> char {
        MazeWalker::new(self).sub_start()
    }

    fn count_steps(&self) -> u64 {
        let mut walker = MazeWalker::new(self);
        let mut count = 0;
        loop {
            count += 1;
            if walker.make_move().is_none() {
                break;
            }
        }
        count / 2
    }

    fn count_inside(&self) -> u64 {
        let mut walker = MazeWalker::new(self);
        let mut path = BTreeSet::new();
        path.insert(walker.pos);
        loop {
            if walker.make_move().is_none() {
                break;
            }
            path.insert(walker.pos);
        }
        path.insert(walker.pos);

        let mut count = 0;
        for irow in 0..self.nrows {
            let mut crossings = 0;
            let mut on_ridge = false;
            let mut ridge_start = None;
            for icol in 0..self.ncols {
                let pos = (irow, icol);
                let tile = self.map[pos.0][pos.1];
                let tile = if tile == 'S' { self.sub_start() } else { tile };
                if path.contains(&pos) {
                    match tile {
                        '|' => {
                            crossings += 1;
                        }
                        'F' => {
                            assert!(!on_ridge);
                            on_ridge = true;
                            ridge_start = Some(tile);
                        }
                        'J' => {
                            assert!(on_ridge);
                            match ridge_start {
                                Some('F') => {
                                    crossings += 1;
                                    on_ridge = false;
                                }
                                Some('L') => {
                                    on_ridge = false;
                                }
                                _ => panic!(),
                            }
                        }
                        '7' => {
                            assert!(on_ridge);
                            match ridge_start {
                                Some('L') => {
                                    crossings += 1;
                                    on_ridge = false;
                                }
                                Some('F') => {
                                    on_ridge = false;
                                }
                                _ => panic!(),
                            }
                        }
                        'L' => {
                            assert!(!on_ridge);
                            on_ridge = true;
                            ridge_start = Some(tile);
                        }
                        '-' => {
                            assert!(on_ridge);
                        }
                        _ => (),
                    }
                } else if crossings % 2 == 1 {
                    count += 1;
                }
            }
            assert!(!on_ridge);
        }
        count
    }
}

impl AocSolution for Day10 {
    fn part1(&self, input: &str) -> String {
        let maze: Maze = input.parse().unwrap();
        maze.count_steps().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let maze: Maze = input.parse().unwrap();
        maze.count_inside().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = r"..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    const EXAMPLE_2: &str = r"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

    #[test]
    fn test_part1_example() {
        assert_eq!(Day10.part1(EXAMPLE_1), "8");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2023, 10).expect("Failed to get input");
        assert_eq!(Day10.part1(&input), "6909");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(Day10.part2(EXAMPLE_2), "4");
    }

    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2023, 10).expect("Failed to get input");
        assert_eq!(Day10.part2(&input), "461");
    }
}
