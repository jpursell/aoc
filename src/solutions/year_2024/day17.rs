use crate::AocSolution;
use std::str::FromStr;

pub struct Day17;

#[derive(Debug)]
enum Instruction {
    Adv(usize),
    Bxl(usize),
    Bst(usize),
    Jnz(usize),
    Bxc(usize),
    Out(usize),
    Bdv(usize),
    Cdv(usize),
}

impl Instruction {
    fn new(opcode: usize, operand: usize) -> Self {
        match opcode {
            0 => Instruction::Adv(operand),
            1 => Instruction::Bxl(operand),
            2 => Instruction::Bst(operand),
            3 => Instruction::Jnz(operand),
            4 => Instruction::Bxc(operand),
            5 => Instruction::Out(operand),
            6 => Instruction::Bdv(operand),
            7 => Instruction::Cdv(operand),
            _ => panic!(),
        }
    }
}

struct Computer {
    registers: [usize; 3],
    instruction_pointer: usize,
}

impl Computer {
    fn run_instruction(&mut self, program: &[usize]) -> Option<usize> {
        let instruction = Instruction::new(
            program[self.instruction_pointer],
            program[self.instruction_pointer + 1],
        );
        self.instruction_pointer += 2;
        match instruction {
            Instruction::Adv(x) => self.adv(x),
            Instruction::Bxl(x) => self.bxl(x),
            Instruction::Bst(x) => self.bst(x),
            Instruction::Jnz(x) => self.jnz(x),
            Instruction::Bxc(x) => self.bxc(x),
            Instruction::Out(x) => self.out(x),
            Instruction::Bdv(x) => self.bdv(x),
            Instruction::Cdv(x) => self.cdv(x),
        }
    }

    fn resolve_combo(&self, combo: usize) -> usize {
        match combo {
            0..=3 => combo,
            4..=6 => self.registers[combo - 4],
            _ => panic!(),
        }
    }

    fn adv(&mut self, combo: usize) -> Option<usize> {
        let combo: usize = self.resolve_combo(combo);
        self.registers[0] /= 2_usize.pow(combo as u32);
        None
    }

    fn bxl(&mut self, literal: usize) -> Option<usize> {
        self.registers[1] ^= literal;
        None
    }

    fn bst(&mut self, combo: usize) -> Option<usize> {
        let combo: usize = self.resolve_combo(combo);
        self.registers[1] = combo % 8;
        None
    }

    fn jnz(&mut self, literal: usize) -> Option<usize> {
        if self.registers[0] != 0 {
            self.instruction_pointer = literal;
        }
        None
    }

    fn bxc(&mut self, _literal: usize) -> Option<usize> {
        self.registers[1] ^= self.registers[2];
        None
    }

    fn out(&mut self, combo: usize) -> Option<usize> {
        Some(self.resolve_combo(combo) % 8)
    }

    fn bdv(&mut self, combo: usize) -> Option<usize> {
        let combo: usize = self.resolve_combo(combo);
        self.registers[1] = self.registers[0] / 2_usize.pow(combo as u32);
        None
    }

    fn cdv(&mut self, combo: usize) -> Option<usize> {
        let combo: usize = self.resolve_combo(combo);
        self.registers[2] = self.registers[0] / 2_usize.pow(combo as u32);
        None
    }

    fn halted(&self, program: &[usize]) -> bool {
        self.instruction_pointer >= program.len()
    }
}

#[derive(Debug)]
struct Puzzle {
    registers: [usize; 3],
    program: Vec<usize>,
}

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        let read_register = |line: &str| {
            let (_, line) = line.split_once(": ").unwrap();
            line.parse::<usize>().unwrap()
        };
        let registers = [
            read_register(lines[0]),
            read_register(lines[1]),
            read_register(lines[2]),
        ];
        let program = lines[4];
        let (_, program) = program.split_once(": ").unwrap();
        let program = program
            .split(',')
            .map(|s| s.trim().parse::<usize>().unwrap())
            .collect();

        Ok(Puzzle { registers, program })
    }
}

impl Puzzle {
    fn process_part1(&self) -> String {
        let mut computer = Computer {
            registers: self.registers,
            instruction_pointer: 0,
        };
        let mut out = Vec::new();
        while !computer.halted(&self.program) {
            if let Some(o) = computer.run_instruction(&self.program) {
                out.push(o.to_string());
            }
        }
        out.join(",")
    }

    fn find_candidates(&self, base: usize, target: usize) -> Vec<usize> {
        let mut out = Vec::new();
        let program: &[usize] = &self.program;
        for num in 0..8 {
            let mut computer = Computer {
                registers: [base + num, 0, 0],
                instruction_pointer: 0,
            };
            let mut output = None;
            while !computer.halted(program) {
                if let Some(o) = computer.run_instruction(program) {
                    output = Some(o);
                    break;
                }
            }
            if output.unwrap() == target {
                out.push(num);
            }
        }
        out
    }

    fn search(&self, base: usize, target_index: usize) -> Option<usize> {
        let candidates = self.find_candidates(base, self.program[target_index]);
        for candidate in candidates {
            let new_base = (base + candidate) * 8;
            if target_index == 0 {
                return Some(base + candidate);
            }
            if let Some(output) = self.search(new_base, target_index - 1) {
                return Some(output);
            }
        }
        None
    }

    fn process_part2(&self) -> usize {
        self.search(0, self.program.len() - 1).unwrap()
    }
}

impl AocSolution for Day17 {
    fn part1(&self, input: &str) -> String {
        let puzzle: Puzzle = input.parse().unwrap();
        puzzle.process_part1()
    }

    fn part2(&self, input: &str) -> String {
        let puzzle: Puzzle = input.parse().unwrap();
        puzzle.process_part2().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_A: &str = r#"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"#;

    const EXAMPLE_B: &str = r#"Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"#;

    #[test]
    fn test_part1_example() {
        assert_eq!(Day17.part1(EXAMPLE_A), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2024, 17).expect("Failed to get input");
        assert_eq!(Day17.part1(&input), "2,1,4,7,6,0,3,1,4");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(Day17.part2(EXAMPLE_B), "117440");
    }

    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2024, 17).expect("Failed to get input");
        assert_eq!(Day17.part2(&input), "266932601404433");
    }
}
