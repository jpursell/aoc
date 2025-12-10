use std::{collections::HashSet, str::FromStr};

use crate::AocSolution;

type Indicator = Vec<bool>;
type Button = Vec<usize>;
type Joltage = Vec<u16>;

#[derive(Debug)]
struct Machine {
    required: Indicator,
    buttons: Vec<Button>,
    joltage: Joltage,
}

fn button_press_indicator(mut state: Indicator, button: &Button) -> Indicator {
    button.iter().for_each(|&i| state[i] = !state[i]);
    state
}
fn button_press_joltage(mut joltage: Joltage, button: &Button) -> Joltage {
    button.iter().for_each(|&i| joltage[i] += 1);
    joltage
}

impl FromStr for Machine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();

        let mut required = parts.next().unwrap().chars();
        assert_eq!(required.next().unwrap(), '[');
        assert_eq!(required.next_back().unwrap(), ']');
        let required = required.map(|c| c == '#').collect();

        let mut joltage = parts.next_back().unwrap().chars();
        assert_eq!(joltage.next().unwrap(), '{');
        assert_eq!(joltage.next_back().unwrap(), '}');
        let joltage: String = joltage.collect();
        let joltage = joltage.split(",").map(|s| s.parse().unwrap()).collect();

        let buttons = parts
            .map(|s| {
                let mut chars = s.chars();
                assert_eq!(chars.next().unwrap(), '(');
                assert_eq!(chars.next_back().unwrap(), ')');
                let trimmed: String = chars.collect();
                trimmed.split(",").map(|s| s.parse().unwrap()).collect()
            })
            .collect();
        Ok(Machine {
            required,
            buttons,
            joltage,
        })
    }
}

fn parse(input: &str) -> Vec<Machine> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn find_fewest_buttons_indicator_lights(machine: &Machine) -> u64 {
    let mut states_a: HashSet<Indicator> = HashSet::new();
    let mut states_b: HashSet<Indicator> = HashSet::new();
    let mut presses: u64 = 0;
    let start: Indicator = machine.required.iter().map(|_| false).collect();
    states_a.insert(start);
    loop {
        presses += 1;
        let (current, next) = if presses.is_multiple_of(2) {
            (&mut states_b, &mut states_a)
        } else {
            (&mut states_a, &mut states_b)
        };
        for button in machine.buttons.iter() {
            for state in current.iter() {
                let new_state = button_press_indicator(state.clone(), button);
                if new_state == machine.required {
                    return presses;
                }
                next.insert(new_state);
            }
        }
        current.clear();
    }
}

fn find_fewest_buttons_joltage(machine: &Machine) -> u64 {
    let mut joltages_a: HashSet<Joltage> = HashSet::new();
    let mut joltages_b: HashSet<Joltage> = HashSet::new();
    let mut presses: u64 = 0;
    let start: Joltage = machine.required.iter().map(|_| 0).collect();
    joltages_a.insert(start);
    loop {
        presses += 1;
        let (current, next) = if presses.is_multiple_of(2) {
            (&mut joltages_b, &mut joltages_a)
        } else {
            (&mut joltages_a, &mut joltages_b)
        };
        for button in machine.buttons.iter() {
            for joltage in current.iter() {
                let new_joltage = button_press_joltage(joltage.clone(), button);
                if new_joltage == machine.joltage {
                    return presses;
                }
                if new_joltage
                    .iter()
                    .zip(machine.joltage.iter())
                    .any(|(new, desired)| new > desired)
                {
                    continue;
                }
                next.insert(new_joltage);
            }
        }
        current.clear();
    }
}

pub struct Day10;

impl AocSolution for Day10 {
    fn part1(&self, input: &str) -> String {
        parse(input)
            .iter()
            .map(find_fewest_buttons_indicator_lights)
            .sum::<u64>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        parse(input)
            .iter()
            .map(find_fewest_buttons_joltage)
            .sum::<u64>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test_hash_set_of_vecs() {
        let set: HashSet<Indicator> = HashSet::from([vec![false, true]]);
        assert!(set.contains(&vec![false, true]));
        assert!(!set.contains(&vec![false, false]));
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(Day10.part1(EXAMPLE), "7");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2025, 10).expect("Failed to get input");
        assert_eq!(Day10.part1(&input), "419");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(Day10.part2(EXAMPLE), "33");
    }

    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2025, 10).expect("Failed to get input");
        assert_eq!(Day10.part2(&input), "REPLACE_WITH_PART2_FULL_RESULT");
    }
}
