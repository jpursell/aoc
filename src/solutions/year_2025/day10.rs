use itertools::Itertools;
use std::{
    collections::{BTreeMap, HashSet},
    str::FromStr,
    time::Instant,
};

use crate::AocSolution;

type Indicator = Vec<u16>;
type Button = Vec<usize>;
type Joltage = Vec<u16>;

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

type ButtonCombos = BTreeMap<Indicator, BTreeMap<Joltage, u16>>;
fn map_button_combinations(buttons: &[Button]) -> ButtonCombos {
    let max_index: u16 = buttons
        .iter()
        .map(|b| b.iter().copied().max().unwrap())
        .max()
        .unwrap() as u16
        + 1;
    let mut out = BTreeMap::new();
    for npressed in 1..=buttons.len() {
        for combo in buttons.iter().combinations(npressed) {
            let mut result: Vec<u16> = vec![0; max_index as usize];
            for button in combo {
                for index in button {
                    result[*index] += 1;
                }
            }
            let parity: Indicator = joltage_to_parity(&result);
            let npressed = npressed as u16;
            out.entry(parity)
                .and_modify(|v: &mut BTreeMap<Vec<u16>, u16>| {
                    v.entry(result.clone())
                        .and_modify(|x: &mut u16| *x = npressed.min(*x))
                        .or_insert(npressed);
                })
                .or_insert(BTreeMap::from([(result, npressed)]));
        }
    }
    out
}

const LARGE: u64 = 1_000_000;

fn find_fewest_buttons_joltage(
    joltage: &Joltage,
    combos: &ButtonCombos,
    cache: &mut BTreeMap<Joltage, u64>,
) -> u64 {
    if joltage.iter().all(|&j| j == 0) {
        return 0;
    }
    if let Some(&cached_value) = cache.get(joltage) {
        return cached_value;
    }
    let parity: Indicator = joltage_to_parity(joltage);

    let value = if !combos.contains_key(&parity) {
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
                    dbg!(&joltage);
                    dbg!(&parity);
                    dbg!(&result);
                    dbg!(&presses);
                    let presses = *presses as u64;
                    find_fewest_buttons_joltage(&new_joltage, combos, cache) * 2 + presses
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
        let start = Instant::now();
        parse(input)
            .iter()
            .inspect(|m| {
                eprintln!(
                    "t:{} Working on machine {:?}",
                    start.elapsed().as_secs_f32(),
                    m
                )
            })
            .map(|m| {
                let combos = map_button_combinations(&m.buttons);
                dbg!(&combos);
                let mut cache = BTreeMap::new();
                find_fewest_buttons_joltage(&m.joltage, &combos, &mut cache)
            })
            .inspect(|x| eprintln!("got {x}"))
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
        map_button_combinations(&machines[0].buttons);
    }

    #[test]
    fn test_part2_basic_solve() {
        let joltage: Joltage = vec![2, 2, 2];
        let buttons: [Button; 3] = [vec![1, 2], vec![0, 2], vec![0, 1]];
        let combos = map_button_combinations(&buttons);
        let mut cache = BTreeMap::new();
        let presses: u64 = find_fewest_buttons_joltage(&joltage, &combos, &mut cache);
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
        assert_eq!(Day10.part2(&input), "REPLACE_WITH_PART2_FULL_RESULT");
    }
}
