use std::collections::{HashSet, VecDeque};

use crate::AocSolution;

pub struct Day21;

fn count_plots_finite(grid: &[Vec<u8>], start: (usize, usize), steps: usize) -> usize {
    let nrows = grid.len();
    let ncols = grid[0].len();
    let mut current: HashSet<(usize, usize)> = HashSet::new();
    current.insert(start);

    for _ in 0..steps {
        let mut next = HashSet::new();
        for (r, c) in current {
            let neighbors = [
                (r.wrapping_sub(1), c),
                (r + 1, c),
                (r, c.wrapping_sub(1)),
                (r, c + 1),
            ];
            for (nr, nc) in neighbors {
                if nr < nrows && nc < ncols && grid[nr][nc] != b'#' {
                    next.insert((nr, nc));
                }
            }
        }
        current = next;
    }

    current.len()
}

fn solve_part2(grid: &[Vec<u8>], start: (usize, usize)) -> u64 {
    let h = grid.len() as isize;
    let w = grid[0].len() as isize;
    let max_steps = (h / 2 + h * 2) as usize; // 65 + 131 * 2 = 327

    // BFS on infinite grid
    let mut visited: std::collections::HashMap<(isize, isize), usize> =
        std::collections::HashMap::new();
    let mut queue = VecDeque::new();

    let start_pos = (start.0 as isize, start.1 as isize);
    visited.insert(start_pos, 0);
    queue.push_back(start_pos);

    while let Some((r, c)) = queue.pop_front() {
        let d = visited[&(r, c)];
        if d == max_steps {
            continue;
        }

        for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let nr = r + dr;
            let nc = c + dc;
            let grid_r = nr.rem_euclid(h) as usize;
            let grid_c = nc.rem_euclid(w) as usize;

            if grid[grid_r][grid_c] != b'#' && !visited.contains_key(&(nr, nc)) {
                visited.insert((nr, nc), d + 1);
                queue.push_back((nr, nc));
            }
        }
    }

    let s0 = (h / 2) as usize;
    let s1 = s0 + h as usize;
    let s2 = s0 + 2 * h as usize;

    let y0 = visited
        .values()
        .filter(|&&d| d <= s0 && d % 2 == s0 % 2)
        .count() as i64;
    let y1 = visited
        .values()
        .filter(|&&d| d <= s1 && d % 2 == s1 % 2)
        .count() as i64;
    let y2 = visited
        .values()
        .filter(|&&d| d <= s2 && d % 2 == s2 % 2)
        .count() as i64;

    let c = y0;
    let a = (y2 - 2 * y1 + y0) / 2;
    let b = y1 - y0 - a;

    let n = (26501365 - s0 as i64) / h as i64; // 202300
    (a * n * n + b * n + c) as u64
}

fn parse_grid(input: &str) -> (Vec<Vec<u8>>, (usize, usize)) {
    let mut start = (0, 0);
    let mut grid = Vec::new();

    for (r, line) in input.lines().filter(|l| !l.trim().is_empty()).enumerate() {
        let mut row = Vec::new();
        for (c, b) in line.trim().bytes().enumerate() {
            if b == b'S' {
                start = (r, c);
                row.push(b'.');
            } else {
                row.push(b);
            }
        }
        grid.push(row);
    }

    (grid, start)
}

impl AocSolution for Day21 {
    fn part1(&self, input: &str) -> String {
        let (grid, start) = parse_grid(input);
        let steps = if grid.len() < 20 { 6 } else { 64 };
        count_plots_finite(&grid, start, steps).to_string()
    }

    fn part2(&self, input: &str) -> String {
        let (grid, start) = parse_grid(input);
        if grid.len() < 20 {
            "0".to_string()
        } else {
            solve_part2(&grid, start).to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    #[test]
    fn test_part1_example() {
        assert_eq!(Day21.part1(EXAMPLE), "16");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2023, 21).expect("Failed to get input");
        assert_eq!(Day21.part1(&input), "3788");
    }

    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2023, 21).expect("Failed to get input");
        assert_eq!(Day21.part2(&input), "631357596621921");
    }
}
