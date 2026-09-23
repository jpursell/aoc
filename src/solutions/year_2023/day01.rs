use crate::AocSolution;

pub struct Day01;

fn day_1a_proc_line(line: &str) -> u32 {
    let mut sum = 0_u32;
    for c in line.chars() {
        match c {
            '0' => break,
            '1' => {
                sum += 10;
                break;
            }
            '2' => {
                sum += 20;
                break;
            }
            '3' => {
                sum += 30;
                break;
            }
            '4' => {
                sum += 40;
                break;
            }
            '5' => {
                sum += 50;
                break;
            }
            '6' => {
                sum += 60;
                break;
            }
            '7' => {
                sum += 70;
                break;
            }
            '8' => {
                sum += 80;
                break;
            }
            '9' => {
                sum += 90;
                break;
            }
            _ => (),
        }
    }
    for c in line.chars().rev() {
        match c {
            '0' => break,
            '1' => {
                sum += 1;
                break;
            }
            '2' => {
                sum += 2;
                break;
            }
            '3' => {
                sum += 3;
                break;
            }
            '4' => {
                sum += 4;
                break;
            }
            '5' => {
                sum += 5;
                break;
            }
            '6' => {
                sum += 6;
                break;
            }
            '7' => {
                sum += 7;
                break;
            }
            '8' => {
                sum += 8;
                break;
            }
            '9' => {
                sum += 9;
                break;
            }
            _ => (),
        }
    }
    sum
}

fn day_1a(input: &str) -> u32 {
    let mut sum: u32 = 0;
    for line in input.lines() {
        sum += day_1a_proc_line(line);
    }
    sum
}

fn day_1b(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        for (pos, char) in line.chars().enumerate() {
            match char {
                '1' => {
                    sum += 10;
                    break;
                }
                '2' => {
                    sum += 20;
                    break;
                }
                '3' => {
                    sum += 30;
                    break;
                }
                '4' => {
                    sum += 40;
                    break;
                }
                '5' => {
                    sum += 50;
                    break;
                }
                '6' => {
                    sum += 60;
                    break;
                }
                '7' => {
                    sum += 70;
                    break;
                }
                '8' => {
                    sum += 80;
                    break;
                }
                '9' => {
                    sum += 90;
                    break;
                }
                'o' => {
                    if line.len() - pos >= 3 && &line[pos + 1..pos + 3] == "ne" {
                        sum += 10;
                        break;
                    }
                }
                't' => {
                    if line.len() - pos >= 3 && &line[pos + 1..pos + 3] == "wo" {
                        sum += 20;
                        break;
                    }
                    if line.len() - pos >= 5 && &line[pos + 1..pos + 5] == "hree" {
                        sum += 30;
                        break;
                    }
                }
                'f' => {
                    if line.len() - pos >= 4 && &line[pos + 1..pos + 4] == "our" {
                        sum += 40;
                        break;
                    }
                    if line.len() - pos >= 4 && &line[pos + 1..pos + 4] == "ive" {
                        sum += 50;
                        break;
                    }
                }
                's' => {
                    if line.len() - pos >= 3 && &line[pos + 1..pos + 3] == "ix" {
                        sum += 60;
                        break;
                    }
                    if line.len() - pos >= 5 && &line[pos + 1..pos + 5] == "even" {
                        sum += 70;
                        break;
                    }
                }
                'e' => {
                    if line.len() - pos >= 5 && &line[pos + 1..pos + 5] == "ight" {
                        sum += 80;
                        break;
                    }
                }
                'n' if line.len() - pos >= 4 && &line[pos + 1..pos + 4] == "ine" => {
                    sum += 90;
                    break;
                }
                _ => (),
            }
        }
        let line_rev = line.chars().rev().collect::<String>();
        for (pos, char) in line_rev.chars().enumerate() {
            match char {
                '1' => {
                    sum += 1;
                    break;
                }
                '2' => {
                    sum += 2;
                    break;
                }
                '3' => {
                    sum += 3;
                    break;
                }
                '4' => {
                    sum += 4;
                    break;
                }
                '5' => {
                    sum += 5;
                    break;
                }
                '6' => {
                    sum += 6;
                    break;
                }
                '7' => {
                    sum += 7;
                    break;
                }
                '8' => {
                    sum += 8;
                    break;
                }
                '9' => {
                    sum += 9;
                    break;
                }
                'e' => {
                    if line_rev.len() - pos >= 3 && &line_rev[pos + 1..pos + 3] == "no" {
                        sum += 1;
                        break;
                    }
                    if line_rev.len() - pos >= 5 && &line_rev[pos + 1..pos + 5] == "erht" {
                        sum += 3;
                        break;
                    }
                    if line_rev.len() - pos >= 4 && &line_rev[pos + 1..pos + 4] == "vif" {
                        sum += 5;
                        break;
                    }
                    if line_rev.len() - pos >= 4 && &line_rev[pos + 1..pos + 4] == "nin" {
                        sum += 9;
                        break;
                    }
                }
                'o' => {
                    if line_rev.len() - pos >= 3 && &line_rev[pos + 1..pos + 3] == "wt" {
                        sum += 2;
                        break;
                    }
                }
                'r' => {
                    if line_rev.len() - pos >= 4 && &line_rev[pos + 1..pos + 4] == "uof" {
                        sum += 4;
                        break;
                    }
                }
                'x' => {
                    if line_rev.len() - pos >= 3 && &line_rev[pos + 1..pos + 3] == "is" {
                        sum += 6;
                        break;
                    }
                }
                'n' => {
                    if line_rev.len() - pos >= 5 && &line_rev[pos + 1..pos + 5] == "eves" {
                        sum += 7;
                        break;
                    }
                }
                't' if line_rev.len() - pos >= 5 && &line_rev[pos + 1..pos + 5] == "hgie" => {
                    sum += 8;
                    break;
                }
                _ => (),
            }
        }
    }
    sum
}

impl AocSolution for Day01 {
    fn part1(&self, input: &str) -> String {
        day_1a(input).to_string()
    }

    fn part2(&self, input: &str) -> String {
        day_1b(input).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_A: &str = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;

    const EXAMPLE_B: &str = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;

    #[test]
    fn test_part1_example() {
        assert_eq!(Day01.part1(EXAMPLE_A), "142");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2023, 1).expect("Failed to get input");
        assert_eq!(Day01.part1(&input), "54644");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(Day01.part2(EXAMPLE_B), "281");
    }

    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2023, 1).expect("Failed to get input");
        assert_eq!(Day01.part2(&input), "53348");
    }
}
