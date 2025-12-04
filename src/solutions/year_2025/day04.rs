use ndarray::prelude::*;

enum Token {
    Roll,
    Space,
}

fn parse(input: &str) -> Array2<Token> {
    let nrows = input.lines().count();
    let ncols = input.lines().next().unwrap().chars().count();
    let mut tokens = Vec::with_capacity(nrows * ncols);
    input.lines().for_each(|line| {
        line.chars()
            .map(|c| match c {
                '@' => Token::Roll,
                '.' => Token::Space,
                _ => panic!("can not handle character"),
            })
            .for_each(|t| {
                tokens.push(t);
            });
    });
    Array2::from_shape_vec((nrows, ncols), tokens).unwrap()
}

fn find_lonely_rolls(grid: &Array2<Token>) -> Array2<bool> {
    let mut lonely = Array2::from_elem(grid.raw_dim(), false);
    grid.indexed_iter().for_each(|((r, c), x)| {
        if matches!(x, Token::Roll) {
            let mut count = 0;
            for dr in -1_i32..=1 {
                for dc in -1_i32..=1 {
                    if dr == 0 && dc == 0 {
                        continue;
                    }
                    let ir = r as i32 + dr;
                    if ir < 0 {
                        continue;
                    }
                    let ic = c as i32 + dc;
                    if ic < 0 {
                        continue;
                    }
                    let index = (ir as usize, ic as usize);
                    if matches!(grid.get(index), Some(Token::Roll)) {
                        count += 1;
                    }
                }
            }
            if count < 4 {
                if let Some(val) = lonely.get_mut((r, c)) {
                    *val = true;
                }
            }
        }
    });
    lonely
}

fn count_lonely_rolls(lonely: &Array2<bool>) -> u64 {
    lonely.map(|v| if *v { 1 } else { 0 }).sum()
}

fn remove_lonely(grid: &mut Array2<Token>, lonely: Array2<bool>) {
    grid.iter_mut().zip(lonely.iter()).for_each(|(g, l)| {
        if *l {
            *g = Token::Space;
        }
    });
}

use crate::AocSolution;

pub struct Day04;

impl AocSolution for Day04 {
    fn part1(&self, input: &str) -> String {
        count_lonely_rolls(&find_lonely_rolls(&parse(input))).to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut grid = parse(input);
        let mut count = 0;
        loop {
            let lonely = find_lonely_rolls(&grid);
            let nlonely = count_lonely_rolls(&lonely);
            if nlonely == 0 {
                break;
            }
            count += nlonely;
            remove_lonely(&mut grid, lonely);
        }
        count.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_part1_example() {
        assert_eq!(Day04.part1(EXAMPLE), "13");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2025, 4).expect("Failed to get input");
        assert_eq!(Day04.part1(&input), "1502");
    }

    #[test]

    fn test_part2_example() {
        assert_eq!(Day04.part2(EXAMPLE), "43");
    }

    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2025, 4).expect("Failed to get input");
        assert_eq!(Day04.part2(&input), "9083");
    }
}
