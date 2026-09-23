use std::collections::HashMap;

use crate::AocSolution;

pub struct Day19;

#[derive(Clone, Debug)]
enum Condition {
    Lt(usize, u64),
    Gt(usize, u64),
}

#[derive(Clone, Debug)]
struct Rule {
    cond: Option<Condition>,
    target: String,
}

#[derive(Clone, Debug)]
struct Workflow {
    rules: Vec<Rule>,
}

fn var_to_idx(c: char) -> usize {
    match c {
        'x' => 0,
        'm' => 1,
        'a' => 2,
        's' => 3,
        _ => panic!("Unknown var {}", c),
    }
}

fn parse_rule(s: &str) -> Rule {
    if let Some((cond_str, target)) = s.split_once(':') {
        let var_idx = var_to_idx(cond_str.chars().next().unwrap());
        let op = cond_str.chars().nth(1).unwrap();
        let val: u64 = cond_str[2..].parse().unwrap();
        let cond = match op {
            '<' => Condition::Lt(var_idx, val),
            '>' => Condition::Gt(var_idx, val),
            _ => panic!("Unknown op {}", op),
        };
        Rule {
            cond: Some(cond),
            target: target.to_string(),
        }
    } else {
        Rule {
            cond: None,
            target: s.to_string(),
        }
    }
}

fn parse_workflows(input: &str) -> HashMap<String, Workflow> {
    let mut map = HashMap::new();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('{') {
            continue;
        }
        let (name, rest) = line.split_once('{').unwrap();
        let rules_str = rest.trim_end_matches('}');
        let rules = rules_str.split(',').map(parse_rule).collect();
        map.insert(name.to_string(), Workflow { rules });
    }
    map
}

fn parse_part(line: &str) -> [u64; 4] {
    let trimmed = line.trim().trim_matches(|c| c == '{' || c == '}');
    let mut part = [0u64; 4];
    for item in trimmed.split(',') {
        let (k, v) = item.split_once('=').unwrap();
        let idx = var_to_idx(k.chars().next().unwrap());
        part[idx] = v.parse().unwrap();
    }
    part
}

fn is_accepted(part: &[u64; 4], workflows: &HashMap<String, Workflow>) -> bool {
    let mut current = "in";
    loop {
        if current == "A" {
            return true;
        }
        if current == "R" {
            return false;
        }
        let wf = &workflows[current];
        for rule in &wf.rules {
            let matches = match &rule.cond {
                None => true,
                Some(Condition::Lt(idx, val)) => part[*idx] < *val,
                Some(Condition::Gt(idx, val)) => part[*idx] > *val,
            };
            if matches {
                current = &rule.target;
                break;
            }
        }
    }
}

type Range = [(u64, u64); 4];

fn count_accepted(mut range: Range, current: &str, workflows: &HashMap<String, Workflow>) -> u64 {
    if current == "A" {
        return range
            .iter()
            .map(|(lo, hi)| if lo <= hi { hi - lo + 1 } else { 0 })
            .product();
    }
    if current == "R" {
        return 0;
    }

    let wf = &workflows[current];
    let mut total = 0;

    for rule in &wf.rules {
        match &rule.cond {
            None => {
                total += count_accepted(range, &rule.target, workflows);
                break;
            }
            Some(Condition::Lt(idx, val)) => {
                let (lo, hi) = range[*idx];
                if lo < *val {
                    let mut accepted_range = range;
                    accepted_range[*idx] = (lo, hi.min(*val - 1));
                    total += count_accepted(accepted_range, &rule.target, workflows);
                }
                if hi >= *val {
                    range[*idx] = (lo.max(*val), hi);
                } else {
                    break;
                }
            }
            Some(Condition::Gt(idx, val)) => {
                let (lo, hi) = range[*idx];
                if hi > *val {
                    let mut accepted_range = range;
                    accepted_range[*idx] = (lo.max(*val + 1), hi);
                    total += count_accepted(accepted_range, &rule.target, workflows);
                }
                if lo <= *val {
                    range[*idx] = (lo, hi.min(*val));
                } else {
                    break;
                }
            }
        }
    }

    total
}

impl AocSolution for Day19 {
    fn part1(&self, input: &str) -> String {
        let (wf_part, parts_part) = input.split_once("\n\n").unwrap();
        let workflows = parse_workflows(wf_part);
        let parts: Vec<[u64; 4]> = parts_part
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(parse_part)
            .collect();

        let sum: u64 = parts
            .into_iter()
            .filter(|p| is_accepted(p, &workflows))
            .map(|p| p.iter().sum::<u64>())
            .sum();

        sum.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let wf_part = if let Some((wf_part, _)) = input.split_once("\n\n") {
            wf_part
        } else {
            input
        };
        let workflows = parse_workflows(wf_part);
        let initial_range: Range = [(1, 4000); 4];

        count_accepted(initial_range, "in", &workflows).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn test_part1_example() {
        assert_eq!(Day19.part1(EXAMPLE), "19114");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2023, 19).expect("Failed to get input");
        assert_eq!(Day19.part1(&input), "532551");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(Day19.part2(EXAMPLE), "167409079868000");
    }

    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2023, 19).expect("Failed to get input");
        assert_eq!(Day19.part2(&input), "134343280273968");
    }
}
