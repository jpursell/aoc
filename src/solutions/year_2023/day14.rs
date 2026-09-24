use crate::AocSolution;

pub struct Day14;

mod a {
    use std::str::FromStr;

    #[derive(Debug, PartialEq, Eq)]
    enum Rock {
        R,
        S,
        N,
    }
    impl TryFrom<char> for Rock {
        type Error = &'static str;
        fn try_from(value: char) -> Result<Self, Self::Error> {
            match value {
                'O' => Ok(Rock::R),
                '.' => Ok(Rock::N),
                '#' => Ok(Rock::S),
                _ => Err("Unknown Rock"),
            }
        }
    }
    struct RockField {
        rocks: Vec<Vec<Rock>>,
    }
    impl FromStr for RockField {
        type Err = &'static str;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let rocks = s
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| Rock::try_from(c).unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            Ok(RockField { rocks })
        }
    }
    #[derive(Debug, PartialEq, Eq)]
    enum Direction {
        N,
        // E,
        // S,
        // W,
    }
    impl RockField {
        fn roll(&mut self, direction: Direction) -> bool {
            assert_eq!(direction, Direction::N);
            let mut changed = false;
            for irow in 1..self.rocks.len() {
                for icol in 0..self.rocks[0].len() {
                    if self.rocks[irow][icol] == Rock::R && self.rocks[irow - 1][icol] == Rock::N {
                        self.rocks[irow][icol] = Rock::N;
                        self.rocks[irow - 1][icol] = Rock::R;
                        changed = true
                    }
                }
            }
            changed
        }
        fn count_rocks(&self) -> usize {
            let nrows = self.rocks.len();
            self.rocks
                .iter()
                .enumerate()
                .map(|(irow, row)| {
                    row.iter()
                        .map(|r| if *r == Rock::R { nrows - irow } else { 0 })
                        .sum::<usize>()
                })
                .sum::<usize>()
        }
    }
    pub fn run(input: &str) -> usize {
        let mut field = input.parse::<RockField>().unwrap();
        loop {
            if !field.roll(Direction::N) {
                break;
            }
        }
        field.count_rocks()
    }
}

mod b {
    use ndarray::{Array2, ArrayViewMut1, Axis};
    use std::str::FromStr;

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum Rock {
        R,
        S,
        N,
    }

    impl TryFrom<char> for Rock {
        type Error = &'static str;
        fn try_from(value: char) -> Result<Self, Self::Error> {
            match value {
                'O' => Ok(Rock::R),
                '.' => Ok(Rock::N),
                '#' => Ok(Rock::S),
                _ => Err("Unknown Rock"),
            }
        }
    }

    #[derive(PartialEq, Debug)]
    struct RockField {
        rocks: Array2<Rock>,
        nrows: usize,
        // ncols: usize,
    }

    impl FromStr for RockField {
        type Err = &'static str;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let rocks = s
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| Rock::try_from(c).unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            let nrows = rocks.len();
            let ncols = rocks[0].len();
            let rocks = rocks.concat();
            let rocks = Array2::from_shape_vec((nrows, ncols), rocks).unwrap();
            Ok(RockField {
                rocks,
                nrows,
                // ncols,
            })
        }
    }

    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    enum Direction {
        N,
        E,
        S,
        W,
    }

    impl RockField {
        fn roll_arr(arr: &mut ArrayViewMut1<Rock>, forward: bool) {
            if forward {
                // if forward then move rocks to higher index
                let mut write = arr.len() - 1;
                for read in (0..arr.len()).rev() {
                    if arr[read] == Rock::S {
                        write = read;
                    }
                    while write > 0 && arr[write] != Rock::N {
                        write -= 1;
                    }
                    if read >= write || arr[write] != Rock::N {
                        continue;
                    }
                    if arr[read] == Rock::R {
                        arr[read] = Rock::N;
                        arr[write] = Rock::R;
                    }
                }
            } else {
                // if !forward then move rocks to lower index
                let mut write = 0;
                for read in 0..arr.len() {
                    if arr[read] == Rock::S {
                        write = read;
                    }
                    while write < arr.len() - 1 && arr[write] != Rock::N {
                        write += 1;
                    }
                    if read <= write || arr[write] != Rock::N {
                        continue;
                    }
                    if arr[read] == Rock::R {
                        arr[read] = Rock::N;
                        arr[write] = Rock::R;
                    }
                }
            }
        }
        fn roll(&mut self, direction: Direction) {
            match direction {
                Direction::S => {
                    self.rocks
                        .axis_iter_mut(Axis(1))
                        .for_each(|mut c| RockField::roll_arr(&mut c, true));
                }
                Direction::N => {
                    self.rocks
                        .axis_iter_mut(Axis(1))
                        .for_each(|mut c| RockField::roll_arr(&mut c, false));
                }
                Direction::E => {
                    self.rocks
                        .axis_iter_mut(Axis(0))
                        .for_each(|mut c| RockField::roll_arr(&mut c, true));
                }
                Direction::W => {
                    self.rocks
                        .axis_iter_mut(Axis(0))
                        .for_each(|mut c| RockField::roll_arr(&mut c, false));
                }
            }
        }

        fn roll_cycle(&mut self) {
            self.roll(Direction::N);
            self.roll(Direction::W);
            self.roll(Direction::S);
            self.roll(Direction::E);
        }

        fn count_rocks(&self) -> usize {
            self.rocks
                .indexed_iter()
                .map(
                    |((irow, _), r)| {
                        if *r == Rock::R {
                            self.nrows - irow
                        } else {
                            0
                        }
                    },
                )
                .sum::<usize>()
        }
    }
    pub fn run(input: &str) -> usize {
        let mut field = input.parse::<RockField>().unwrap();
        let data = (0..300)
            .map(|_| {
                field.roll_cycle();
                field.count_rocks()
            })
            .collect::<Vec<_>>();
        let last = *data.last().unwrap();
        let pos = data
            .iter()
            .enumerate()
            .filter(|(_, x)| **x == last)
            .map(|(i, _)| i)
            .collect::<Vec<_>>();
        let pdiff = pos.windows(2).map(|x| x[1] - x[0]).collect::<Vec<_>>();
        pos.iter()
            .zip(pdiff.iter())
            .for_each(|(p, d)| println!("{} {}", p, d));
        assert!(pdiff.len() > 1);
        assert!(pdiff.iter().all(|d| *d == pdiff[0]));

        let ncycles = 1_000_000_000;
        data[pos[0] + ((ncycles - 1 - pos[0]) % pdiff[0])]
    }
}

impl AocSolution for Day14 {
    fn part1(&self, input: &str) -> String {
        a::run(input).to_string()
    }

    fn part2(&self, input: &str) -> String {
        b::run(input).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;

    #[test]
    fn test_part1_example() {
        assert_eq!(Day14.part1(EXAMPLE), "136");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2023, 14).expect("Failed to get input");
        assert_eq!(Day14.part1(&input), "110128");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(Day14.part2(EXAMPLE), "64");
    }

    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2023, 14).expect("Failed to get input");
        assert_eq!(Day14.part2(&input), "103861");
    }
}
