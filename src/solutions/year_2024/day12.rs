use crate::AocSolution;
use ndarray::prelude::*;
use std::{
    collections::{BTreeMap, BTreeSet},
    str::FromStr,
};

pub struct Day12;

// Common functions and structs
fn label_image(map: ArrayView2<char>) -> (Array2<usize>, usize) {
    let mut out = Array2::zeros(map.raw_dim());
    let mut label = 0;
    let shape = out.shape().to_vec();
    // init rows with 1 1d label images
    for irow in 0..shape[0] {
        label += 1;
        out[[irow, 0]] = label;
        for icol in 1..shape[1] {
            if map[[irow, icol]] != map[[irow, icol - 1]] {
                label += 1;
            }
            out[[irow, icol]] = label;
        }
    }
    // connect regions
    let mut label_region = BTreeMap::new();
    let mut regions = Vec::new();
    for icol in 0..shape[1] {
        for irow in 1..shape[0] {
            let above_label = out[[irow - 1, icol]];
            let above_char = map[[irow - 1, icol]];
            let current_label = out[[irow, icol]];
            let current_char = map[[irow, icol]];
            let mut add_key = |key| {
                label_region.entry(key).or_insert_with(|| {
                    regions.push(BTreeSet::from([key]));
                    regions.len() - 1
                });
            };
            add_key(above_label);
            add_key(current_label);
            if above_char != current_char {
                continue;
            }
            let above_region = label_region[&above_label];
            let current_region = label_region[&current_label];
            // already merged?
            if above_region == current_region {
                continue;
            }
            // merge
            for key in &regions[current_region] {
                *label_region.get_mut(key).unwrap() = above_region;
            }
            if above_region < current_region {
                let (left_regions, right_regions) = regions.split_at_mut(current_region);
                left_regions[above_region].append(&mut right_regions[0]);
            } else {
                let (left_regions, right_regions) = regions.split_at_mut(above_region);
                right_regions[0].append(&mut left_regions[current_region]);
            }
        }
    }
    // remove empty regions
    let regions = regions
        .into_iter()
        .filter(|r| !r.is_empty())
        .collect::<Vec<_>>();
    for (iregion, region) in regions.iter().enumerate() {
        for key in region {
            *label_region.get_mut(key).unwrap() = iregion;
        }
    }
    out.iter_mut().for_each(|x| *x = label_region[x]);
    (out, regions.len())
}

#[derive(Debug)]
struct Puzzle {
    map: Array2<char>,
}

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nrows = s.lines().count();
        if nrows == 0 {
            return Ok(Puzzle {
                map: Array2::from_shape_vec((0, 0), vec![]).unwrap(),
            });
        }
        let ncols = s.lines().next().unwrap().chars().count();
        let mut map = Vec::with_capacity(nrows * ncols);
        for line in s.lines() {
            for c in line.chars() {
                map.push(c);
            }
        }
        let map = Array2::from_shape_vec((nrows, ncols), map).unwrap();
        Ok(Puzzle { map })
    }
}

impl Puzzle {
    fn process_part1(&self) -> usize {
        struct RegionPart1 {
            area: usize,
            perimeter: usize,
        }

        let (labels, num_regions) = label_image(self.map.view());
        let mut regions = Vec::with_capacity(num_regions);
        for _ in 0..num_regions {
            regions.push(RegionPart1 {
                area: 0,
                perimeter: 0,
            });
        }
        let shape = labels.shape().to_vec();
        for (pos, &label) in labels.indexed_iter() {
            let pos = [pos.0, pos.1];
            let region = &mut regions[label];
            region.area += 1;
            // check up
            if pos[0] == 0 || labels[[pos[0] - 1, pos[1]]] != label {
                region.perimeter += 1;
            }
            // check down
            if pos[0] == shape[0] - 1 || labels[[pos[0] + 1, pos[1]]] != label {
                region.perimeter += 1;
            }
            // check left
            if pos[1] == 0 || labels[[pos[0], pos[1] - 1]] != label {
                region.perimeter += 1;
            }
            // check right
            if pos[1] == shape[1] - 1 || labels[[pos[0], pos[1] + 1]] != label {
                region.perimeter += 1;
            }
        }
        regions.iter().map(|r| r.area * r.perimeter).sum()
    }

