use crate::AocSolution;

pub struct Day{DAY};

impl AocSolution for Day{DAY} {
    fn part1(&self, _input: &str) -> String {
        "Not implemented".to_string()
    }

    fn part2(&self, _input: &str) -> String {
        "Not implemented".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"";

    #[test]
    fn test_part1_example() {
        assert_eq!(Day{DAY}.part1(EXAMPLE), "REPLACE_WITH_PART1_EXAMPLE_RESULT");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day({YEAR}, {DAY}).expect("Failed to get input");
        assert_eq!(Day{DAY}.part1(&input), "REPLACE_WITH_PART1_FULL_RESULT");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(Day{DAY}.part2(EXAMPLE), "REPLACE_WITH_PART2_EXAMPLE_RESULT");
    }

    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day({YEAR}, {DAY}).expect("Failed to get input");
        assert_eq!(Day{DAY}.part2(&input), "REPLACE_WITH_PART2_FULL_RESULT");
    }
}