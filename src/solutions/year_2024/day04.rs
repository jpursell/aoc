use crate::AocSolution;
use ndarray::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Token {
    X,
    M,
    A,
    S,
    O,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

pub struct Day04;

impl Day04 {
    fn extract(str: &str) -> Array2<Token> {
        let nrows = str.lines().count();
        let ncols = str.lines().next().unwrap().chars().count();
        let shape = (nrows, ncols);
        let mut tokens = Vec::with_capacity(nrows * ncols);
        for line in str.lines() {
            for char in line.chars() {
                match char {
                    'X' => tokens.push(Token::X),
                    'M' => tokens.push(Token::M),
                    'A' => tokens.push(Token::A),
                    'S' => tokens.push(Token::S),
                    _ => tokens.push(Token::O),
                }
            }
        }
        Array2::from_shape_vec(shape, tokens).unwrap()
    }

    fn check_location_direction_p1(
        tokens: ArrayView2<Token>,
        i: usize,
        j: usize,
        direction: &Direction,
    ) -> bool {
        let i = i as i64;
        let j = j as i64;
        let (di, dj) = match direction {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            Direction::UpRight => (-1, 1),
            Direction::UpLeft => (-1, -1),
            Direction::DownRight => (1, 1),
            Direction::DownLeft => (1, -1),
        };
        let token_offsets = [(Token::M, 1), (Token::A, 2), (Token::S, 3)];
        for (match_token, offset) in &token_offsets {
            let row = usize::try_from(i + di * offset);
            if row.is_err() {
                return false;
            }
            let col = usize::try_from(j + dj * offset);
            if col.is_err() {
                return false;
            }
            match tokens.get([row.unwrap(), col.unwrap()]) {
                Some(token) => {
                    if token != match_token {
                        return false;
                    }
                }
                None => {
                    return false;
                }
            }
        }
        true
    }

    fn check_location_p1(tokens: ArrayView2<Token>, i: usize, j: usize) -> usize {
        let directions = [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
            Direction::UpRight,
            Direction::UpLeft,
            Direction::DownRight,
            Direction::DownLeft,
        ];
        directions
            .iter()
            .filter(|&d| Day04::check_location_direction_p1(tokens, i, j, d))
            .count()
    }

    fn process_p1(tokens: ArrayView2<Token>) -> usize {
        let mut out = 0;

        for ((i, j), token) in tokens.indexed_iter() {
            if token == &Token::X {
                out += Day04::check_location_p1(tokens, i, j);
            }
        }
        out
    }

    fn check_location_p2(tokens: ArrayView2<Token>, i: usize, j: usize) -> bool {
        let ul = tokens[[i - 1, j - 1]];
        let ur = tokens[[i - 1, j + 1]];
        let ll = tokens[[i + 1, j - 1]];
        let lr = tokens[[i + 1, j + 1]];
        match ul {
            Token::M => {
                if lr != Token::S {
                    return false;
                }
            }
            Token::S => {
                if lr != Token::M {
                    return false;
                }
            }
            _ => {
                return false;
            }
        }
        match ur {
            Token::M => {
                if ll != Token::S {
                    return false;
                }
            }
            Token::S => {
                if ll != Token::M {
                    return false;
                }
            }
            _ => {
                return false;
            }
        }
        true
    }

    fn process_p2(tokens: ArrayView2<Token>) -> usize {
        let mut out = 0;

        let shape = tokens.shape();
        for ((i, j), token) in tokens.indexed_iter() {
            if i == 0 || j == 0 || i == shape[0] - 1 || j == shape[1] - 1 {
                continue;
            }
            if token == &Token::A && Day04::check_location_p2(tokens, i, j) {
                out += 1;
            }
        }
        out
    }
}

impl AocSolution for Day04 {
    fn part1(&self, input: &str) -> String {
        let tokens = Day04::extract(input);
        Day04::process_p1(tokens.view()).to_string()
    }

    fn part2(&self, input: &str) -> String {
        let tokens = Day04::extract(input);
        Day04::process_p2(tokens.view()).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_part1_example() {
        assert_eq!(Day04.part1(EXAMPLE), "18");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2024, 4).expect("Failed to get input");
        assert_eq!(Day04.part1(&input), "2344");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(Day04.part2(EXAMPLE), "9");
    }

    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2024, 4).expect("Failed to get input");
        assert_eq!(Day04.part2(&input), "1815");
    }
}
