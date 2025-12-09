use std::{collections::BTreeSet, fmt::Display};

#[derive(Debug)]
struct Node {
    pos: [u32; 3],
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}, {}]", self.pos[0], self.pos[1], self.pos[2])
    }
}

#[derive(Debug)]
struct Connection {
    indices: [usize; 2],
}

fn make_node_list(input: &str) -> Vec<Node> {
    input
        .lines()
        .map(|line| {
            let vec: Vec<u32> = line
                .split(",")
                .map(|s| s.parse().expect("Count not parse node position"))
                .collect();
            Node {
                pos: [vec[0], vec[1], vec[2]],
            }
        })
        .collect()
}

fn compute_sq_distance(a: &Node, b: &Node) -> f64 {
    a.pos
        .iter()
        .zip(b.pos.iter())
        .map(|(a, b)| (*a as f64 - *b as f64).powi(2))
        .sum()
}

fn find_sq_distance_map(nodes: &[Node]) -> Vec<((usize, usize), f64)> {
    let mut distances = Vec::with_capacity(nodes.len() * (nodes.len() - 1) / 2);
    for (ia, a) in nodes.iter().enumerate() {
        for (ib, b) in nodes.iter().enumerate().skip(ia + 1) {
            let dist: f64 = compute_sq_distance(a, b);
            distances.push(((ia, ib), dist));
        }
    }
    distances
}

fn find_closest_connections(nodes: &[Node], n: Option<usize>) -> Vec<Connection> {
    let mut distances = find_sq_distance_map(nodes);
    distances.sort_by(|a, b| a.1.total_cmp(&b.1));
    let distances = match n {
        Some(n) => &distances[..n],
        None => &distances,
    };
    distances
        .iter()
        .map(|(pos, _dist)| Connection {
            indices: [pos.0, pos.1],
        })
        .collect()
}

fn find_connected_groups(connections: &[Connection], nodes: &[Node]) -> Vec<BTreeSet<usize>> {
    let nnodes = nodes.len();
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
                // don't do anything if already in same group
                if ag != bg {
                    // merge groups
                    let mut b_group = groups.swap_remove(bg);
                    let ag = groups
                        .iter()
                        .enumerate()
                        .find(|g| g.1.contains(a))
                        .map(|g| g.0)
                        .unwrap();
                    groups[ag].append(&mut b_group);
                }
            }
        }
        if groups.len() == 1 && groups[0].len() == nnodes {
            return vec![BTreeSet::from([*a, *b])];
        }
    }
    groups
}
fn part1(input: &str, n: usize) -> String {
    let nodes = make_node_list(input);
    let connections = find_closest_connections(&nodes, Some(n));
    let groups = find_connected_groups(&connections, &nodes);
    let mut group_sizes: Vec<usize> = groups.iter().map(|g| g.len()).collect();
    group_sizes.sort();
    group_sizes
        .iter()
        .rev()
        .take(3)
        .product::<usize>()
        .to_string()
}
fn part2(input: &str) -> String {
    let nodes = make_node_list(input);
    let connections = find_closest_connections(&nodes, None);
    let groups = find_connected_groups(&connections, &nodes);
    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0].len(), 2);
    let indices: Vec<&usize> = groups[0].iter().collect();
    assert_eq!(indices.len(), 2);
    let node_a = &nodes[*indices[0]];
    let node_b = &nodes[*indices[1]];
    (node_a.pos[0] as u64 * node_b.pos[0] as u64).to_string()
}
use crate::AocSolution;

pub struct Day08;

impl AocSolution for Day08 {
    fn part1(&self, input: &str) -> String {
        part1(input, 1000)
    }

    fn part2(&self, input: &str) -> String {
        part2(input)
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
        assert_eq!(Day08.part1(&input), "75680");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(Day08.part2(EXAMPLE), "25272");
    }

    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2025, 8).expect("Failed to get input");
        assert_eq!(Day08.part2(&input), "8995844880");
    }
}
