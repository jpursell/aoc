use crate::AocSolution;

pub struct Day09;

fn predict_next(numbers: &[i64]) -> i64 {
    let mut vecs: Vec<Vec<i64>> = Vec::new();
    vecs.push(numbers.to_vec());
    while vecs.last().unwrap().iter().any(|&x| x != 0) {
        let next: Vec<i64> = vecs
            .last()
            .unwrap()
            .windows(2)
            .map(|w| w[1] - w[0])
            .collect();
        vecs.push(next);
    }
    vecs.iter().map(|v| v.last().unwrap()).sum::<i64>()
}

fn predict_prev(numbers: &[i64]) -> i64 {
    let mut vecs: Vec<Vec<i64>> = Vec::new();
    vecs.push(numbers.to_vec());
    while vecs.last().unwrap().iter().any(|&x| x != 0) {
        let next: Vec<i64> = vecs
            .last()
            .unwrap()
            .windows(2)
            .map(|w| w[1] - w[0])
            .collect();
        vecs.push(next);
    }
    let mut val = 0;
    for vec in vecs.iter().rev().skip(1) {
        val = vec[0] - val;
    }
    val
}

fn parse_line(line: &str) -> Vec<i64> {
    line.split_whitespace()
        .filter_map(|s| s.parse::<i64>().ok())
        .collect()
}

impl AocSolution for Day09 {
    fn part1(&self, input: &str) -> String {
        let total: i64 = input
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| predict_next(&parse_line(line)))
            .sum();
        total.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let total: i64 = input
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| predict_prev(&parse_line(line)))
            .sum();
        total.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_part1_example() {
        assert_eq!(Day09.part1(EXAMPLE), "114");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2023, 9).expect("Failed to get input");
        assert_eq!(Day09.part1(&input), "1995001648");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(Day09.part2(EXAMPLE), "2");
    }

    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2023, 9).expect("Failed to get input");
        assert_eq!(Day09.part2(&input), "988");
    }
}
