use crate::AocSolution;

pub struct Day16;

#[derive(Clone, Copy)]
struct Beam {
    r: usize,
    c: usize,
    dir: usize, // 0: N, 1: E, 2: S, 3: W
}

fn count_energized(grid: &[&[u8]], start: Beam) -> usize {
    let nrows = grid.len();
    let ncols = grid[0].len();
    let mut visited = vec![0u8; nrows * ncols];
    let mut stack = Vec::with_capacity(256);

    stack.push(start);

    while let Some(Beam { r, c, dir }) = stack.pop() {
        let idx = r * ncols + c;
        if visited[idx] & (1 << dir) != 0 {
            continue;
        }
        visited[idx] |= 1 << dir;

        let cell = grid[r][c];
        let next_dirs = match cell {
            b'.' => Dirs::one(dir),
            b'-' => {
                if dir == 0 || dir == 2 {
                    Dirs::two(1, 3)
                } else {
                    Dirs::one(dir)
                }
            }
            b'|' => {
                if dir == 1 || dir == 3 {
                    Dirs::two(0, 2)
                } else {
                    Dirs::one(dir)
                }
            }
            b'/' => Dirs::one(dir ^ 1),
            b'\\' => Dirs::one(3 - dir),
            _ => Dirs::one(dir),
        };

        for next_dir in next_dirs.as_slice() {
            let (dr, dc) = match next_dir {
                0 => (-1isize, 0isize),
                1 => (0, 1),
                2 => (1, 0),
                3 => (0, -1),
                _ => unreachable!(),
            };
            let nr = r as isize + dr;
            let nc = c as isize + dc;
            if nr >= 0 && nr < nrows as isize && nc >= 0 && nc < ncols as isize {
                stack.push(Beam {
                    r: nr as usize,
                    c: nc as usize,
                    dir: *next_dir,
                });
            }
        }
    }

    visited.iter().filter(|&&v| v > 0).count()
}

#[derive(Clone, Copy)]
struct Dirs {
    dirs: [usize; 2],
    len: usize,
}

impl Dirs {
    fn one(d: usize) -> Self {
        Dirs {
            dirs: [d, 0],
            len: 1,
        }
    }
    fn two(d1: usize, d2: usize) -> Self {
        Dirs {
            dirs: [d1, d2],
            len: 2,
        }
    }
    fn as_slice(&self) -> &[usize] {
        &self.dirs[..self.len]
    }
}

impl AocSolution for Day16 {
    fn part1(&self, input: &str) -> String {
        let grid: Vec<&[u8]> = input
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|l| l.as_bytes())
            .collect();
        let start = Beam { r: 0, c: 0, dir: 1 };
        count_energized(&grid, start).to_string()
    }

    fn part2(&self, input: &str) -> String {
        let grid: Vec<&[u8]> = input
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|l| l.as_bytes())
            .collect();
        let nrows = grid.len();
        let ncols = grid[0].len();

        let mut max_energy = 0;

        for c in 0..ncols {
            max_energy = max_energy.max(count_energized(&grid, Beam { r: 0, c, dir: 2 }));
            max_energy = max_energy.max(count_energized(
                &grid,
                Beam {
                    r: nrows - 1,
                    c,
                    dir: 0,
                },
            ));
        }

        for r in 0..nrows {
            max_energy = max_energy.max(count_energized(&grid, Beam { r, c: 0, dir: 1 }));
            max_energy = max_energy.max(count_energized(
                &grid,
                Beam {
                    r,
                    c: ncols - 1,
                    dir: 3,
                },
            ));
        }

        max_energy.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    fn test_part1_example() {
        assert_eq!(Day16.part1(EXAMPLE), "46");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2023, 16).expect("Failed to get input");
        assert_eq!(Day16.part1(&input), "7236");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(Day16.part2(EXAMPLE), "51");
    }

    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2023, 16).expect("Failed to get input");
        assert_eq!(Day16.part2(&input), "7521");
    }
}
