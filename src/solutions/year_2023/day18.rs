use crate::AocSolution;

pub struct Day18;

fn solve<I>(instructions: I) -> i64
where
    I: IntoIterator<Item = (char, i64)>,
{
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut double_area: i64 = 0;
    let mut perimeter: i64 = 0;

    for (dir, len) in instructions {
        let (dx, dy) = match dir {
            'R' => (1, 0),
            'D' => (0, 1),
            'L' => (-1, 0),
            'U' => (0, -1),
            _ => panic!("Unknown direction {}", dir),
        };
        let nx = x + dx * len;
        let ny = y + dy * len;
        double_area += x * ny - nx * y;
        perimeter += len;
        x = nx;
        y = ny;
    }

    double_area.abs() / 2 + perimeter / 2 + 1
}

fn parse_line_part1(line: &str) -> (char, i64) {
    let mut parts = line.split_whitespace();
    let dir = parts.next().unwrap().chars().next().unwrap();
    let len: i64 = parts.next().unwrap().parse().unwrap();
    (dir, len)
}

fn parse_line_part2(line: &str) -> (char, i64) {
    let hex_part = line
        .split_whitespace()
        .nth(2)
        .unwrap()
        .trim_matches(|c| c == '(' || c == ')' || c == '#');
    let len = i64::from_str_radix(&hex_part[..5], 16).unwrap();
    let dir = match &hex_part[5..6] {
        "0" => 'R',
        "1" => 'D',
        "2" => 'L',
        "3" => 'U',
        d => panic!("Unknown dir code {}", d),
    };
    (dir, len)
}

impl AocSolution for Day18 {
    fn part1(&self, input: &str) -> String {
        let instructions = input
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(parse_line_part1);
        solve(instructions).to_string()
    }

    fn part2(&self, input: &str) -> String {
        let instructions = input
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(parse_line_part2);
        solve(instructions).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn test_part1_example() {
        assert_eq!(Day18.part1(EXAMPLE), "62");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2023, 18).expect("Failed to get input");
        assert_eq!(Day18.part1(&input), "48503");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(Day18.part2(EXAMPLE), "952408144115");
    }

    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2023, 18).expect("Failed to get input");
        assert_eq!(Day18.part2(&input), "148442153147147");
    }
}
