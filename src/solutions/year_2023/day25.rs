use std::collections::{HashMap, HashSet, VecDeque};

use crate::AocSolution;

pub struct Day25;

fn parse_graph(input: &str) -> (usize, Vec<Vec<usize>>) {
    let mut name_to_id = HashMap::new();
    let mut get_id = |name: &str| {
        let next_id = name_to_id.len();
        *name_to_id.entry(name.to_string()).or_insert(next_id)
    };

    let mut edges = Vec::new();
    for line in input.lines().filter(|l| !l.trim().is_empty()) {
        let (u_str, vs_str) = line.trim().split_once(':').unwrap();
        let u = get_id(u_str.trim());
        for v_str in vs_str.split_whitespace() {
            let v = get_id(v_str.trim());
            edges.push((u, v));
        }
    }

    let num_vertices = name_to_id.len();
    let mut adj = vec![Vec::new(); num_vertices];
    for (u, v) in edges {
        adj[u].push(v);
        adj[v].push(u);
    }

    (num_vertices, adj)
}

fn solve_min_cut(num_vertices: usize, adj: &[Vec<usize>]) -> usize {
    let s = 0;

    for t in 1..num_vertices {
        let mut capacity: HashMap<(usize, usize), i32> = HashMap::new();
        for (u, neighbors) in adj.iter().enumerate().take(num_vertices) {
            for &v in neighbors {
                capacity.insert((u, v), 1);
            }
        }

        let mut flow = 0;
        while flow <= 3 {
            // BFS for augmenting path
            let mut parent: HashMap<usize, usize> = HashMap::new();
            let mut queue = VecDeque::new();
            queue.push_back(s);

            while let Some(curr) = queue.pop_front() {
                if curr == t {
                    break;
                }
                for &nxt in &adj[curr] {
                    if nxt != s
                        && !parent.contains_key(&nxt)
                        && *capacity.get(&(curr, nxt)).unwrap_or(&0) > 0
                    {
                        parent.insert(nxt, curr);
                        queue.push_back(nxt);
                    }
                }
            }

            if !parent.contains_key(&t) {
                break;
            }

            flow += 1;
            let mut curr = t;
            while curr != s {
                let p = parent[&curr];
                *capacity.get_mut(&(p, curr)).unwrap() -= 1;
                *capacity.entry((curr, p)).or_insert(0) += 1;
                curr = p;
            }
        }

        if flow == 3 {
            // Find component reachable from s in residual graph
            let mut visited = HashSet::new();
            let mut queue = VecDeque::new();
            visited.insert(s);
            queue.push_back(s);

            while let Some(curr) = queue.pop_front() {
                for &nxt in &adj[curr] {
                    if !visited.contains(&nxt) && *capacity.get(&(curr, nxt)).unwrap_or(&0) > 0 {
                        visited.insert(nxt);
                        queue.push_back(nxt);
                    }
                }
            }

            let c1 = visited.len();
            let c2 = num_vertices - c1;
            return c1 * c2;
        }
    }

    panic!("Min cut not found");
}

impl AocSolution for Day25 {
    fn part1(&self, input: &str) -> String {
        let (num_vertices, adj) = parse_graph(input);
        solve_min_cut(num_vertices, &adj).to_string()
    }

    fn part2(&self, _input: &str) -> String {
        "0".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

    #[test]
    fn test_part1_example() {
        assert_eq!(Day25.part1(EXAMPLE), "54");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2023, 25).expect("Failed to get input");
        assert_eq!(Day25.part1(&input), "531437");
    }

    #[test]
    fn test_part2_full() {
        assert_eq!(Day25.part2(""), "0");
    }
}
