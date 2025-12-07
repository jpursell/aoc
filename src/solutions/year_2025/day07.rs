use ndarray::prelude::*;

use crate::AocSolution;

enum Token {
    Space,
    Splitter,
    Tachyon(u64),
}

fn parse(input: &str) -> Array2<Token> {
    let nrows = input.lines().count();
    let ncols = input
        .lines()
        .next()
        .expect("Not even 1 line given to parse")
        .chars()
        .count();
    let tokens = input
        .chars()
        .filter(|c| c != &'\n')
        .map(|c| match c {
            '.' => Token::Space,
            '^' => Token::Splitter,
            'S' => Token::Tachyon(1),
            _ => panic!("Unexpected token"),
        })
        .collect();
    Array2::from_shape_vec((nrows, ncols), tokens).expect("Could not form 2d array")
}

fn simulate_beams(mut map: Array2<Token>) -> (u64, u64) {
    let nrows = map.shape()[0];
    let ncols = map.shape()[1];
    let mut splits = 0;
    for irow in 0..nrows {
        for icol in 0..ncols {
            if let Token::Tachyon(n) = map[[irow, icol]] {
                splits += set_tachyon(map.view_mut(), (irow + 1, icol), n);
            }
        }
    }
    let npaths = map
        .row(nrows - 1)
        .iter()
        .map(|t| if let Token::Tachyon(n) = t { n } else { &0 })
        .sum();
    (splits, npaths)
}

fn set_tachyon(mut map: ArrayViewMut2<Token>, pos: (usize, usize), n: u64) -> u64 {
    match map.get(pos) {
        Some(Token::Space) => {
            *(map.get_mut(pos).unwrap()) = Token::Tachyon(n);
            0
        }
        Some(Token::Splitter) => {
            set_tachyon(map.view_mut(), (pos.0, pos.1 - 1), n)
                + set_tachyon(map.view_mut(), (pos.0, pos.1 + 1), n)
                + 1
        }
        Some(Token::Tachyon(x)) => {
            *(map.get_mut(pos).unwrap()) = Token::Tachyon(n + x);
            0
        }
        None => 0,
    }
}

pub struct Day07;

impl AocSolution for Day07 {
    fn part1(&self, input: &str) -> String {
        simulate_beams(parse(input)).0.to_string()
    }

    fn part2(&self, input: &str) -> String {
        simulate_beams(parse(input)).1.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_part1_example() {
        assert_eq!(Day07.part1(EXAMPLE), "21");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2025, 7).expect("Failed to get input");
        assert_eq!(Day07.part1(&input), "1642");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(Day07.part2(EXAMPLE), "40");
    }

    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2025, 7).expect("Failed to get input");
        assert_eq!(Day07.part2(&input), "47274292756692");
    }
}
