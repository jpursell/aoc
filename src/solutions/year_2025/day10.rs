use std::{collections::HashSet, str::FromStr};

use crate::AocSolution;

type State = Vec<bool>;
type Button = Vec<usize>;

#[derive(Debug)]
struct Machine {
    required: State,
    buttons: Vec<Button>,
}

fn button_press(mut state: State, button: &Button) -> State {
    button.iter().for_each(|&i| state[i] = !state[i]);
    state
}

impl FromStr for Machine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();

        let mut required = parts.next().unwrap().chars();
        assert_eq!(required.next().unwrap(), '[');
        assert_eq!(required.next_back().unwrap(), ']');
        let required = required.map(|c| c == '#').collect();

        let _ = parts.next_back().unwrap();

        let buttons = parts
            .map(|s| {
                let mut chars = s.chars();
                assert_eq!(chars.next().unwrap(), '(');
                assert_eq!(chars.next_back().unwrap(), ')');
                let trimmed: String = chars.collect();
                trimmed.split(",").map(|s| s.parse().unwrap()).collect()
            })
            .collect();
        Ok(Machine { required, buttons })
    }
}

fn parse(input: &str) -> Vec<Machine> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn find_fewest_buttons(machine: &Machine) -> u64 {
    let mut states_a: HashSet<State> = HashSet::new();
    let mut states_b: HashSet<State> = HashSet::new();
    let mut presses: u64 = 0;
    let start: State = machine.required.iter().map(|_| false).collect();
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
                let new_state = button_press(state.clone(), button);
                if new_state == machine.required {
                    return presses;
                }
                next.insert(new_state);
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
            .map(find_fewest_buttons)
            .sum::<u64>()
            .to_string()
    }

    fn part2(&self, _input: &str) -> String {
        "Not implemented".to_string()
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
        let set: HashSet<State> = HashSet::from([vec![false, true]]);
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
        assert_eq!(Day10.part1(&input), "REPLACE_WITH_PART1_FULL_RESULT");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(Day10.part2(EXAMPLE), "REPLACE_WITH_PART2_EXAMPLE_RESULT");
    }

    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2025, 10).expect("Failed to get input");
        assert_eq!(Day10.part2(&input), "REPLACE_WITH_PART2_FULL_RESULT");
    }
}
