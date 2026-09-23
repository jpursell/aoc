use crate::AocSolution;

pub struct Day13;

fn diff_rows(grid: &[Vec<u8>], r1: usize, r2: usize) -> usize {
    grid[r1]
        .iter()
        .zip(&grid[r2])
        .filter(|(a, b)| a != b)
        .count()
}

fn diff_cols(grid: &[Vec<u8>], c1: usize, c2: usize) -> usize {
    (0..grid.len())
        .filter(|&r| grid[r][c1] != grid[r][c2])
        .count()
}

fn find_row_reflection(grid: &[Vec<u8>], target_diff: usize) -> Option<usize> {
    let nrows = grid.len();
    for r in 1..nrows {
        let mut diffs = 0;
        let count = r.min(nrows - r);
        for d in 0..count {
            diffs += diff_rows(grid, r - 1 - d, r + d);
            if diffs > target_diff {
                break;
            }
        }
        if diffs == target_diff {
            return Some(r);
        }
    }
    None
}

fn find_col_reflection(grid: &[Vec<u8>], target_diff: usize) -> Option<usize> {
    let ncols = grid[0].len();
    for c in 1..ncols {
        let mut diffs = 0;
        let count = c.min(ncols - c);
        for d in 0..count {
            diffs += diff_cols(grid, c - 1 - d, c + d);
            if diffs > target_diff {
                break;
            }
        }
        if diffs == target_diff {
            return Some(c);
        }
    }
    None
}

fn summarize_pattern(pattern_str: &str, target_diff: usize) -> usize {
    let grid: Vec<Vec<u8>> = pattern_str
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.as_bytes().to_vec())
        .collect();
    if let Some(r) = find_row_reflection(&grid, target_diff) {
        return r * 100;
    }
    if let Some(c) = find_col_reflection(&grid, target_diff) {
        return c;
    }
    panic!("No reflection found for pattern:\n{}", pattern_str);
}

fn solve(input: &str, target_diff: usize) -> usize {
    input
        .split("\n\n")
        .map(|block| block.trim())
        .filter(|block| !block.is_empty())
        .map(|block| summarize_pattern(block, target_diff))
        .sum()
}

impl AocSolution for Day13 {
    fn part1(&self, input: &str) -> String {
        solve(input, 0).to_string()
    }

    fn part2(&self, input: &str) -> String {
        solve(input, 1).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn test_part1_example() {
        assert_eq!(Day13.part1(EXAMPLE), "405");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2023, 13).expect("Failed to get input");
        assert_eq!(Day13.part1(&input), "29130");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(Day13.part2(EXAMPLE), "400");
    }

    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2023, 13).expect("Failed to get input");
        assert_eq!(Day13.part2(&input), "33438");
    }
}
