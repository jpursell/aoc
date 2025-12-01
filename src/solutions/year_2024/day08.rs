use std::{
    collections::{btree_map::Entry, BTreeMap, BTreeSet},
    str::FromStr,
};

use crate::AocSolution;

pub struct Day08;

#[derive(Debug)]
struct Puzzle {
    shape: [usize; 2],
    antennas: BTreeMap<char, Vec<[usize; 2]>>,
}

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nrows = s.lines().count();
        let ncols = s.lines().next().unwrap().chars().count();
        let shape = [nrows, ncols];
        let mut antennas = BTreeMap::new();
        for (irow, line) in s.lines().enumerate() {
            for (icol, c) in line.chars().enumerate() {
                if c == '.' {
                    continue;
                }
                match antennas.entry(c) {
                    Entry::Vacant(vacant_entry) => {
                        vacant_entry.insert(vec![[irow, icol]]);
                    }
                    Entry::Occupied(mut occupied_entry) => {
                        occupied_entry.get_mut().push([irow, icol]);
                    }
                }
            }
        }
        Ok(Puzzle { shape, antennas })
    }
}

impl Puzzle {
    fn process_p1(&mut self) -> usize {
        let mut antinodes = BTreeSet::<[usize; 2]>::new();
        for antenna_positions in self.antennas.values() {
            for start in antenna_positions {
                for end in antenna_positions {
                    if start == end {
                        continue;
                    }
                    let start = start.map(|x| x as i64);
                    let end = end.map(|x| x as i64);
                    let diff = [end[0] - start[0], end[1] - start[1]];
                    let antinode = [end[0] + diff[0], end[1] + diff[1]];
                    if antinode[0] < 0 || antinode[1] < 0 {
                        continue;
                    }
                    let antinode = antinode.map(|x| x as usize);
                    if antinode[0] >= self.shape[0] || antinode[1] >= self.shape[1] {
                        continue;
                    }
                    antinodes.insert(antinode);
                }
            }
        }
        antinodes.len()
    }

    fn process_p2(&mut self) -> usize {
        let mut antinodes = BTreeSet::<[usize; 2]>::new();
        for antenna_positions in self.antennas.values() {
            for start in antenna_positions {
                for end in antenna_positions {
                    if start == end {
                        continue;
                    }
                    let start = start.map(|x| x as i64);
                    let end = end.map(|x| x as i64);
                    let diff = [end[0] - start[0], end[1] - start[1]];
                    let mut harmonic = 1;
                    loop {
                        let antinode =
                            [start[0] + diff[0] * harmonic, start[1] + diff[1] * harmonic];
                        if antinode[0] < 0 || antinode[1] < 0 {
                            break;
                        }
                        let antinode = antinode.map(|x| x as usize);
                        if antinode[0] >= self.shape[0] || antinode[1] >= self.shape[1] {
                            break;
                        }
                        antinodes.insert(antinode);
                        harmonic += 1;
                    }
                }
            }
        }
        antinodes.len()
    }
}

impl AocSolution for Day08 {
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

    const EXAMPLE: &str = r"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_part1_example() {
        assert_eq!(Day08.part1(EXAMPLE), "14");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2024, 8).expect("Failed to get input");
        assert_eq!(Day08.part1(&input), "392");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(Day08.part2(EXAMPLE), "34");
    }

    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2024, 8).expect("Failed to get input");
        assert_eq!(Day08.part2(&input), "1235");
    }
}
