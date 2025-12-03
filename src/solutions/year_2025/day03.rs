use crate::AocSolution;

pub struct Day03;

fn max_joltage(line: &str, ndigits: usize) -> u64 {
    let mut out = 0;
    let mut skip = 0;
    for digit in (0..ndigits).rev() {
        let mut best: Option<(usize, u32)> = None;
        for (num, char) in line
            .chars()
            .skip(skip)
            .take(line.len() - skip - digit)
            .enumerate()
        {
            let val = char.to_digit(10).unwrap();
            if let Some((_, best_val)) = best {
                if val > best_val {
                    best = Some((num, val));
                }
            } else {
                best = Some((num, val));
            }
        }
        if let Some((num, val)) = best {
            skip += num + 1;
            out += val as u64 * 10_u64.pow(digit as u32);
        }
    }
    out
}

impl AocSolution for Day03 {
    fn part1(&self, input: &str) -> String {
        input
            .lines()
            .map(|s| max_joltage(s, 2))
            .sum::<u64>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        input
            .lines()
            .map(|s| max_joltage(s, 12))
            .sum::<u64>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_part1_example() {
        assert_eq!(Day03.part1(EXAMPLE), "357");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2025, 3).expect("Failed to get input");
        assert_eq!(Day03.part1(&input), "17332");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(Day03.part2(EXAMPLE), "3121910778619");
    }

    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2025, 3).expect("Failed to get input");
        assert_eq!(Day03.part2(&input), "172516781546707");
    }
}
