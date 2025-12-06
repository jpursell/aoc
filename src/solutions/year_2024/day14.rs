use crate::AocSolution;
use ndarray::prelude::*;
use std::str::FromStr;

pub struct Day14;

#[derive(Debug)]
struct Robot {
    position: [i64; 2],
    velocity: [i64; 2],
}

impl Robot {
    fn position_after(&self, time: i64, room_size: [i64; 2]) -> [i64; 2] {
        [
            self.position_after_1d(0, time, room_size),
            self.position_after_1d(1, time, room_size),
        ]
    }
    fn position_after_1d(&self, axis: usize, time: i64, room_size: [i64; 2]) -> i64 {
        assert!(self.position[axis] >= 0);
        let p = self.position[axis] as u64;
        let mut v = self.velocity[axis];
        let r = room_size[axis] as u64;
        while v < 0 {
            v += r as i64;
        }
        let v = v as u64;
        ((p + v * time as u64) % r) as i64
    }
}

impl FromStr for Robot {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (position, velocity) = s.split_once(" ").unwrap();
        let parse_numbers = |s: &str| {
            let (_, s) = s.split_at(2);
            let (x, y) = s.split_once(",").unwrap();
            [x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap()]
        };
        let position = parse_numbers(position);
        let velocity = parse_numbers(velocity);
        Ok(Robot { position, velocity })
    }
}

#[derive(Debug)]
struct Puzzle {
    robots: Vec<Robot>,
}

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let count = s.lines().count();
        let mut robots = Vec::with_capacity(count);
        for line in s.lines() {
            robots.push(line.parse::<Robot>().unwrap());
        }
        Ok(Puzzle { robots })
    }
}

fn quadrant(position: [i64; 2], room_size: [i64; 2]) -> Option<usize> {
    assert!(position[0] >= 0);
    assert!(position[1] >= 0);
    assert!(position[0] < room_size[0]);
    assert!(position[1] < room_size[1]);
    let half = room_size.map(|x| (x - 1) / 2);
    match position[0].cmp(&half[0]) {
        std::cmp::Ordering::Less => match position[1].cmp(&half[1]) {
            std::cmp::Ordering::Less => Some(0),
            std::cmp::Ordering::Equal => None,
            std::cmp::Ordering::Greater => Some(1),
        },
        std::cmp::Ordering::Equal => None,
        std::cmp::Ordering::Greater => match position[1].cmp(&half[1]) {
            std::cmp::Ordering::Less => Some(2),
            std::cmp::Ordering::Equal => None,
            std::cmp::Ordering::Greater => Some(3),
        },
    }
}

fn symmetry(room: ArrayView2<bool>) -> usize {
    let mut out = 0;
    let shape = room.shape();
    for irow in 1..shape[0] {
        for icol in 1..shape[1] {
            let mut is_symmetric = true;
            for drow in 0..=irow.min(shape[0] - 1 - irow) {
                for dcol in 0..=icol.min(shape[1] - 1 - icol) {
                    if room[[irow - drow, icol - dcol]] != room[[irow + drow, icol + dcol]] {
                        is_symmetric = false;
                        break;
                    }
                }
                if !is_symmetric {
                    break;
                }
            }
            if is_symmetric {
                out += 1;
            }
        }
    }
    out
}

impl Puzzle {
    fn process_part1(&self, time: i64, room_size: [i64; 2]) -> usize {
        let mut quadrant_robot_counts = [0; 4];
        for robot in &self.robots {
            if let Some(quadrant) = quadrant(robot.position_after(time, room_size), room_size) {
                quadrant_robot_counts[quadrant] += 1;
            }
        }
        quadrant_robot_counts.iter().product::<usize>()
    }

    fn make_room(&self, time: i64, room_size: [i64; 2]) -> Array2<bool> {
        let shape = room_size.map(|x| x as usize);
        let mut room = Array2::from_elem(shape, false);
        for robot in &self.robots {
            *room
                .get_mut(robot.position_after(time, room_size).map(|x| x as usize))
                .unwrap() = true;
        }
        room
    }

    fn process_part2(&self, room_size: [i64; 2]) -> usize {
        let mut max_symmetry = 0;
        let mut time_of_max_symmetry = 0;
        let max_time = 10403;
        for time in 1..max_time {
            let room = self.make_room(time, room_size);
            let symmetry = symmetry(room.view());
            if symmetry >= max_symmetry {
                max_symmetry = symmetry;
                time_of_max_symmetry = time;
            }
        }
        time_of_max_symmetry as usize
    }
}

impl AocSolution for Day14 {
    fn part1(&self, input: &str) -> String {
        let puzzle: Puzzle = input.parse().unwrap();
        let time = 100;
        let room_size = [101, 103];
        puzzle.process_part1(time, room_size).to_string()
    }

    fn part2(&self, input: &str) -> String {
        let puzzle: Puzzle = input.parse().unwrap();
        let room_size = [101, 103];
        puzzle.process_part2(room_size).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"#;

    #[test]
    fn test_part1_example() {
        let puzzle: Puzzle = EXAMPLE.parse().unwrap();
        let time = 100;
        let room_size = [11, 7];
        assert_eq!(puzzle.process_part1(time, room_size), 12);
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2024, 14).expect("Failed to get input");
        assert_eq!(Day14.part1(&input), "211773366");
    }

    // Part 2 for day 14 is not a straightforward calculation, the original code
    // was for visual inspection. The implemented solution finds the time of max
    // symmetry, but there is no example to test against.
    // #[test]
    // fn test_part2_example() {
    //     let puzzle: Puzzle = EXAMPLE.parse().unwrap();
    //     let room_size = [11, 7];
    //     assert_eq!(puzzle.process_part2(room_size), 0);
    // }

    // This test takes too long to run.
    // #[test]
    // fn test_part2_full() {
    //     let input = crate::get_input_for_day(2024, 14).expect("Failed to get input");
    //     // The value is taken from the last printed value in the original 14b.rs
    //     assert_eq!(Day14.part2(&input), "10403");
    // }
}
