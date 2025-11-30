use std::{
    collections::{btree_map::Entry, BTreeMap, BTreeSet},
    str::FromStr,
};

use crate::AocSolution;

#[derive(Debug)]
struct Rule {
    left: usize,
    right: usize,
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split_once("|");
        if split.is_none() {
            return Err(());
        }
        let (left, right) = split.unwrap();
        let left = left.parse::<usize>();
        if left.is_err() {
            return Err(());
        }
        let left = left.unwrap();
        let right = right.parse::<usize>();
        if right.is_err() {
            return Err(());
        }
        let right = right.unwrap();
        Ok(Rule { left, right })
    }
}
#[derive(Debug, Clone)]
struct Update {
    pages: Vec<usize>,
}
impl FromStr for Update {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pages: Vec<&str> = s.split(",").collect();
        if pages.is_empty() {
            return Err(());
        }
        let pages = pages.iter().map(|p| p.parse::<usize>()).collect::<Vec<_>>();
        if pages.iter().any(|p| p.is_err()) {
            return Err(());
        }
        let pages: Vec<usize> = pages.iter().map(|p| *p.as_ref().unwrap()).collect();
        Ok(Update { pages })
    }
}
impl Update {
    fn check(&self, ruleset: &Ruleset) -> bool {
        let mut seen = BTreeSet::new();
        for page in &self.pages {
            if let Some(rights) = ruleset.rules.get(page) {
                if seen.intersection(rights).count() > 0 {
                    return false;
                }
            }
            seen.insert(*page);
        }
        true
    }
    fn middle(&self) -> usize {
        self.pages[(self.pages.len() - 1) / 2]
    }
    fn reorder(&self, ruleset: &Ruleset) -> Update {
        let mut out = self.clone();
        loop {
            let mut seen = BTreeSet::new();
            let mut locs = BTreeMap::new();
            let mut swap = None;
            for (ipage, page) in out.pages.iter().enumerate() {
                if let Some(rights) = ruleset.rules.get(page) {
                    if let Some(violation) = seen.intersection(rights).next() {
                        swap = Some([ipage, locs[violation]]);
                    }
                }
                if swap.is_some() {
                    break;
                }
                seen.insert(*page);
                locs.insert(page, ipage);
            }
            if let Some([a, b]) = swap {
                out.pages.swap(a, b);
            } else {
                break;
            }
        }
        out
    }
}
#[derive(Debug)]
struct Puzzle {
    rules: Vec<Rule>,
    updates: Vec<Update>,
}

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut get_rules = true;
        let mut rules = Vec::new();
        let mut updates = Vec::new();
        for line in s.lines() {
            if line.is_empty() {
                get_rules = false;
                continue;
            }
            if get_rules {
                rules.push(line.parse::<Rule>()?);
            } else {
                updates.push(line.parse::<Update>()?);
            }
        }
        Ok(Puzzle { rules, updates })
    }
}

#[derive(Debug)]
struct Ruleset {
    rules: BTreeMap<usize, BTreeSet<usize>>,
}
impl Ruleset {
    fn new(rules: &[Rule]) -> Self {
        let mut ruleset: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
        for Rule { left, right } in rules {
            match ruleset.entry(*left) {
                Entry::Vacant(e) => {
                    e.insert(BTreeSet::from([*right]));
                }
                Entry::Occupied(mut e) => {
                    e.get_mut().insert(*right);
                }
            }
        }
        Ruleset { rules: ruleset }
    }
}

impl Puzzle {
    fn process_p1(&self) -> usize {
        let mut out = 0;
        let ruleset = Ruleset::new(&self.rules);
        for update in &self.updates {
            if update.check(&ruleset) {
                out += update.middle();
            }
        }
        out
    }

    fn process_p2(&self) -> usize {
        let mut out = 0;
        let ruleset = Ruleset::new(&self.rules);
        for update in &self.updates {
            if !update.check(&ruleset) {
                out += update.reorder(&ruleset).middle();
            }
        }
        out
    }
}

pub struct Day05;

impl AocSolution for Day05 {
    fn part1(&self, input: &str) -> String {
        let puzzle = input.parse::<Puzzle>().unwrap();
        puzzle.process_p1().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let puzzle = input.parse::<Puzzle>().unwrap();
        puzzle.process_p2().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_part1_example() {
        assert_eq!(Day05.part1(EXAMPLE), "143");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2024, 5).expect("Failed to get input");
        assert_eq!(Day05.part1(&input), "6034");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(Day05.part2(EXAMPLE), "123");
    }

    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2024, 5).expect("Failed to get input");
        assert_eq!(Day05.part2(&input), "6305");
    }
}
