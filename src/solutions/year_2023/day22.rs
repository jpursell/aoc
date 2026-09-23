use std::collections::{HashSet, VecDeque};

use crate::AocSolution;

pub struct Day22;

#[derive(Clone, Debug)]
struct Brick {
    _id: usize,
    x1: usize,
    x2: usize,
    y1: usize,
    y2: usize,
    z1: usize,
    z2: usize,
}

fn parse_bricks(input: &str) -> Vec<Brick> {
    let mut bricks = Vec::new();
    for (id, line) in input.lines().filter(|l| !l.trim().is_empty()).enumerate() {
        let (p1, p2) = line.trim().split_once('~').unwrap();
        let c1: Vec<usize> = p1.split(',').map(|s| s.parse().unwrap()).collect();
        let c2: Vec<usize> = p2.split(',').map(|s| s.parse().unwrap()).collect();

        bricks.push(Brick {
            _id: id,
            x1: c1[0].min(c2[0]),
            x2: c1[0].max(c2[0]),
            y1: c1[1].min(c2[1]),
            y2: c1[1].max(c2[1]),
            z1: c1[2].min(c2[2]),
            z2: c1[2].max(c2[2]),
        });
    }
    bricks
}

struct Settled {
    supports: Vec<HashSet<usize>>,
    supported_by: Vec<HashSet<usize>>,
    num_bricks: usize,
}

#[allow(clippy::needless_range_loop)]
fn settle(mut bricks: Vec<Brick>) -> Settled {
    bricks.sort_by_key(|b| b.z1);
    let n = bricks.len();

    // Heightmap: (max_z, Option<brick_id>)
    let mut heightmap = [[(0usize, None); 10]; 10];
    let mut supports: Vec<HashSet<usize>> = vec![HashSet::new(); n];
    let mut supported_by: Vec<HashSet<usize>> = vec![HashSet::new(); n];

    for (new_id, brick) in bricks.iter().enumerate() {
        let mut max_z = 0;
        for x in brick.x1..=brick.x2 {
            for y in brick.y1..=brick.y2 {
                max_z = max_z.max(heightmap[x][y].0);
            }
        }

        for x in brick.x1..=brick.x2 {
            for y in brick.y1..=brick.y2 {
                let (h, opt_id) = heightmap[x][y];
                if h == max_z {
                    if let Some(supporter_id) = opt_id {
                        supported_by[new_id].insert(supporter_id);
                        supports[supporter_id].insert(new_id);
                    }
                }
            }
        }

        let brick_height = brick.z2 - brick.z1 + 1;
        let new_top = max_z + brick_height;

        for x in brick.x1..=brick.x2 {
            for y in brick.y1..=brick.y2 {
                heightmap[x][y] = (new_top, Some(new_id));
            }
        }
    }

    Settled {
        supports,
        supported_by,
        num_bricks: n,
    }
}

impl AocSolution for Day22 {
    fn part1(&self, input: &str) -> String {
        let bricks = parse_bricks(input);
        let settled = settle(bricks);

        let count = (0..settled.num_bricks)
            .filter(|&b| {
                settled.supports[b]
                    .iter()
                    .all(|&child| settled.supported_by[child].len() > 1)
            })
            .count();

        count.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let bricks = parse_bricks(input);
        let settled = settle(bricks);

        let mut total_fall = 0;

        for b in 0..settled.num_bricks {
            let mut fallen = HashSet::new();
            let mut queue = VecDeque::new();

            fallen.insert(b);
            queue.push_back(b);

            while let Some(curr) = queue.pop_front() {
                for &child in &settled.supports[curr] {
                    if !fallen.contains(&child)
                        && settled.supported_by[child]
                            .iter()
                            .all(|sup| fallen.contains(sup))
                    {
                        fallen.insert(child);
                        queue.push_back(child);
                    }
                }
            }

            total_fall += fallen.len() - 1;
        }

        total_fall.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

    #[test]
    fn test_part1_example() {
        assert_eq!(Day22.part1(EXAMPLE), "5");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2023, 22).expect("Failed to get input");
        assert_eq!(Day22.part1(&input), "389");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(Day22.part2(EXAMPLE), "7");
    }

    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2023, 22).expect("Failed to get input");
        assert_eq!(Day22.part2(&input), "70609");
    }
}