    fn process_part2(&self) -> usize {
        #[derive(Debug)]
        struct RegionPart2 {
            area: usize,
            sides: usize,
        }

        let (labels, num_regions) = label_image(self.map.view());
        let mut regions = Vec::with_capacity(num_regions);
        for _ in 0..num_regions {
            regions.push(RegionPart2 { area: 0, sides: 0 });
        }
        for &label in labels.iter() {
            regions.get_mut(label).unwrap().area += 1;
        }
        let shape = labels.shape().to_vec();

        // check vertical sides
        {
            // top
            for icol in 0..shape[1] {
                let current_label = labels[[0, icol]];

                // left
                if icol == 0 || labels[[0, icol - 1]] != current_label {
                    regions[current_label].sides += 1;
                }

                // right
                if icol == shape[1] - 1 || labels[[0, icol + 1]] != current_label {
                    regions[current_label].sides += 1;
                }
            }

            for irow in 1..shape[0] {
                for icol in 0..shape[1] {
                    let current_label = &labels[[irow, icol]];

                    let left_label = if icol > 0 {
                        labels.get([irow, icol - 1])
                    } else {
                        None
                    };
                    let right_label = labels.get([irow, icol + 1]);
                    let up_label = labels.get([irow - 1, icol]).unwrap();
                    let up_left_label = if icol > 0 {
                        labels.get([irow - 1, icol - 1])
                    } else {
                        None
                    };
                    let up_right_label = labels.get([irow - 1, icol + 1]);

                    // left
                    if let Some(left_label) = left_label {
                        if left_label != current_label
                            && (up_label != current_label
                                || up_left_label.unwrap() == current_label)
                        {
                            regions[*current_label].sides += 1;
                        }
                    } else {
                        // on left edge of image
                        if up_label != current_label {
                            regions[*current_label].sides += 1;
                        }
                    }

                    // right
                    if let Some(right_label) = right_label {
                        if right_label != current_label
                            && (up_label != current_label
                                || current_label == up_right_label.unwrap())
                        {
                            regions[*current_label].sides += 1;
                        }
                    } else {
                        // on right edge of image
                        if up_label != current_label {
                            regions[*current_label].sides += 1;
                        }
                    }
                }
            }
        }

        // check horizontal sides
        {
            // left
            for irow in 0..shape[0] {
                let current_label = labels[[irow, 0]];

                // top
                if irow == 0 || labels[[irow - 1, 0]] != current_label {
                    regions[current_label].sides += 1;
                }

                // bottom
                if irow == shape[0] - 1 || labels[[irow + 1, 0]] != current_label {
                    regions[current_label].sides += 1;
                }
            }

            for icol in 1..shape[1] {
                for irow in 0..shape[0] {
                    let current_label = &labels[[irow, icol]];
                    let top_label = if irow > 0 {
                        labels.get([irow - 1, icol])
                    } else {
                        None
                    };
                    let bottom_label = labels.get([irow + 1, icol]);
                    let left_label = labels.get([irow, icol - 1]).unwrap();
                    let up_left_label = if irow > 0 {
                        labels.get([irow - 1, icol - 1])
                    } else {
                        None
                    };
                    let bottom_left_label = labels.get([irow + 1, icol - 1]);

                    // top
                    if let Some(top_label) = top_label {
                        if top_label != current_label
                            && (left_label != current_label
                                || current_label == up_left_label.unwrap())
                        {
                            regions[*current_label].sides += 1;
                        }
                    } else {
                        // on top edge of image
                        if left_label != current_label {
                            regions[*current_label].sides += 1;
                        }
                    }

                    // bottom
                    if let Some(bottom_label) = bottom_label {
                        if bottom_label != current_label
                            && (left_label != current_label
                                || current_label == bottom_left_label.unwrap())
                        {
                            regions[*current_label].sides += 1;
                        }
                    } else {
                        // on bottom edge of image
                        if left_label != current_label {
                            regions[*current_label].sides += 1;
                        }
                    }
                }
            }
        }
        regions.iter().map(|r| r.area * r.sides).sum()
    }
}

impl AocSolution for Day12 {
    fn part1(&self, input: &str) -> String {
        let puzzle: Puzzle = input.parse().unwrap();
        puzzle.process_part1().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let puzzle: Puzzle = input.parse().unwrap();
        puzzle.process_part2().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_A: &str = r#"AAAA
BBCD
BBCC
EEEC"#;
    const TEST_B: &str = r#"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"#;
    const TEST_C: &str = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#;
    const TEST_E: &str = r#"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE"#;
    const TEST_M: &str = r#"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA"#;

    #[test]
    fn test_part1_test_a() {
        assert_eq!(Day12.part1(TEST_A), "140");
    }

    #[test]
    fn test_part1_test_b() {
        assert_eq!(Day12.part1(TEST_B), "772");
    }

    #[test]
    fn test_part1_test_c() {
        assert_eq!(Day12.part1(TEST_C), "1930");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2024, 12).expect("Failed to get input");
        assert_eq!(Day12.part1(&input), "1456082");
    }

    #[test]
    fn test_part2_test_a() {
        assert_eq!(Day12.part2(TEST_A), "80");
    }

    #[test]
    fn test_part2_test_b() {
        assert_eq!(Day12.part2(TEST_B), "436");
    }

    #[test]
    fn test_part2_test_e() {
        assert_eq!(Day12.part2(TEST_E), "236");
    }

    #[test]
    fn test_part2_test_m() {
        assert_eq!(Day12.part2(TEST_M), "368");
    }

    #[test]
    fn test_part2_test_c() {
        assert_eq!(Day12.part2(TEST_C), "1206");
    }

    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2024, 12).expect("Failed to get input");
        assert_eq!(Day12.part2(&input), "872382");
    }
}
