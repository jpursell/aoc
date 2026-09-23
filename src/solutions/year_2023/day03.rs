use crate::AocSolution;

pub struct Day03;

struct Schematic<'a> {
    lines: Vec<&'a str>,
    width: usize,
}

fn is_op(c: char) -> bool {
    match c {
        '*' | '+' | '#' | '$' | '-' | '/' | '=' | '%' | '&' | '@' => true,
        '.' => false,
        other => {
            if other.is_numeric() {
                false
            } else {
                panic!("Unexpected char {}", c)
            }
        }
    }
}

enum State {
    Looking,
    Found,
}

struct NumLocation {
    iline: usize,
    start: usize,
    end: usize,
}

impl NumLocation {
    fn new(iline: usize, start: usize, end: usize) -> NumLocation {
        NumLocation { iline, start, end }
    }
}

impl<'a> Schematic<'a> {
    fn new(input: &str) -> Schematic<'_> {
        let lines = input
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<&str>>();
        let width = lines[0].len();
        for line in &lines {
            assert_eq!(width, line.len());
        }
        Schematic { lines, width }
    }

    fn extract_part(&self, loc: &NumLocation) -> u32 {
        self.lines[loc.iline][loc.start..loc.end]
            .parse::<u32>()
            .unwrap()
    }

    fn is_part(&self, loc: &NumLocation) -> bool {
        let left = if loc.start == 0 {
            0
        } else {
            if is_op(self.lines[loc.iline].chars().nth(loc.start - 1).unwrap()) {
                return true;
            }
            loc.start - 1
        };
        let right = if loc.end == self.width {
            self.width
        } else {
            if is_op(self.lines[loc.iline].chars().nth(loc.end).unwrap()) {
                return true;
            }
            loc.end + 1
        };
        if loc.iline > 0 {
            for c in self.lines[loc.iline - 1][left..right].chars() {
                if is_op(c) {
                    return true;
                }
            }
        }
        if loc.iline < self.lines.len() - 1 {
            for c in self.lines[loc.iline + 1][left..right].chars() {
                if is_op(c) {
                    return true;
                }
            }
        }
        false
    }

    fn find_num_locations(&self) -> Vec<NumLocation> {
        let mut output = Vec::new();
        for (iline, line) in self.lines.iter().enumerate() {
            let mut start = 0;
            let mut state = State::Looking;
            for (pos, c) in line.chars().enumerate() {
                match state {
                    State::Looking => {
                        if c.is_numeric() {
                            start = pos;
                            state = State::Found;
                        }
                    }
                    State::Found => {
                        if !c.is_numeric() {
                            state = State::Looking;
                            output.push(NumLocation::new(iline, start, pos));
                        }
                    }
                }
            }
            if matches!(state, State::Found) {
                output.push(NumLocation::new(iline, start, line.len()));
            }
        }
        output
    }

    fn find_parts(&self) -> Vec<NumLocation> {
        self.find_num_locations()
            .into_iter()
            .filter(|loc| self.is_part(loc))
            .collect()
    }

    fn get_gear_ratio(&self, iline: usize, pos: usize, parts: &[NumLocation]) -> Option<u32> {
        let matching = parts
            .iter()
            .filter(|loc| {
                (loc.iline == iline
                    || (iline > 0 && loc.iline == iline - 1)
                    || loc.iline == iline + 1)
                    && (pos <= loc.end && pos as i64 >= loc.start as i64 - 1)
            })
            .collect::<Vec<&NumLocation>>();
        if matching.len() == 2 {
            Some(matching.iter().map(|loc| self.extract_part(loc)).product())
        } else {
            None
        }
    }

    fn sum_gear_ratios(&self) -> u32 {
        let parts = self.find_parts();
        let mut sum = 0_u32;
        for (iline, line) in self.lines.iter().enumerate() {
            for (pos, c) in line.chars().enumerate() {
                if c == '*' {
                    if let Some(ratio) = self.get_gear_ratio(iline, pos, &parts) {
                        sum += ratio;
                    }
                }
            }
        }
        sum
    }
}

fn day_3a(input: &str) -> u32 {
    let schematic = Schematic::new(input);
    let parts = schematic.find_parts();
    parts.iter().map(|loc| schematic.extract_part(loc)).sum()
}

fn day_3b(input: &str) -> u32 {
    Schematic::new(input).sum_gear_ratios()
}

impl AocSolution for Day03 {
    fn part1(&self, input: &str) -> String {
        day_3a(input).to_string()
    }

    fn part2(&self, input: &str) -> String {
        day_3b(input).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

    #[test]
    fn test_part1_example() {
        assert_eq!(Day03.part1(EXAMPLE), "4361");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2023, 3).expect("Failed to get input");
        assert_eq!(Day03.part1(&input), "509115");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(Day03.part2(EXAMPLE), "467835");
    }

    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2023, 3).expect("Failed to get input");
        assert_eq!(Day03.part2(&input), "75220503");
    }
}
