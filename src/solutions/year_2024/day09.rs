use std::str::FromStr;

use crate::AocSolution;

pub struct Day09;

#[derive(Debug)]
struct Puzzle {
    digits: Vec<usize>,
}

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let count = s.chars().count();
        let mut digits = Vec::with_capacity(count);
        for digit in s.chars() {
            digits.push(digit.to_string().parse::<usize>().unwrap());
        }
        Ok(Puzzle { digits })
    }
}

impl Puzzle {
    fn create_disc(&self) -> Vec<Option<usize>> {
        let mut disc = Vec::with_capacity(self.digits.iter().sum());
        let mut space = false;
        let mut id = 0;
        for digit in &self.digits {
            if space {
                for _ in 0..*digit {
                    disc.push(None);
                }
            } else {
                for _ in 0..*digit {
                    disc.push(Some(id));
                }
                id += 1;
            }
            space = !space;
        }
        disc
    }

    fn compact_disc_p1(disc: &mut [Option<usize>]) {
        let mut write_head = 0;
        let mut read_head = disc.len() - 1;
        loop {
            while write_head < disc.len() && disc[write_head].is_some() {
                write_head += 1;
            }
            if write_head >= disc.len() {
                break;
            } // All are Some, or write_head reached end

            while read_head > 0 && disc[read_head].is_none() {
                read_head -= 1;
            }
            if read_head <= write_head {
                break;
            }
            disc[write_head] = disc[read_head];
            disc[read_head] = None;
        }
    }

    fn find_first_space(disc: &[Option<usize>], size: usize, end: usize) -> Option<usize> {
        let mut start = None;
        let mut size_found = 0;
        for (i, block) in disc.iter().enumerate() {
            if i >= end {
                return None;
            }
            if block.is_some() {
                size_found = 0;
                start = None;
            } else {
                if start.is_none() {
                    start = Some(i);
                }
                size_found += 1;
                if size_found >= size {
                    return start;
                }
            }
        }
        None
    }

    fn compact_disc_p2(disc: &mut [Option<usize>]) {
        let mut read_head = disc.len() - 1;
        let start_id = disc[read_head].unwrap();
        for id in (1..=start_id).rev() {
            while read_head > 0 && (disc[read_head].is_none() || disc[read_head].unwrap() != id) {
                read_head -= 1;
            }
            if read_head == 0 {
                break;
            } // Reached start of disc
            let block_end = read_head + 1;
            while read_head > 0 && disc[read_head].is_some() && disc[read_head].unwrap() == id {
                read_head -= 1;
            }
            let block_start = if read_head == 0 && disc[0].is_some() && disc[0].unwrap() == id {
                0
            } else {
                read_head + 1
            };
            let block_length = block_end - block_start;

            let gap = Puzzle::find_first_space(disc, block_length, block_start);
            if gap.is_none() {
                continue;
            }
            let gap = gap.unwrap();
            for i in 0..block_length {
                disc[i + gap] = disc[i + block_start];
                disc[i + block_start] = None;
            }
        }
    }

    fn process_p1(&mut self) -> usize {
        let mut disc = self.create_disc();
        Puzzle::compact_disc_p1(&mut disc);
        disc.iter()
            .enumerate()
            .filter(|(_, id)| id.is_some())
            .map(|(index, id)| index * id.unwrap())
            .sum::<usize>()
    }

    fn process_p2(&mut self) -> usize {
        let mut disc = self.create_disc();
        Puzzle::compact_disc_p2(&mut disc);
        disc.iter()
            .enumerate()
            .filter(|(_, id)| id.is_some())
            .map(|(index, id)| index * id.unwrap())
            .sum::<usize>()
    }
}

impl AocSolution for Day09 {
    fn part1(&self, input: &str) -> String {
        let mut puzzle = input.parse::<Puzzle>().unwrap();
        puzzle.process_p1().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut puzzle = input.parse::<Puzzle>().unwrap();
        puzzle.process_p2().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"2333133121414131402";

    #[test]
    fn test_part1_example() {
        assert_eq!(Day09.part1(EXAMPLE), "1928");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2024, 9).expect("Failed to get input");
        assert_eq!(Day09.part1(&input), "6320029754031");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(Day09.part2(EXAMPLE), "2858");
    }

    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2024, 9).expect("Failed to get input");
        assert_eq!(Day09.part2(&input), "6347435485773");
    }
}
