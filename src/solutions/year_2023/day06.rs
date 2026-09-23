use crate::AocSolution;
use std::str::FromStr;

pub struct Day06;

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn num_ways_to_win(&self) -> u64 {
        let d = self.distance as f64;
        let t = self.time as f64;
        let inner = (t * t - 4.0 * d).sqrt();
        let low = (t - inner) / 2.0;
        let mut low_ceil = low.ceil();
        if low == low_ceil {
            low_ceil += 1.0;
        }
        let high = (t + inner) / 2.0;
        let mut high_floor = high.floor();
        if high_floor == high {
            high_floor -= 1.0;
        }
        high_floor as u64 - low_ceil as u64 + 1
    }
}

#[derive(Debug)]
struct RaceHistory {
    races: Vec<Race>,
}

impl FromStr for RaceHistory {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().filter(|l| !l.trim().is_empty()).collect();
        assert_eq!(lines.len(), 2);

        let (_, times) = lines[0].split_once("Time:").unwrap();
        let times: Vec<u64> = times
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect();

        let (_, distances) = lines[1].split_once("Distance:").unwrap();
        let distances: Vec<u64> = distances
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect();

        assert_eq!(times.len(), distances.len());

        let races = times
            .iter()
            .zip(distances.iter())
            .map(|(t, d)| Race {
                time: *t,
                distance: *d,
            })
            .collect();

        Ok(RaceHistory { races })
    }
}

impl RaceHistory {
    fn num_ways_to_win(&self) -> u64 {
        self.races
            .iter()
            .map(|race| race.num_ways_to_win())
            .product()
    }
}

#[derive(Debug)]
struct RaceHistoryB {
    race: Race,
}

impl FromStr for RaceHistoryB {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().filter(|l| !l.trim().is_empty()).collect();
        assert_eq!(lines.len(), 2);

        let (_, time) = lines[0].split_once("Time:").unwrap();
        let time = time
            .chars()
            .filter(|c| c.is_numeric())
            .collect::<String>()
            .parse::<u64>()
            .unwrap();

        let (_, distance) = lines[1].split_once("Distance:").unwrap();
        let distance = distance
            .chars()
            .filter(|c| c.is_numeric())
            .collect::<String>()
            .parse::<u64>()
            .unwrap();

        let race = Race { time, distance };
        Ok(RaceHistoryB { race })
    }
}

impl RaceHistoryB {
    fn num_ways_to_win(&self) -> u64 {
        self.race.num_ways_to_win()
    }
}

impl AocSolution for Day06 {
    fn part1(&self, input: &str) -> String {
        input
            .parse::<RaceHistory>()
            .expect("Failed to parse RaceHistory")
            .num_ways_to_win()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        input
            .parse::<RaceHistoryB>()
            .expect("Failed to parse RaceHistoryB")
            .num_ways_to_win()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"Time:      7  15   30
Distance:  9  40  200"#;

    #[test]
    fn test_part1_example() {
        assert_eq!(Day06.part1(EXAMPLE), "288");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2023, 6).expect("Failed to get input");
        assert_eq!(Day06.part1(&input), "74698");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(Day06.part2(EXAMPLE), "71503");
    }

    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2023, 6).expect("Failed to get input");
        assert_eq!(Day06.part2(&input), "27563421");
    }
}
