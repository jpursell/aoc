use std::collections::BTreeSet;
use std::str::FromStr;

use crate::AocSolution;

pub struct Day11;

#[derive(Debug, Clone)]
struct Point {
    row: usize,
    col: usize,
}

#[derive(Debug, Clone)]
struct Universe {
    points: Vec<Point>,
}

impl FromStr for Universe {
    type Err = &'static str;
    fn from_str(input: &str) -> Result<Self, <Self as FromStr>::Err> {
        let mut points = Vec::new();
        for (row, line) in input.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                if c == '#' {
                    points.push(Point { row, col });
                }
            }
        }
        Ok(Universe { points })
    }
}

fn compute_distance(point0: &Point, point1: &Point) -> u64 {
    (point0.row.abs_diff(point1.row) + point0.col.abs_diff(point1.col)) as u64
}

impl Universe {
    fn find_empty_rows(&self) -> Vec<usize> {
        let rows = self.points.iter().map(|p| p.row).collect::<BTreeSet<_>>();
        let max_row = rows.iter().max().copied().unwrap_or(0);
        let mut empty_rows = (0..max_row)
            .filter(|row| !rows.contains(row))
            .collect::<Vec<_>>();
        empty_rows.sort();
        empty_rows
    }

    fn find_empty_cols(&self) -> Vec<usize> {
        let cols = self.points.iter().map(|p| p.col).collect::<BTreeSet<_>>();
        let max_col = cols.iter().max().copied().unwrap_or(0);
        let mut empty_cols = (0..max_col)
            .filter(|col| !cols.contains(col))
            .collect::<Vec<_>>();
        empty_cols.sort();
        empty_cols
    }

    fn expand_points(&mut self, factor: u64) {
        let expansion = (factor - 1) as usize;
        for row in self.find_empty_rows().iter().rev() {
            for point in self.points.iter_mut() {
                if point.row > *row {
                    point.row += expansion;
                }
            }
        }
        for col in self.find_empty_cols().iter().rev() {
            for point in self.points.iter_mut() {
                if point.col > *col {
                    point.col += expansion;
                }
            }
        }
    }

    fn calc_distances_sum(&mut self, factor: u64) -> u64 {
        self.expand_points(factor);
        let mut distance = 0;
        for (ipoint, point0) in self.points.iter().enumerate() {
            for point1 in &self.points[ipoint + 1..] {
                distance += compute_distance(point0, point1);
            }
        }
        distance
    }
}

impl AocSolution for Day11 {
    fn part1(&self, input: &str) -> String {
        let mut universe: Universe = input.parse().unwrap();
        universe.calc_distances_sum(2).to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut universe: Universe = input.parse().unwrap();
        universe.calc_distances_sum(1_000_000).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_part1_example() {
        assert_eq!(Day11.part1(EXAMPLE), "374");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2023, 11).expect("Failed to get input");
        assert_eq!(Day11.part1(&input), "9177603");
    }

    #[test]
    fn test_part2_example() {
        let mut universe: Universe = EXAMPLE.parse().unwrap();
        assert_eq!(universe.calc_distances_sum(10), 1030);
        let mut universe: Universe = EXAMPLE.parse().unwrap();
        assert_eq!(universe.calc_distances_sum(100), 8410);
    }

    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2023, 11).expect("Failed to get input");
        assert_eq!(Day11.part2(&input), "632003913611");
    }
}
