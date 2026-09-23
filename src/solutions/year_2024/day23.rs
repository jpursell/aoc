use crate::AocSolution;
use itertools::Itertools;
use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    str::FromStr,
};

pub struct Day23;

#[derive(Debug)]
struct Connection {
    nodes: [String; 2],
}

impl FromStr for Connection {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once('-').unwrap();
        Ok(Connection {
            nodes: [a.into(), b.into()],
        })
    }
}

#[derive(Debug)]
struct Puzzle {
    connections: Vec<Connection>,
}

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let connections = s
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.trim().parse::<Connection>().unwrap())
            .collect();
        Ok(Puzzle { connections })
    }
}

impl Puzzle {
    fn make_connection_map(&self) -> HashMap<&str, HashSet<&str>> {
        let mut connection_map: HashMap<&str, HashSet<&str>> = HashMap::new();
        for connection in &self.connections {
            let source: &str = &connection.nodes[0];
            let dest: &str = &connection.nodes[1];
            let mut insert_connection = |source, dest| match connection_map.entry(source) {
                Entry::Occupied(mut occupied_entry) => {
                    occupied_entry.get_mut().insert(dest);
                }
                Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(HashSet::from([dest]));
                }
            };
            insert_connection(source, dest);
            insert_connection(dest, source);
        }
        connection_map
    }

    fn part1(&self) -> usize {
        let connection_map = self.make_connection_map();
        let mut trios: HashSet<[&str; 3]> = HashSet::new();
        for (&node, connections) in &connection_map {
            for others in connections.iter().combinations(2) {
                if !connection_map[others[0]].contains(others[1]) {
                    continue;
                }
                let mut trio = [node, *others[0], *others[1]];
                trio.sort();
                trios.insert(trio);
            }
        }
        trios
            .iter()
            .filter(|trio| trio.iter().any(|node| node.starts_with('t')))
            .count()
    }

    fn find_clique_of_size(
        &self,
        connection_map: &HashMap<&str, HashSet<&str>>,
        size: usize,
    ) -> Option<String> {
        for (key, connections) in connection_map {
            if connections.len() + 1 < size {
                continue;
            }
            'combo: for combination in connections.iter().combinations(size - 1) {
                for connection in combination.iter().combinations(2) {
                    if !connection_map[**connection[0]].contains(**connection[1]) {
                        continue 'combo;
                    }
                }
                let mut nodes: Vec<&str> = Vec::with_capacity(size);
                nodes.push(key);
                combination.iter().for_each(|x| nodes.push(**x));
                nodes.sort();
                return Some(nodes.join(","));
            }
        }
        None
    }

    fn part2(&self) -> String {
        let connection_map = self.make_connection_map();
        let max_degree = connection_map.values().map(|v| v.len()).max().unwrap_or(0);
        for size in (1..=max_degree + 1).rev() {
            if let Some(clique) = self.find_clique_of_size(&connection_map, size) {
                return clique;
            }
        }
        String::new()
    }
}

impl AocSolution for Day23 {
    fn part1(&self, input: &str) -> String {
        let puzzle: Puzzle = input.parse().unwrap();
        puzzle.part1().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let puzzle: Puzzle = input.parse().unwrap();
        puzzle.part2()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn"#;

    #[test]
    fn test_part1_example() {
        assert_eq!(Day23.part1(EXAMPLE), "7");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2024, 23).expect("Failed to get input");
        assert_eq!(Day23.part1(&input), "1083");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(Day23.part2(EXAMPLE), "co,de,ka,ta");
    }

    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2024, 23).expect("Failed to get input");
        assert_eq!(
            Day23.part2(&input),
            "as,bu,cp,dj,ez,fd,hu,it,kj,nx,pp,xh,yu"
        );
    }
}
