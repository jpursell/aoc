use crate::AocSolution;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

struct Instruction {
    direction: Direction,
    value: i16,
}

struct InstructionSequence {
    instructions: Vec<Instruction>,
}

fn parse(input: &str) -> InstructionSequence {
    let mut instructions: Vec<Instruction> = Vec::with_capacity(input.lines().count());
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let (dir, val) = line.split_at(1);
        let dir = match dir {
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Got unknown direction"),
        };
        let val: i16 = val.parse().unwrap();
        instructions.push(Instruction {
            direction: dir,
            value: val,
        });
    }
    InstructionSequence { instructions }
}

pub struct Day01;

fn clicks_zero(mut current: i16, change: i16) -> (i16, u64) {
    if change == 0 {
        return (current, 0);
    }
    if change < 0 {
        let start_above_zero = current > 0;
        current += change;
        if current > 0 {
            return (current, 0);
        }
        if current == 0 {
            return (current, 1);
        }

        let modulus: u64 = (current / -100) as u64;
        current %= 100;
        if current < 0 {
            current += 100;
        }
        let mut count = modulus;
        if start_above_zero {
            count += 1;
        }
        return (current, count);
    }
    current += change;
    if current < 99 {
        return (current, 0);
    }
    let modulus: u64 = current as u64 / 100;
    current %= 100;
    (current, modulus)
}

impl AocSolution for Day01 {
    fn part1(&self, input: &str) -> String {
        let mut current: i16 = 50;
        let mut count: u64 = 0;
        let instructions = parse(input);
        for Instruction { direction, value } in &instructions.instructions {
            match direction {
                Direction::Left => current -= value,
                Direction::Right => current += value,
            }
            current %= 100;
            if current == 0 {
                count += 1;
            }
        }
        count.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut current: i16 = 50;
        let mut count: u64 = 0;
        let instructions = parse(input);
        for Instruction { direction, value } in &instructions.instructions {
            let change = match direction {
                Direction::Left => -*value,
                Direction::Right => *value,
            };
            let (new_current, zero_clicks) = clicks_zero(current, change);
            current = new_current;
            count += zero_clicks;
        }
        count.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_part1_example() {
        assert_eq!(Day01.part1(EXAMPLE), "3");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2025, 1).expect("Failed to get input");
        assert_eq!(Day01.part1(&input), "1034");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(Day01.part2(EXAMPLE), "6");
    }

    #[test]
    fn negative_cases() {
        let current = 2;
        let change = 0;
        let (new_current, zero_clicks) = clicks_zero(current, change);
        assert_eq!(new_current, 2);
        assert_eq!(zero_clicks, 0);

        let change = -1;
        let (new_current, zero_clicks) = clicks_zero(current, change);
        assert_eq!(new_current, 1);
        assert_eq!(zero_clicks, 0);

        let change = -2;
        let (new_current, zero_clicks) = clicks_zero(current, change);
        assert_eq!(new_current, 0);
        assert_eq!(zero_clicks, 1);

        let change = -3;
        let (new_current, zero_clicks) = clicks_zero(current, change);
        assert_eq!(new_current, 99);
        assert_eq!(zero_clicks, 1);

        let change = -101;
        let (new_current, zero_clicks) = clicks_zero(current, change);
        assert_eq!(new_current, 1);
        assert_eq!(zero_clicks, 1);

        let change = -102;
        let (new_current, zero_clicks) = clicks_zero(current, change);
        assert_eq!(new_current, 0);
        assert_eq!(zero_clicks, 2);

        let change = -103;
        let (new_current, zero_clicks) = clicks_zero(current, change);
        assert_eq!(new_current, 99);
        assert_eq!(zero_clicks, 2);
    }
    #[test]
    fn test_part2_pos_land_on_zero() {
        let current = 52;
        let change = 48;
        let (new_current, zero_clicks) = clicks_zero(current, change);
        assert_eq!(new_current, 0);
        assert_eq!(zero_clicks, 1);
    }
    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2025, 1).expect("Failed to get input");
        assert_eq!(Day01.part2(&input), "6166");
    }
}
