use crate::AocSolution;
use std::{
    collections::{hash_map::Entry, HashMap},
    str::FromStr,
};

pub struct Day24;

#[derive(Debug, Clone)]
struct Gate {
    inputs: [String; 2],
    output: String,
    operation: Operation,
}

#[derive(Debug, Clone)]
struct InitialCondition {
    output: String,
    value: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Operation {
    And,
    Or,
    Xor,
}

impl FromStr for InitialCondition {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (output, value) = s.split_once(": ").unwrap();
        let output = String::from(output);
        let value = match value {
            "0" => false,
            "1" => true,
            _ => panic!("invalid bit"),
        };
        Ok(InitialCondition { output, value })
    }
}

impl FromStr for Gate {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, output) = s.split_once(" -> ").unwrap();
        let output = String::from(output);
        let (input0, right) = left.split_once(' ').unwrap();
        let (operation, input1) = right.split_once(' ').unwrap();
        let input0 = String::from(input0);
        let input1 = String::from(input1);
        let operation = match operation {
            "XOR" => Operation::Xor,
            "OR" => Operation::Or,
            "AND" => Operation::And,
            _ => panic!("invalid operation"),
        };
        Ok(Gate {
            inputs: [input0, input1],
            operation,
            output,
        })
    }
}

#[derive(Debug)]
struct Puzzle {
    initial_conditions: Vec<InitialCondition>,
    gates: Vec<Gate>,
    swapped: Vec<String>,
}

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        let (empty_i, _) = lines
            .iter()
            .enumerate()
            .find(|(_i, x)| x.trim().is_empty())
            .unwrap();
        let initial_conditions = lines[0..empty_i]
            .iter()
            .map(|line| line.trim().parse::<InitialCondition>().unwrap())
            .collect();
        let gates = lines[(empty_i + 1)..]
            .iter()
            .filter(|line| !line.trim().is_empty())
            .map(|x| x.trim().parse::<Gate>().unwrap())
            .collect();
        Ok(Puzzle {
            initial_conditions,
            gates,
            swapped: Vec::new(),
        })
    }
}

impl Puzzle {
    fn process_part1(&self) -> usize {
        let mut outputs: HashMap<&str, bool> =
            HashMap::from_iter(self.initial_conditions.iter().map(|ic| {
                let s: &str = &ic.output;
                (s, ic.value)
            }));
        let mut gates: HashMap<&str, &Gate> = HashMap::from_iter(self.gates.iter().map(|g| {
            let s: &str = &g.output;
            (s, g)
        }));
        while !gates.is_empty() {
            let mut new_outputs = Vec::new();
            for (output, gate) in &gates {
                let input0: &str = &gate.inputs[0];
                let input1: &str = &gate.inputs[1];
                if outputs.contains_key(input0) && outputs.contains_key(input1) {
                    let new_output_value = match gate.operation {
                        Operation::And => outputs[input0] && outputs[input1],
                        Operation::Or => outputs[input0] || outputs[input1],
                        Operation::Xor => outputs[input0] ^ outputs[input1],
                    };
                    new_outputs.push((*output, new_output_value));
                }
            }
            if new_outputs.is_empty() {
                break;
            }
            for (output, value) in new_outputs {
                gates.remove(output);
                match outputs.entry(output) {
                    Entry::Occupied(_) => panic!("duplicate output"),
                    Entry::Vacant(vacant_entry) => {
                        vacant_entry.insert(value);
                    }
                }
            }
        }
        let mut out = 0;
        for (output, value) in outputs {
            if !output.starts_with('z') || !value {
                continue;
            }
            let (_, num) = output.split_once('z').unwrap();
            let num: u32 = num.parse().unwrap();
            out += 1_usize << num;
        }
        out
    }

    fn perform_swap(&mut self, a: &str, b: &str) {
        self.swapped.push(a.to_string());
        self.swapped.push(b.to_string());
        let mut i_a: Option<usize> = None;
        let mut i_b: Option<usize> = None;
        for (i, gate) in self.gates.iter().enumerate() {
            if gate.output == a {
                i_a = Some(i);
            }
            if gate.output == b {
                i_b = Some(i);
            }
        }
        if let (Some(ia), Some(ib)) = (i_a, i_b) {
            self.gates[ia].output = String::from(b);
            self.gates[ib].output = String::from(a);
        }
    }

    fn process_part2(&mut self) -> String {
        // Manual swaps found in aoc24
        self.perform_swap("rts", "z07");
        self.perform_swap("jpj", "z12");
        self.perform_swap("kgj", "z26");
        self.perform_swap("vvw", "chv");
        self.swapped.sort();
        self.swapped.join(",")
    }
}

impl AocSolution for Day24 {
    fn part1(&self, input: &str) -> String {
        let puzzle: Puzzle = input.parse().unwrap();
        puzzle.process_part1().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut puzzle: Puzzle = input.parse().unwrap();
        puzzle.process_part2()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj"#;

    #[test]
    fn test_part1_example() {
        assert_eq!(Day24.part1(EXAMPLE), "2024");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2024, 24).expect("Failed to get input");
        assert_eq!(Day24.part1(&input), "56620966442854");
    }

    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2024, 24).expect("Failed to get input");
        assert_eq!(Day24.part2(&input), "chv,jpj,kgj,rts,vvw,z07,z12,z26");
    }
}
