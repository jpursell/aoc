use itertools::Itertools;
use std::{
    collections::{BTreeMap, HashSet},
    str::FromStr,
};

use crate::AocSolution;

type Indicator = Vec<bool>;
type Button = Vec<usize>;
type Joltage = Vec<u8>;

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
        let joltage: Vec<_> = joltage.split(",").map(|s| s.parse().unwrap()).collect();
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

fn find_fewest_buttons_joltage(buttons: &[Button], joltage: Joltage) -> u64 {
    // machine
    // req    buttons                         joltage
    // [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
    //
    // the variables would be how much of each button
    //
    // the output is the sum of the button pushes
    0
}

fn map_button_combinations(buttons: &[Button]) -> BTreeMap<Vec<bool>, BTreeMap<Vec<u8>, u8>> {
    let max_index: u8 = buttons
        .iter()
        .map(|b| b.iter().copied().max().unwrap())
        .max()
        .unwrap() as u8
        + 1;
    let mut out = BTreeMap::new();
    for npressed in 1..=buttons.len() {
        for combo in buttons.iter().combinations(npressed) {
            let mut result: Vec<u8> = vec![0; max_index as usize];
            for button in combo {
                for index in button {
                    result[*index] += 1;
                }
            }
            let parity: Vec<bool> = result.iter().map(|&x| x % 2 == 0).collect();
            let npressed = npressed as u8;
            out.entry(parity)
                .and_modify(|v: &mut BTreeMap<Vec<u8>, u8>| {
                    v.entry(result.clone())
                        .and_modify(|x: &mut u8| *x = npressed.min(*x))
                        .or_insert(npressed);
                })
                .or_insert(BTreeMap::from([(result, npressed)]));
        }
    }
    out
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

    fn part2(&self, _input: &str) -> String {
        // parse(input)
        //     .iter()
        //     .map(find_fewest_buttons_joltage)
        //     .sum::<u64>()
        //     .to_string()
        "not implemented".into()
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
    fn test_map_button_combinations() {
        let machines = parse(EXAMPLE);
        dbg!(&machines[0].buttons);
        let map = map_button_combinations(&machines[0].buttons);
        dbg!(map);
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
