use crate::AocSolution;

#[derive(Debug)]
enum Op {
    Plus,
    Times,
}
fn solve_worksheet(input: &str) -> u64 {
    let nlines = input.lines().count();
    let numbers: Vec<Vec<u64>> = input
        .lines()
        .take(nlines - 1)
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect();
    let ops: Vec<Op> = input
        .lines()
        .next_back()
        .unwrap()
        .split_whitespace()
        .map(|s| match s {
            "*" => Op::Times,
            "+" => Op::Plus,
            _ => panic!("Got unknown operator"),
        })
        .collect();
    ops.iter()
        .enumerate()
        .map(|(i, op)| {
            numbers.iter().fold(
                match op {
                    Op::Plus => 0,
                    Op::Times => 1,
                },
                |acc, val| match op {
                    Op::Plus => acc + val[i],
                    Op::Times => acc * val[i],
                },
            )
        })
        .sum()
}

fn solve_transpose_worksheet(input: &str) -> u64 {
    let mut iters: Vec<_> = input.lines().map(|line| line.chars().rev()).collect();
    let mut stack: Vec<u64> = Vec::new();
    let mut total = 0;
    loop {
        let last_col: Vec<_> = iters.iter_mut().map(|i| i.next()).collect();
        if last_col[0].is_none() {
            break;
        }
        let last_col: String = last_col.iter().map(|x| x.unwrap()).collect();
        total += match last_col.chars().next_back().unwrap() {
            ' ' => {
                stack.push(
                    last_col
                        .trim()
                        .parse()
                        .expect("Failed to parse col ending in space to number"),
                );
                0
            }
            c => {
                let last_num: String = last_col.chars().take(last_col.len() - 1).collect();
                stack.push(last_num.trim().parse().unwrap());
                let val = match c {
                    '+' => stack.iter().sum(),
                    '*' => stack.iter().product(),
                    _ => panic!("Got unknown operator"),
                };
                stack.clear();

                // clear empty col
                iters.iter_mut().for_each(|i| {
                    i.next();
                });
                val
            }
        };
    }
    total
}

pub struct Day06;

impl AocSolution for Day06 {
    fn part1(&self, input: &str) -> String {
        solve_worksheet(input).to_string()
    }

    fn part2(&self, input: &str) -> String {
        solve_transpose_worksheet(input).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_part1_example() {
        assert_eq!(Day06.part1(EXAMPLE), "4277556");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2025, 6).expect("Failed to get input");
        assert_eq!(Day06.part1(&input), "6891729672676");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(Day06.part2(EXAMPLE), "3263827");
    }

    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2025, 6).expect("Failed to get input");
        assert_eq!(Day06.part2(&input), "9770311947567");
    }
}
