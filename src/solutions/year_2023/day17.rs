use std::cmp::Ordering;
use std::collections::BinaryHeap;

use crate::AocSolution;

pub struct Day17;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    r: usize,
    c: usize,
    dir: usize, // 0: Horizontal (next moves are vertical), 1: Vertical (next moves are horizontal)
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn solve(input: &str, min_step: usize, max_step: usize) -> usize {
    let grid: Vec<Vec<usize>> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.bytes().map(|b| (b - b'0') as usize).collect())
        .collect();

    let nrows = grid.len();
    let ncols = grid[0].len();

    let mut dist = vec![vec![[usize::MAX; 2]; ncols]; nrows];
    let mut heap = BinaryHeap::new();

    dist[0][0][0] = 0;
    dist[0][0][1] = 0;
    heap.push(State {
        cost: 0,
        r: 0,
        c: 0,
        dir: 0,
    });
    heap.push(State {
        cost: 0,
        r: 0,
        c: 0,
        dir: 1,
    });

    while let Some(State { cost, r, c, dir }) = heap.pop() {
        if r == nrows - 1 && c == ncols - 1 {
            return cost;
        }

        if cost > dist[r][c][dir] {
            continue;
        }

        let next_dir = 1 - dir;
        let directions: &[(isize, isize)] = if dir == 0 {
            // Last move was horizontal, next moves are vertical (North, South)
            &[(-1, 0), (1, 0)]
        } else {
            // Last move was vertical, next moves are horizontal (East, West)
            &[(0, -1), (0, 1)]
        };

        for &(dr, dc) in directions {
            let mut step_cost = 0;
            for k in 1..=max_step {
                let nr = r as isize + dr * k as isize;
                let nc = c as isize + dc * k as isize;

                if nr < 0 || nr >= nrows as isize || nc < 0 || nc >= ncols as isize {
                    break;
                }

                let (nr, nc) = (nr as usize, nc as usize);
                step_cost += grid[nr][nc];

                if k >= min_step {
                    let new_cost = cost + step_cost;
                    if new_cost < dist[nr][nc][next_dir] {
                        dist[nr][nc][next_dir] = new_cost;
                        heap.push(State {
                            cost: new_cost,
                            r: nr,
                            c: nc,
                            dir: next_dir,
                        });
                    }
                }
            }
        }
    }

    panic!("No path found");
}

impl AocSolution for Day17 {
    fn part1(&self, input: &str) -> String {
        solve(input, 1, 3).to_string()
    }

    fn part2(&self, input: &str) -> String {
        solve(input, 4, 10).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    const EXAMPLE_2: &str = r"111111111111
999999999991
999999999991
999999999991
999999999991";

    #[test]
    fn test_part1_example() {
        assert_eq!(Day17.part1(EXAMPLE), "102");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2023, 17).expect("Failed to get input");
        assert_eq!(Day17.part1(&input), "1065");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(Day17.part2(EXAMPLE), "94");
        assert_eq!(Day17.part2(EXAMPLE_2), "71");
    }

    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2023, 17).expect("Failed to get input");
        assert_eq!(Day17.part2(&input), "1249");
    }
}
