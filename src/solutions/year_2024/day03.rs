use crate::AocSolution;

pub struct Day03;

use regex::Regex;
#[derive(Debug)]
struct Mul {
    a: usize,
    b: usize,
}
fn extract(str: &str) -> Vec<Mul> {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let mut out = Vec::new();
    for (_, [a, b]) in re.captures_iter(str).map(|c| c.extract()) {
        let a = a.parse().unwrap();
        let b = b.parse().unwrap();
        out.push(Mul { a, b });
    }
    out
}

fn process(vecs: Vec<Mul>) -> usize {
    let mut out = 0;
    for a in vecs.iter() {
        out += a.a * a.b;
    }
    out
}

#[derive(Clone, Copy, Debug)]
enum Token {
    Do,
    Dont,
    Mul(usize),
    None,
}
fn extract2(str: &str) -> Vec<Token> {
    let mul_re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let do_re = Regex::new(r"do\(\)").unwrap();
    let dont_re = Regex::new(r"don't\(\)").unwrap();

    let mut tokens = vec![Token::None; str.len()];
    for cap in mul_re.captures_iter(str) {
        let loc = cap.get(0).unwrap().start();
        let (_, [a, b]) = cap.extract();
        let a: usize = a.parse().unwrap();
        let b: usize = b.parse().unwrap();
        let val = a * b;
        tokens[loc] = Token::Mul(val);
    }
    for cap in do_re.captures_iter(str) {
        let loc = cap.get(0).unwrap().start();
        tokens[loc] = Token::Do;
    }
    for cap in dont_re.captures_iter(str) {
        let loc = cap.get(0).unwrap().start();
        tokens[loc] = Token::Dont;
    }
    tokens
}

fn process2(tokens: &[Token]) -> usize {
    let mut out = 0;
    let mut active = true;
    for token in tokens {
        match token {
            Token::Do => {
                active = true;
            }
            Token::Dont => {
                active = false;
            }
            Token::Mul(val) => {
                if active {
                    out += val
                }
            }
            _ => (),
        }
    }
    out
}

impl AocSolution for Day03 {
    fn part1(&self, input: &str) -> String {
        let out = extract(input);
        process(out).to_string()
    }

    fn part2(&self, input: &str) -> String {
        let out = extract2(input);
        process2(&out).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str =
        r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const EXAMPLE2: &str =
        r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_part1_example() {
        assert_eq!(Day03.part1(EXAMPLE), "161");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2024, 3).expect("Failed to get input");
        assert_eq!(Day03.part1(&input), "191183308");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(Day03.part2(EXAMPLE2), "48");
    }

    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2024, 3).expect("Failed to get input");
        assert_eq!(Day03.part2(&input), "92082041");
    }
}
