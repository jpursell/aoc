use crate::AocSolution;

pub struct Day02;

fn day_2a(input: &str) -> u32 {
    let mut sum: u32 = 0;
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let (game_str, data_str) = line.split_at(line.find(':').unwrap());
        let game_num = game_str[game_str.find("Game ").unwrap() + 5..]
            .parse::<u32>()
            .unwrap();
        let mut possible = true;
        for grab_str in data_str[1..].split(';') {
            for count_str in grab_str.split(',') {
                let count_str = count_str.trim();
                let (num, color) = count_str.split_once(' ').unwrap();
                let color = color.trim();
                let num = num.parse::<u32>().unwrap();
                possible &= match color {
                    "red" => num <= 12,
                    "green" => num <= 13,
                    "blue" => num <= 14,
                    other => panic!("Got bad color with len {}: {}", other.len(), other),
                };
            }
        }
        if possible {
            sum += game_num;
        }
    }
    sum
}

fn day_2b(input: &str) -> u32 {
    let mut sum: u32 = 0;
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let (_game_str, data_str) = line.split_at(line.find(':').unwrap());
        let mut max_red: u32 = 0;
        let mut max_green: u32 = 0;
        let mut max_blue: u32 = 0;
        for grab_str in data_str[1..].split(';') {
            for count_str in grab_str.split(',') {
                let count_str = count_str.trim();
                let (num, color) = count_str.split_once(' ').unwrap();
                let color = color.trim();
                let num = num.parse::<u32>().unwrap();
                match color {
                    "red" => max_red = max_red.max(num),
                    "green" => max_green = max_green.max(num),
                    "blue" => max_blue = max_blue.max(num),
                    other => panic!("Got bad color with len {}: {}", other.len(), other),
                };
            }
        }
        let power = max_red * max_blue * max_green;
        sum += power;
    }
    sum
}

impl AocSolution for Day02 {
    fn part1(&self, input: &str) -> String {
        day_2a(input).to_string()
    }

    fn part2(&self, input: &str) -> String {
        day_2b(input).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    #[test]
    fn test_part1_example() {
        assert_eq!(Day02.part1(EXAMPLE), "8");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2023, 2).expect("Failed to get input");
        assert_eq!(Day02.part1(&input), "1853");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(Day02.part2(EXAMPLE), "2286");
    }

    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2023, 2).expect("Failed to get input");
        assert_eq!(Day02.part2(&input), "72706");
    }
}
