use std::str::FromStr;

use crate::AocSolution;

#[derive(Debug)]
struct Equation {
    result: usize,
    values: Vec<usize>,
}

impl FromStr for Equation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (result, values) = s.split_once(": ").unwrap();
        let result = result.parse().unwrap();
        let values: Vec<usize> = values
            .split(" ")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        Ok(Equation { result, values })
    }
}

fn concatenate(a: usize, b: usize) -> usize {
    let mut value = 1;
    while value <= b {
        value *= 10;
    }
    a * value + b
}

impl Equation {
    fn solvable_p1(&self) -> bool {
        self.count_solutions_p1(0, self.values[0]) > 0
    }
    fn count_solutions_p1(&self, position: usize, partial_value: usize) -> usize {
        assert!(position < self.values.len() - 1);
        let at_bottom = position == self.values.len() - 2;
        let mut solutions = 0;
        // try Add
        {
            let partial_value = partial_value + self.values[position + 1];
            if partial_value <= self.result {
                if at_bottom {
                    if partial_value == self.result {
                        solutions += 1;
                    }
                } else {
                    solutions += self.count_solutions_p1(position + 1, partial_value)
                }
            }
        }
        // try Multiply
        {
            let partial_value = partial_value * self.values[position + 1];
            if partial_value <= self.result {
                if at_bottom {
                    if partial_value == self.result {
                        solutions += 1;
                    }
                } else {
                    solutions += self.count_solutions_p1(position + 1, partial_value)
                }
            }
        }
        solutions
    }

    fn solvable_p2(&self) -> bool {
        self.count_solutions_p2(0, self.values[0]) > 0
    }
    fn count_solutions_p2(&self, position: usize, partial_value: usize) -> usize {
        assert!(position < self.values.len() - 1);
        let at_bottom = position == self.values.len() - 2;
        let mut solutions = 0;
        // try Add
        {
            let partial_value = partial_value + self.values[position + 1];
            if partial_value <= self.result {
                if at_bottom {
                    if partial_value == self.result {
                        solutions += 1;
                    }
                } else {
                    solutions += self.count_solutions_p2(position + 1, partial_value)
                }
            }
        }
        // try Multiply
        {
            let partial_value = partial_value * self.values[position + 1];
            if partial_value <= self.result {
                if at_bottom {
                    if partial_value == self.result {
                        solutions += 1;
                    }
                } else {
                    solutions += self.count_solutions_p2(position + 1, partial_value)
                }
            }
        }
        // try Concatenation
        {
            let partial_value = concatenate(partial_value, self.values[position + 1]);
            if partial_value <= self.result {
                if at_bottom {
                    if partial_value == self.result {
                        solutions += 1;
                    }
                } else {
                    solutions += self.count_solutions_p2(position + 1, partial_value)
                }
            }
        }
        solutions
    }
}

#[derive(Debug)]
struct Puzzle {
    equations: Vec<Equation>,
}

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let count = s.lines().count();
        let mut equations = Vec::with_capacity(count);
        for line in s.lines() {
            equations.push(line.parse::<Equation>().unwrap());
        }
        Ok(Puzzle { equations })
    }
}

impl Puzzle {
    fn process_p1(&mut self) -> usize {
        let mut out = 0;
        for equation in &self.equations {
            if equation.solvable_p1() {
                out += equation.result;
            }
        }
        out
    }

    fn process_p2(&mut self) -> usize {
        let mut out = 0;
        for equation in &self.equations {
            if equation.solvable_p2() {
                out += equation.result;
            }
        }
        out
    }
}

pub struct Day07;

impl AocSolution for Day07 {
    fn part1(&self, input: &str) -> String {
        let mut puzzle = input.parse::<Puzzle>().unwrap();
        puzzle.process_p1().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut puzzle = input.parse::<Puzzle>().unwrap();
        puzzle.process_p2().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_part1_example() {
        assert_eq!(Day07.part1(EXAMPLE), "3749");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2024, 7).expect("Failed to get input");
        assert_eq!(Day07.part1(&input), "1038838357795");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(Day07.part2(EXAMPLE), "11387");
    }

    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2024, 7).expect("Failed to get input");
        assert_eq!(Day07.part2(&input), "254136560217241");
    }
}
