use crate::AocSolution;

pub struct Day13;

#[derive(Debug)]
struct Game {
    buttons: [[usize; 2]; 2],
    prize: [usize; 2],
}

impl Game {
    fn solve(&self) -> Option<[usize; 2]> {
        let [[ax, ay], [bx, by]] = self.buttons.map(|v| v.map(|x| x as f64));
        let [px, py] = self.prize.map(|x| x as f64);
        let b = (py * ax - ay * px) / (by * ax - bx * ay);
        let a = (px - bx * b) / ax;
        if a >= 0.0 && b >= 0.0 && a % 1.0 == 0.0 && b % 1.0 == 0.0 {
            Some([a as usize, b as usize])
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Puzzle {
    games: Vec<Game>,
}

impl Puzzle {
    fn process(&self) -> usize {
        self.games
            .iter()
            .map(|g| {
                if let Some([a, b]) = g.solve() {
                    a * 3 + b
                } else {
                    0
                }
            })
            .sum()
    }
}

fn parse_games(s: &str, part2: bool) -> Vec<Game> {
    let mut games = Vec::new();
    let mut lines = s.lines();
    loop {
        let chunk: Vec<&str> = lines.by_ref().take(3).collect();
        if chunk.len() < 3 {
            break;
        }

        let parse_num = |s: &str| {
            let (_, num) = s.split_once("+").unwrap();
            num.parse::<usize>().unwrap()
        };
        let parse_button = |s: &str| {
            let (_, rem) = s.split_once(": ").unwrap();
            let (x, y) = rem.split_once(", ").unwrap();
            [parse_num(x), parse_num(y)]
        };
        let parse_prize_num = |s: &str| {
            let (_, num) = s.split_once("=").unwrap();
            let val = num.parse::<usize>().unwrap();
            if part2 {
                val + 10000000000000
            } else {
                val
            }
        };
        let parse_prize = |s: &str| {
            let (_, rem) = s.split_once(": ").unwrap();
            let (x, y) = rem.split_once(", ").unwrap();
            [parse_prize_num(x), parse_prize_num(y)]
        };
        let buttons = [parse_button(chunk[0]), parse_button(chunk[1])];
        let prize = parse_prize(chunk[2]);
        games.push(Game { buttons, prize });
        lines.next(); // Skip the blank line
    }
    games
}

impl AocSolution for Day13 {
    fn part1(&self, input: &str) -> String {
        let games = parse_games(input, false);
        let puzzle = Puzzle { games };
        puzzle.process().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let games = parse_games(input, true);
        let puzzle = Puzzle { games };
        puzzle.process().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#;

    #[test]
    fn test_part1_example() {
        assert_eq!(Day13.part1(EXAMPLE), "480");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2024, 13).expect("Failed to get input");
        assert_eq!(Day13.part1(&input), "28059");
    }

    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2024, 13).expect("Failed to get input");
        assert_eq!(Day13.part2(&input), "102255878088512");
    }
}
