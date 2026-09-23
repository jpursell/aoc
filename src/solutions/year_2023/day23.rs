use std::collections::HashMap;

use crate::AocSolution;

pub struct Day23;

fn dfs_part1(
    r: usize,
    c: usize,
    end: (usize, usize),
    grid: &[&[u8]],
    visited: &mut [Vec<bool>],
) -> Option<usize> {
    if (r, c) == end {
        return Some(0);
    }

    visited[r][c] = true;
    let mut max_dist = None;

    let dirs: &[(isize, isize)] = match grid[r][c] {
        b'^' => &[(-1, 0)],
        b'v' => &[(1, 0)],
        b'<' => &[(0, -1)],
        b'>' => &[(0, 1)],
        _ => &[(-1, 0), (1, 0), (0, -1), (0, 1)],
    };

    let nrows = grid.len();
    let ncols = grid[0].len();

    for &(dr, dc) in dirs {
        let nr = r as isize + dr;
        let nc = c as isize + dc;
        if nr >= 0 && nr < nrows as isize && nc >= 0 && nc < ncols as isize {
            let (nr, nc) = (nr as usize, nc as usize);
            if grid[nr][nc] != b'#' && !visited[nr][nc] {
                if let Some(dist) = dfs_part1(nr, nc, end, grid, visited) {
                    max_dist = Some(max_dist.map_or(dist + 1, |m: usize| m.max(dist + 1)));
                }
            }
        }
    }

    visited[r][c] = false;
    max_dist
}

fn build_graph_part2(
    grid: &[&[u8]],
    start: (usize, usize),
    end: (usize, usize),
) -> (usize, usize, Vec<Vec<(usize, usize)>>) {
    let nrows = grid.len();
    let ncols = grid[0].len();

    let mut junctions = vec![start, end];
    for r in 0..nrows {
        for c in 0..ncols {
            if grid[r][c] != b'#' && (r, c) != start && (r, c) != end {
                let mut nbrs = 0;
                for (dr, dc) in [(-1isize, 0isize), (1, 0), (0, -1), (0, 1)] {
                    let nr = r as isize + dr;
                    let nc = c as isize + dc;
                    if nr >= 0
                        && nr < nrows as isize
                        && nc >= 0
                        && nc < ncols as isize
                        && grid[nr as usize][nc as usize] != b'#'
                    {
                        nbrs += 1;
                    }
                }
                if nbrs > 2 {
                    junctions.push((r, c));
                }
            }
        }
    }

    let j_map: HashMap<(usize, usize), usize> = junctions
        .iter()
        .enumerate()
        .map(|(i, &pos)| (pos, i))
        .collect();

    let mut adj = vec![Vec::new(); junctions.len()];

    for (i, &(r, c)) in junctions.iter().enumerate() {
        for (dr, dc) in [(-1isize, 0isize), (1, 0), (0, -1), (0, 1)] {
            let nr = r as isize + dr;
            let nc = c as isize + dc;
            if nr < 0 || nr >= nrows as isize || nc < 0 || nc >= ncols as isize {
                continue;
            }
            if grid[nr as usize][nc as usize] == b'#' {
                continue;
            }

            let mut prev = (r, c);
            let mut curr = (nr as usize, nc as usize);
            let mut dist = 1;

            while !j_map.contains_key(&curr) {
                let mut next_steps = Vec::new();
                for (ddr, ddc) in [(-1isize, 0isize), (1, 0), (0, -1), (0, 1)] {
                    let nnr = curr.0 as isize + ddr;
                    let nnc = curr.1 as isize + ddc;
                    if nnr >= 0 && nnr < nrows as isize && nnc >= 0 && nnc < ncols as isize {
                        let next_pos = (nnr as usize, nnc as usize);
                        if grid[next_pos.0][next_pos.1] != b'#' && next_pos != prev {
                            next_steps.push(next_pos);
                        }
                    }
                }
                if next_steps.is_empty() {
                    break;
                }
                prev = curr;
                curr = next_steps[0];
                dist += 1;
            }

            if let Some(&target_idx) = j_map.get(&curr) {
                adj[i].push((target_idx, dist));
            }
        }
    }

    (0, 1, adj)
}

fn dfs_part2(
    u: usize,
    target: usize,
    visited: u64,
    current_dist: usize,
    best: &mut usize,
    adj: &[Vec<(usize, usize)>],
) {
    if u == target {
        if current_dist > *best {
            *best = current_dist;
        }
        return;
    }

    for &(v, weight) in &adj[u] {
        if visited & (1 << v) == 0 {
            dfs_part2(
                v,
                target,
                visited | (1 << v),
                current_dist + weight,
                best,
                adj,
            );
        }
    }
}

impl AocSolution for Day23 {
    fn part1(&self, input: &str) -> String {
        let grid: Vec<&[u8]> = input
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(|l| l.as_bytes())
            .collect();
        let start = (0, grid[0].iter().position(|&b| b == b'.').unwrap());
        let end = (
            grid.len() - 1,
            grid[grid.len() - 1]
                .iter()
                .position(|&b| b == b'.')
                .unwrap(),
        );
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];

        dfs_part1(start.0, start.1, end, &grid, &mut visited)
            .unwrap()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let grid: Vec<&[u8]> = input
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(|l| l.as_bytes())
            .collect();
        let start = (0, grid[0].iter().position(|&b| b == b'.').unwrap());
        let end = (
            grid.len() - 1,
            grid[grid.len() - 1]
                .iter()
                .position(|&b| b == b'.')
                .unwrap(),
        );

        let (start_idx, end_idx, adj) = build_graph_part2(&grid, start, end);
        let mut best = 0;
        dfs_part2(start_idx, end_idx, 1 << start_idx, 0, &mut best, &adj);

        best.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    #[test]
    fn test_part1_example() {
        assert_eq!(Day23.part1(EXAMPLE), "94");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2023, 23).expect("Failed to get input");
        assert_eq!(Day23.part1(&input), "1998");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(Day23.part2(EXAMPLE), "154");
    }

    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2023, 23).expect("Failed to get input");
        assert_eq!(Day23.part2(&input), "6434");
    }
}
