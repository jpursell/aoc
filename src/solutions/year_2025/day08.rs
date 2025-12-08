use std::collections::BTreeSet;

use ndarray::prelude::*;
struct Node {
    pos: [u16; 3],
}
struct Connection {
    indices: [usize; 2],
}
fn make_node_list(input: &str) -> Vec<Node> {
    input
        .lines()
        .map(|line| {
            let vec: Vec<u16> = line
                .split(",")
                .map(|s| s.parse().expect("Count not parse node position"))
                .collect();
            Node {
                pos: [vec[0], vec[1], vec[2]],
            }
        })
        .collect()
}
fn compute_sq_distance(a: &Node, b: &Node) -> f32 {
    a.pos
        .iter()
        .zip(b.pos.iter())
        .map(|(a, b)| (*a as f32 - *b as f32).powi(2))
        .sum()
}
fn find_sq_distance_map(nodes: &[Node]) -> Array2<f32> {
    let mut dmap = Array2::zeros((nodes.len(), nodes.len()));
    for (ia, a) in nodes.iter().enumerate() {
        for (ib, b) in nodes.iter().enumerate().skip(ia + 1) {
            let dist: f32 = compute_sq_distance(a, b);
            *(dmap.get_mut([ia, ib]).unwrap()) = dist;
        }
    }
    dmap
}
fn find_closest_connections(nodes: &[Node], n: usize) -> Vec<Connection> {
    let dmap = find_sq_distance_map(nodes);
    let mut distances: Vec<((usize, usize), &f32)> = dmap
        .indexed_iter()
        .filter(|(_pos, dist)| **dist > 0.0)
        .collect();
    distances.sort_by(|a, b| a.1.total_cmp(b.1));
    distances
        .iter()
        .take(n)
        .map(|(pos, _dist)| Connection {
            indices: [pos.0, pos.1],
        })
        .collect()
}

fn find_connected_groups(connections: &[Connection]) -> Vec<BTreeSet<usize>> {
    let mut groups: Vec<BTreeSet<usize>> = Vec::new();
    for connection in connections {
        let [a, b] = &connection.indices;
        let a_group = groups
            .iter()
            .enumerate()
            .find(|g| g.1.contains(a))
            .map(|g| g.0);
        let b_group = groups
            .iter()
            .enumerate()
            .find(|g| g.1.contains(b))
            .map(|g| g.0);
        match (a_group, b_group) {
            (None, None) => {
                // add both nodes to a new group
                groups.push(BTreeSet::from_iter(connection.indices.iter().copied()));
            }
            (None, Some(bg)) => {
                // add unconnected node to existing group
                groups[bg].insert(*a);
            }
            (Some(ag), None) => {
                // add unconnected node to existing group
                groups[ag].insert(*b);
            }
            (Some(ag), Some(bg)) => {
                // merge groups
                let mut b_group = groups.swap_remove(bg);
                groups[ag].append(&mut b_group);
            }
        }
    }
    groups
}
fn part1(input: &str, n: usize) -> String {
    let nodes = make_node_list(input);
    let connections = find_closest_connections(&nodes, n);
    let groups = find_connected_groups(&connections);
    groups
        .iter()
        .map(|g| g.len())
        .product::<usize>()
        .to_string()
}
use crate::AocSolution;

pub struct Day08;

impl AocSolution for Day08 {
    fn part1(&self, input: &str) -> String {
        part1(input, 1000)
    }

    fn part2(&self, _input: &str) -> String {
        "Not implemented".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE, 10), "40");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2025, 8).expect("Failed to get input");
        assert_eq!(Day08.part1(&input), "REPLACE_WITH_PART1_FULL_RESULT");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(Day08.part2(EXAMPLE), "REPLACE_WITH_PART2_EXAMPLE_RESULT");
    }

    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2025, 8).expect("Failed to get input");
        assert_eq!(Day08.part2(&input), "REPLACE_WITH_PART2_FULL_RESULT");
    }
}
