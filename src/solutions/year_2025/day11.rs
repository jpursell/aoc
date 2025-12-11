use std::collections::{HashMap, HashSet};

use crate::AocSolution;

type Node = String;
type Connections = HashMap<Node, HashSet<Node>>;

fn parse(input: &str) -> Connections {
    input
        .lines()
        .map(|line| {
            let (key, values) = line.split_once(": ").unwrap();
            let values: HashSet<Node> = values.split_whitespace().map(|s| s.into()).collect();
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
                next_nodes.iter().for_each(|n| {
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

pub struct Day11;

impl AocSolution for Day11 {
    fn part1(&self, input: &str) -> String {
        let connections = parse(input);
        let start: &Node = &"you".into();
        count_paths(&connections, start).to_string()
    }

    fn part2(&self, input: &str) -> String {
        let connections = parse(input);
        let start: &Node = &"svr".into();
        count_paths(&connections, start).to_string()
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
        assert_eq!(Day11.part2(&input), "REPLACE_WITH_PART2_FULL_RESULT");
    }
}
