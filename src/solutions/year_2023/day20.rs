use std::collections::{HashMap, VecDeque};

use crate::AocSolution;

pub struct Day20;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Pulse {
    Low,
    High,
}

#[derive(Clone, Debug)]
enum ModuleKind {
    Broadcaster,
    FlipFlop(bool),                      // is_on
    Conjunction(HashMap<String, Pulse>), // memory of input pulses
}

#[derive(Clone, Debug)]
struct Module {
    kind: ModuleKind,
    destinations: Vec<String>,
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a / gcd(a, b)) * b
    }
}

fn parse_modules(input: &str) -> HashMap<String, Module> {
    let mut modules = HashMap::new();
    let mut inputs_map: HashMap<String, Vec<String>> = HashMap::new();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let (name_part, dest_part) = line.split_once(" -> ").unwrap();
        let destinations: Vec<String> =
            dest_part.split(',').map(|s| s.trim().to_string()).collect();

        let (name, kind) = if name_part == "broadcaster" {
            ("broadcaster".to_string(), ModuleKind::Broadcaster)
        } else if let Some(n) = name_part.strip_prefix('%') {
            (n.to_string(), ModuleKind::FlipFlop(false))
        } else if let Some(n) = name_part.strip_prefix('&') {
            (n.to_string(), ModuleKind::Conjunction(HashMap::new()))
        } else {
            panic!("Unknown module prefix: {}", name_part);
        };

        for dest in &destinations {
            inputs_map
                .entry(dest.clone())
                .or_default()
                .push(name.clone());
        }

        modules.insert(name, Module { kind, destinations });
    }

    // Initialize conjunction memories with all their inputs
    for (name, module) in modules.iter_mut() {
        if let ModuleKind::Conjunction(ref mut mem) = module.kind {
            if let Some(srcs) = inputs_map.get(name) {
                for src in srcs {
                    mem.insert(src.clone(), Pulse::Low);
                }
            }
        }
    }

    modules
}

impl AocSolution for Day20 {
    fn part1(&self, input: &str) -> String {
        let mut modules = parse_modules(input);
        let mut low_count: u64 = 0;
        let mut high_count: u64 = 0;

        for _ in 0..1000 {
            let mut queue: VecDeque<(String, String, Pulse)> = VecDeque::new();
            queue.push_back(("button".to_string(), "broadcaster".to_string(), Pulse::Low));

            while let Some((src, dest, pulse)) = queue.pop_front() {
                match pulse {
                    Pulse::Low => low_count += 1,
                    Pulse::High => high_count += 1,
                }

                let Some(module) = modules.get_mut(&dest) else {
                    continue;
                };

                match &mut module.kind {
                    ModuleKind::Broadcaster => {
                        for d in &module.destinations {
                            queue.push_back((dest.clone(), d.clone(), pulse));
                        }
                    }
                    ModuleKind::FlipFlop(ref mut is_on) => {
                        if pulse == Pulse::Low {
                            *is_on = !*is_on;
                            let out_pulse = if *is_on { Pulse::High } else { Pulse::Low };
                            for d in &module.destinations {
                                queue.push_back((dest.clone(), d.clone(), out_pulse));
                            }
                        }
                    }
                    ModuleKind::Conjunction(ref mut memory) => {
                        memory.insert(src, pulse);
                        let all_high = memory.values().all(|&p| p == Pulse::High);
                        let out_pulse = if all_high { Pulse::Low } else { Pulse::High };
                        for d in &module.destinations {
                            queue.push_back((dest.clone(), d.clone(), out_pulse));
                        }
                    }
                }
            }
        }

        (low_count * high_count).to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut modules = parse_modules(input);

        // Find module that outputs to "rx"
        let feed = modules
            .iter()
            .find(|(_, m)| m.destinations.contains(&"rx".to_string()))
            .map(|(name, _)| name.clone());

        let Some(feed) = feed else {
            return "0".to_string();
        };

        // Find all modules that output to feed
        let mut feed_inputs: HashMap<String, Option<u64>> = modules
            .iter()
            .filter(|(_, m)| m.destinations.contains(&feed))
            .map(|(name, _)| (name.clone(), None))
            .collect();

        let mut press_count: u64 = 0;

        loop {
            press_count += 1;
            let mut queue: VecDeque<(String, String, Pulse)> = VecDeque::new();
            queue.push_back(("button".to_string(), "broadcaster".to_string(), Pulse::Low));

            while let Some((src, dest, pulse)) = queue.pop_front() {
                if dest == feed && pulse == Pulse::High {
                    if let Some(cycle) = feed_inputs.get_mut(&src) {
                        if cycle.is_none() {
                            *cycle = Some(press_count);
                        }
                    }
                    if feed_inputs.values().all(|v| v.is_some()) {
                        let total_lcm = feed_inputs.values().map(|v| v.unwrap()).fold(1, lcm);
                        return total_lcm.to_string();
                    }
                }

                let Some(module) = modules.get_mut(&dest) else {
                    continue;
                };

                match &mut module.kind {
                    ModuleKind::Broadcaster => {
                        for d in &module.destinations {
                            queue.push_back((dest.clone(), d.clone(), pulse));
                        }
                    }
                    ModuleKind::FlipFlop(ref mut is_on) => {
                        if pulse == Pulse::Low {
                            *is_on = !*is_on;
                            let out_pulse = if *is_on { Pulse::High } else { Pulse::Low };
                            for d in &module.destinations {
                                queue.push_back((dest.clone(), d.clone(), out_pulse));
                            }
                        }
                    }
                    ModuleKind::Conjunction(ref mut memory) => {
                        memory.insert(src, pulse);
                        let all_high = memory.values().all(|&p| p == Pulse::High);
                        let out_pulse = if all_high { Pulse::Low } else { Pulse::High };
                        for d in &module.destinations {
                            queue.push_back((dest.clone(), d.clone(), out_pulse));
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = r"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

    const EXAMPLE_2: &str = r"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

    #[test]
    fn test_part1_example() {
        assert_eq!(Day20.part1(EXAMPLE_1), "32000000");
        assert_eq!(Day20.part1(EXAMPLE_2), "11687500");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2023, 20).expect("Failed to get input");
        assert_eq!(Day20.part1(&input), "681194780");
    }

    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2023, 20).expect("Failed to get input");
        assert_eq!(Day20.part2(&input), "238593356738827");
    }
}
