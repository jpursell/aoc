use itertools::Itertools;
use std::{
    collections::{BTreeMap, HashSet},
    str::FromStr,
};

use crate::AocSolution;

type Indicator = Vec<u16>;
type Button = Vec<usize>;
type Joltage = Vec<u16>;
type Presses = u64;

const LARGE: u64 = 1_000_000;
const ZERO: u64 = 0;

#[derive(Debug)]
struct Machine {
    required: Indicator,
    buttons: Vec<Button>,
    joltage: Joltage,
}

fn button_press_indicator(mut state: Indicator, button: &Button) -> Indicator {
    button.iter().for_each(|&i| state[i] = (state[i] + 1) % 2);
    state
}

impl FromStr for Machine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();

        let mut required = parts.next().unwrap().chars();
        assert_eq!(required.next().unwrap(), '[');
        assert_eq!(required.next_back().unwrap(), ']');
        let required = required.map(|c| if c == '#' { 1 } else { 0 }).collect();

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
    let start: Indicator = machine.required.iter().map(|_| 0).collect();
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

fn joltage_to_parity(joltage: &Joltage) -> Indicator {
    joltage.iter().map(|&x| x % 2).collect()
}

type ButtonCombos = BTreeMap<Indicator, BTreeMap<Joltage, Presses>>;
fn map_button_combinations(buttons: &[Button]) -> ButtonCombos {
    let max_index: u16 = buttons
        .iter()
        .map(|b| b.iter().copied().max().unwrap())
        .max()
        .unwrap() as u16
        + 1;
    let mut out = BTreeMap::new();
    for npressed in 0..=buttons.len() {
        for combo in (0..buttons.len()).combinations(npressed) {
            let mut result: Vec<u16> = vec![0; max_index as usize];
            let mut by_index: Vec<u64> = vec![0; buttons.len()];
            for button_index in &combo {
                let button = &buttons[*button_index];
                for index in button {
                    result[*index] += 1;
                }
                by_index[*button_index] += 1;
            }
            let parity: Indicator = joltage_to_parity(&result);
            let npressed: Presses = npressed as u64;
            out.entry(parity)
                .and_modify(|v: &mut BTreeMap<Joltage, Presses>| {
                    v.entry(result.clone())
                        // not needed because smaller npresses inserted first .and_modify(|x: &mut Presses| *x = npressed.clone().min(x.clone()))
                        .or_insert(npressed);
                })
                .or_insert(BTreeMap::from([(result, npressed)]));
        }
    }
    out
}

fn find_fewest_buttons_joltage(
    joltage: &Joltage,
    combos: &ButtonCombos,
    cache: &mut BTreeMap<Joltage, Presses>,
) -> Presses {
    if joltage.iter().all(|&j| j == 0) {
        return ZERO;
    }
    if let Some(cached_value) = cache.get(joltage) {
        return *cached_value;
    }
    let parity: Indicator = joltage_to_parity(joltage);

    let value: Presses = if !combos.contains_key(&parity) {
        LARGE
    } else {
        combos[&parity]
            .iter()
            .map(|(result, presses)| {
                if joltage.iter().zip(result.iter()).any(|(j, r)| j < r) {
                    LARGE
                } else {
                    let new_joltage: Joltage = joltage
                        .iter()
                        .zip(result.iter())
                        .map(|(j, r)| {
                            let d = j - r;
                            assert_eq!(d % 2, 0);
                            d / 2
                        })
                        .collect();
                    let new_answer = find_fewest_buttons_joltage(&new_joltage, combos, cache);
                    new_answer * 2 + *presses
                }
            })
            .min()
            .unwrap()
    };
    cache.insert(joltage.clone(), value);
    value
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
            .map(|m| {
                let combos = map_button_combinations(&m.buttons);
                let mut cache = BTreeMap::new();
                find_fewest_buttons_joltage(&m.joltage, &combos, &mut cache)
            })
            .sum::<Presses>()
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
    fn test_part1_example() {
        assert_eq!(Day10.part1(EXAMPLE), "7");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2025, 10).expect("Failed to get input");
        assert_eq!(Day10.part1(&input), "419");
    }

    #[test]
    fn test_part2_basic_solve() {
        let joltage: Joltage = vec![2, 2, 2];
        let buttons: [Button; 3] = [vec![1, 2], vec![0, 2], vec![0, 1]];
        let combos = map_button_combinations(&buttons);
        let mut cache = BTreeMap::new();
        let presses: Presses = find_fewest_buttons_joltage(&joltage, &combos, &mut cache);
        assert_eq!(presses, 3);
    }

    #[test]
    fn test_part2_example_1() {
        assert_eq!(
            Day10.part2("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}"),
            "10"
        );
    }

    #[test]
    fn test_part2_example_2() {
        assert_eq!(
            Day10.part2("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}"),
            "12"
        );
    }

    #[test]
    fn test_part2_example_3() {
        assert_eq!(
            Day10.part2("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"),
            "11"
        );
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(Day10.part2(EXAMPLE), "33");
    }

    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2025, 10).expect("Failed to get input");
        assert_eq!(Day10.part2(&input), "18369");
    }
}
