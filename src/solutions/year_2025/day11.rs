use std::collections::{HashMap, HashSet};

use crate::AocSolution;

type Node = String;
type Connections = HashMap<Node, HashSet<(Node, bool)>>;

fn parse(input: &str, special_nodes: &HashSet<Node>) -> Connections {
    input
        .lines()
        .map(|line| {
            let (key, values) = line.split_once(": ").unwrap();
            let values: HashSet<(Node, bool)> = values
                .split_whitespace()
                .map(|s| {
                    let s: String = s.into();
                    let special = special_nodes.contains(&s);
                    (s, special)
                })
                .collect();
            let key: Node = key.into();
            (key, values)
        })
        .collect()
}

fn count_paths(connections: &Connections, start: &Node) -> u64 {
    let out_node: &Node = &"out".into();
    let mut state_a: HashMap<&Node, u64> = HashMap::new();
    let mut state_b: HashMap<&Node, u64> = HashMap::new();
    let mut steps: u64 = 0;
    state_a.insert(start, 1);
    let mut out_paths = 0;
    loop {
        steps += 1;
        let (current, next) = if steps.is_multiple_of(2) {
            (&mut state_b, &mut state_a)
        } else {
            (&mut state_a, &mut state_b)
        };
        for (&node, npaths) in current.iter() {
            if let Some(next_nodes) = connections.get(node) {
                next_nodes.iter().for_each(|(n, _isspecial)| {
                    if n == out_node {
                        out_paths += npaths;
                    } else {
                        next.entry(n)
                            .and_modify(|current_count| {
                                *current_count += npaths;
                            })
                            .or_insert(*npaths);
                    }
                });
            }
        }
        if next.is_empty() {
            break;
        }
        current.clear();
    }
    out_paths
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct GroupState {
    current: Node,
    has_fft: bool,
    has_dac: bool,
}

impl GroupState {
    fn new(current: Node) -> Self {
        let has_fft = false;
        let has_dac = false;
        GroupState {
            current,
            has_fft,
            has_dac,
        }
    }
    fn add_special(&mut self, special: &str) {
        match special {
            "fft" => {
                self.has_fft = true;
            }
            "dac" => {
                self.has_dac = true;
            }
            _ => (),
        }
    }
    fn all_special(&self) -> bool {
        self.has_fft && self.has_dac
    }
}

fn count_paths_with_groups(connections: &Connections, start: &Node) -> u64 {
    let out_node: &Node = &"out".into();
    let mut state_a: HashMap<GroupState, u64> = HashMap::new();
    let mut state_b: HashMap<GroupState, u64> = HashMap::new();
    let mut steps: u64 = 0;
    state_a.insert(GroupState::new(start.into()), 1);
    let mut out_paths = 0;
    loop {
        steps += 1;
        let (current, next) = if steps.is_multiple_of(2) {
            (&mut state_b, &mut state_a)
        } else {
            (&mut state_a, &mut state_b)
        };
        for (node, npaths) in current.iter() {
            if let Some(next_nodes) = connections.get(&node.current) {
                next_nodes.iter().for_each(|(n, isspecial)| {
                    let mut new_node: GroupState = node.clone();
                    if *isspecial {
                        new_node.add_special(n);
                    }
                    new_node.current = n.to_string();
                    if n == out_node && new_node.all_special() {
                        out_paths += npaths;
                    } else {
                        next.entry(new_node)
                            .and_modify(|current_count| {
                                *current_count += npaths;
                            })
                            .or_insert(*npaths);
                    }
                });
            }
        }
        if next.is_empty() {
            break;
        }
        current.clear();
    }
    out_paths
}

pub struct Day11;

impl AocSolution for Day11 {
    fn part1(&self, input: &str) -> String {
        let special_nodes = HashSet::new();
        let connections = parse(input, &special_nodes);
        let start: &Node = &"you".into();
        count_paths(&connections, start).to_string()
    }

    fn part2(&self, input: &str) -> String {
        let special_nodes: HashSet<Node> =
            [Node::from("fft"), Node::from("dac")].into_iter().collect();
        let connections = parse(input, &special_nodes);
        let start: &Node = &"svr".into();
        count_paths_with_groups(&connections, start).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    const EXAMPLE_2: &str = r"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    #[test]
    fn test_part1_example() {
        assert_eq!(Day11.part1(EXAMPLE), "5");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2025, 11).expect("Failed to get input");
        assert_eq!(Day11.part1(&input), "708");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(Day11.part2(EXAMPLE_2), "2");
    }

    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2025, 11).expect("Failed to get input");
        assert_eq!(Day11.part2(&input), "545394698933400");
    }
}
