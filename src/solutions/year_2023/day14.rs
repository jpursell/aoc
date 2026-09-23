use std::collections::HashMap;

use crate::AocSolution;

pub struct Day14;

fn tilt_start(arr: &mut [u8]) {
    let mut write = 0;
    for read in 0..arr.len() {
        if arr[read] == b'#' {
            write = read + 1;
        } else if arr[read] == b'O' {
            arr[read] = b'.';
            arr[write] = b'O';
            write += 1;
        }
    }
}

fn tilt_end(arr: &mut [u8]) {
    let mut write = arr.len().saturating_sub(1);
    for read in (0..arr.len()).rev() {
        if arr[read] == b'#' {
            write = read.saturating_sub(1);
        } else if arr[read] == b'O' {
            arr[read] = b'.';
            arr[write] = b'O';
            write = write.saturating_sub(1);
        }
    }
}

fn tilt_north(grid: &mut [Vec<u8>]) {
    let nrows = grid.len();
    let ncols = grid[0].len();
    for c in 0..ncols {
        let mut col: Vec<u8> = (0..nrows).map(|r| grid[r][c]).collect();
        tilt_start(&mut col);
        for r in 0..nrows {
            grid[r][c] = col[r];
        }
    }
}

fn tilt_south(grid: &mut [Vec<u8>]) {
    let nrows = grid.len();
    let ncols = grid[0].len();
    for c in 0..ncols {
        let mut col: Vec<u8> = (0..nrows).map(|r| grid[r][c]).collect();
        tilt_end(&mut col);
        for r in 0..nrows {
            grid[r][c] = col[r];
        }
    }
}

fn tilt_west(grid: &mut [Vec<u8>]) {
    for row in grid.iter_mut() {
        tilt_start(row);
    }
}

fn tilt_east(grid: &mut [Vec<u8>]) {
    for row in grid.iter_mut() {
        tilt_end(row);
    }
}

fn cycle(grid: &mut [Vec<u8>]) {
    tilt_north(grid);
    tilt_west(grid);
    tilt_south(grid);
    tilt_east(grid);
}

fn calculate_load(grid: &[Vec<u8>]) -> usize {
    let nrows = grid.len();
    grid.iter()
        .enumerate()
        .map(|(r, row)| {
            let count = row.iter().filter(|&&c| c == b'O').count();
            count * (nrows - r)
        })
        .sum()
}

fn parse_grid(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.as_bytes().to_vec())
        .collect()
}

impl AocSolution for Day14 {
    fn part1(&self, input: &str) -> String {
        let mut grid = parse_grid(input);
        tilt_north(&mut grid);
        calculate_load(&grid).to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut grid = parse_grid(input);
        let mut seen = HashMap::new();
        let total_cycles = 1_000_000_000;

        let mut step = 0;
        while step < total_cycles {
            let key = grid.clone();
            if let Some(&prev_step) = seen.get(&key) {
                let cycle_len = step - prev_step;
                let remaining = (total_cycles - step) % cycle_len;
                for _ in 0..remaining {
                    cycle(&mut grid);
                }
                break;
            }
            seen.insert(key, step);
            cycle(&mut grid);
            step += 1;
        }

        calculate_load(&grid).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

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
