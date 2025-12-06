use crate::AocSolution;
use std::str::FromStr;

pub struct Day11;

// Common helper functions
fn digit_count(num: &usize) -> usize {
    let mut digits = 0;
    let mut val = 1;
    while &val <= num {
        val *= 10;
        digits += 1;
    }
    digits
}

fn split_digits(num: &usize, ndigits: usize) -> (usize, usize) {
    let val = 10_usize.pow((ndigits / 2) as u32);
    let left = num / val;
    let right = num - left * val;
    (left, right)
}

// Part 1 specific structures and implementations
const BLINKS_PART1: usize = 25;

#[derive(Debug)]
struct PuzzlePart1 {
    buffers: [Vec<usize>; 2],
    blinks: usize,
}

impl FromStr for PuzzlePart1 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stones = Vec::new();
        for num in s.split_whitespace() {
            stones.push(num.parse::<usize>().unwrap());
        }
        let size = stones.len() * 2_usize.pow(BLINKS_PART1 as u32);
        let mut buffers = [Vec::with_capacity(size), Vec::with_capacity(size)];
        for stone in stones {
            buffers[0].push(stone);
        }
        Ok(PuzzlePart1 { buffers, blinks: 0 })
    }
}

impl PuzzlePart1 {
    fn current_index(&self) -> usize {
        self.blinks % 2
    }
    fn stone_count(&self) -> usize {
        self.buffers[self.current_index()].len()
    }
    fn blink(&mut self) {
        let current_index = self.current_index();
        let (b0, b1) = self.buffers.split_at_mut(1);
        let current;
        let next;
        if current_index == 0 {
            current = &b0[0];
            next = &mut b1[0];
        } else {
            current = &b1[0];
            next = &mut b0[0];
        }
        next.clear();
        for stone in current {
            if stone == &0 {
                next.push(1)
            } else {
                let digit_count = digit_count(stone);
                if digit_count.is_multiple_of(2) {
                    let (left, right) = split_digits(stone, digit_count);
                    next.push(left);
                    next.push(right);
                } else {
                    next.push(stone * 2024);
                }
            }
        }
    }
}

// Part 2 specific structures and implementations
#[derive(Debug)]
struct PuzzlePart2 {
    stones: Vec<usize>,
}

impl FromStr for PuzzlePart2 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stones = Vec::new();
        for num in s.split_whitespace() {
            stones.push(num.parse::<usize>().unwrap());
        }
        Ok(PuzzlePart2 { stones })
    }
}

#[derive(Debug)]
enum Stone {
    Val(usize),
    Seq((usize, usize)),
}
fn convert_split_val(val: usize) -> Stone {
    match val {
        0..=9 => Stone::Seq((val, 0)),
        n => Stone::Val(n),
    }
}
fn handle_stone(stone: &Stone) -> (Stone, Option<Stone>) {
    match stone {
        Stone::Val(0) => (Stone::Val(1), None),
        Stone::Val(n) => {
            let ndigits = digit_count(n);
            if ndigits.is_multiple_of(2) {
                let (left, right) = split_digits(n, ndigits);
                (convert_split_val(left), Some(convert_split_val(right)))
            } else {
                (Stone::Val(2024 * n), None)
            }
        }
        Stone::Seq((num, it)) => (Stone::Seq((*num, it + 1)), None),
    }
}

fn create_sequence(num: usize) -> Vec<Vec<usize>> {
    let mut sequence = Vec::with_capacity(10);
    for _ in 0..10 {
        sequence.push(Vec::with_capacity(num + 1)); // +1 for the initial value
        sequence.last_mut().unwrap().push(1);
    }
    let mut buffers = Vec::with_capacity(2);
    for _ in 0..2 {
        let mut vec = Vec::with_capacity(10);
        for _ in 0..10 {
            vec.push(Vec::new());
        }
        buffers.push(vec);
    }
    for (num, inner_vec) in buffers[0].iter_mut().enumerate() {
        inner_vec.push(Stone::Val(num));
    }
    for blink in 0..num {
        let (b0, b1) = buffers.split_at_mut(1);
        for i in 0..10 {
            let (current, next) = if blink % 2 == 0 {
                (&b0[0][i], &mut b1[0][i])
            } else {
                (&b1[0][i], &mut b0[0][i])
            };
            next.clear();
            for stone in current {
                let stones = handle_stone(stone);
                next.push(stones.0);
                if let Some(stone) = stones.1 {
                    next.push(stone)
                }
            }
            let new_value = next
                .iter()
                .map(|s| match s {
                    Stone::Seq((num_seq, it_seq)) => sequence[*num_seq][*it_seq],
                    Stone::Val(_) => 1,
                })
                .sum();
            sequence[i].push(new_value);
        }
    }
    sequence
}

fn count_stone(init: usize, sequence: &[Vec<usize>], nblinks: usize) -> usize {
    let mut buffers = Vec::with_capacity(2);
    for _ in 0..2 {
        buffers.push(Vec::new());
    }
    buffers[0].push(Stone::Val(init));
    let (b0, b1) = buffers.split_at_mut(1);
    for blink in 0..nblinks {
        let (current, next) = if blink % 2 == 0 {
            (&b0[0], &mut b1[0])
        } else {
            (&b1[0], &mut b0[0])
        };
        next.clear();
        for stone in current {
            let stones = handle_stone(stone);
            next.push(stones.0);
            if let Some(stone) = stones.1 {
                next.push(stone)
            }
        }
    }
    let next = if nblinks % 2 == 1 { &b1[0] } else { &b0[0] };
    next.iter()
        .map(|s| match s {
            Stone::Seq((num_seq, it_seq)) => sequence[*num_seq][*it_seq],
            Stone::Val(_) => 1,
        })
        .sum()
}

impl AocSolution for Day11 {
    fn part1(&self, input: &str) -> String {
        let mut puzzle = input.parse::<PuzzlePart1>().unwrap();
        while puzzle.blinks < BLINKS_PART1 {
            puzzle.blink();
            puzzle.blinks += 1;
        }
        puzzle.stone_count().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let puzzle = input.parse::<PuzzlePart2>().unwrap(); // Removed mut here, as it's not needed for the immediate step
        let nblinks = 75; // This value was taken from the main function in 11b.rs
        let sequence = create_sequence(nblinks);
        puzzle
            .stones
            .iter()
            .map(|s| count_stone(*s, &sequence, nblinks))
            .sum::<usize>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"125 17"#;

    #[test]
    fn test_part1_example() {
        assert_eq!(Day11.part1(EXAMPLE), "55312");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2024, 11).expect("Failed to get input");
        assert_eq!(Day11.part1(&input), "203609");
    }

    #[test]
    fn test_part2_example() {
        let nblinks_example = 25; // This value was used in 11b.rs test with EXAMPLE
        let puzzle = EXAMPLE.parse::<PuzzlePart2>().unwrap();
        let sequence = create_sequence(nblinks_example);
        let result = puzzle
            .stones
            .iter()
            .map(|s| count_stone(*s, &sequence, nblinks_example))
            .sum::<usize>();
        assert_eq!(result.to_string(), "55312");
    }

    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2024, 11).expect("Failed to get input");
        assert_eq!(Day11.part2(&input), "240954878211138");
    }

    #[test]
    fn test_digit_count_1() {
        let out = digit_count(&1);
        assert_eq!(out, 1);
    }
    #[test]
    fn test_digit_count_9() {
        let out = digit_count(&9);
        assert_eq!(out, 1);
    }
    #[test]
    fn test_digit_count_10() {
        let out = digit_count(&10);
        assert_eq!(out, 2);
    }
}
