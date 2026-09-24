use crate::AocSolution;

pub struct Day25;

/*
// In aoc23, day 25 part 1 was unfinished (attempted with petgraph / kaminpar):
//
// struct AocGraph {
//     graph: Graph<String, String, Undirected>,
// }
//
// impl FromStr for AocGraph {
//     type Err = ();
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         todo!("trying to make a graph like here https://docs.rs/petgraph/latest/petgraph/graph/struct.Graph.html#");
//     }
// }
*/

impl AocSolution for Day25 {
    fn part1(&self, _input: &str) -> String {
        todo!("trying to make a graph like here https://docs.rs/petgraph/latest/petgraph/graph/struct.Graph.html#")
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
    #[ignore = "Unfinished in aoc23"]
    fn test_part1_example() {
        assert_eq!(Day25.part1(EXAMPLE), "54");
    }

    #[test]
    #[ignore = "Unfinished in aoc23"]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2023, 25).expect("Failed to get input");
        assert_eq!(Day25.part1(&input), "531437");
    }

    #[test]
    fn test_part2_full() {
        assert_eq!(Day25.part2(""), "0");
    }
}
