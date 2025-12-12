use crate::AocSolution;
use itertools::Itertools;
use ndarray::prelude::*;

#[derive(Debug)]
struct Shape {
    grid: Array2<bool>,
}
#[derive(Debug)]
struct Tree {
    shape: [u8; 2],
    shape_counts: Vec<u8>,
}
#[derive(Debug)]
struct Puzzle {
    shapes: Vec<Shape>,
    trees: Vec<Tree>,
}
fn parse(input: &str) -> Puzzle {
    let mut lines = input.lines();
    let nshapes = 6;
    let shape_size = (3, 3);
    let shape_numel = shape_size.0 * shape_size.1;
    let shapes = (&mut lines)
        .chunks(5)
        .into_iter()
        .enumerate()
        .take(nshapes)
        .map(|(index, shape_lines)| {
            let mut shape_lines = shape_lines.into_iter();
            let mut index_chars = shape_lines.next().unwrap().chars();
            assert_eq!(
                index_chars.next().unwrap(),
                char::from_digit(index as u32, 10).expect("failed to decode index")
            );
            assert_eq!(index_chars.next().unwrap(), ':');
            assert_eq!(index_chars.next(), None);
            let mut shape: Vec<bool> = Vec::with_capacity(shape_numel);
            shape_lines.take(3).for_each(|line| {
                line.chars().for_each(|c| match c {
                    '.' => shape.push(false),
                    '#' => shape.push(true),
                    _ => panic!(),
                })
            });
            Shape {
                grid: Array2::from_shape_vec(shape_size, shape).unwrap(),
            }
        })
        .collect();

    let trees = lines
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (shape, shape_counts) = line.split_once(": ").expect("Could not split tree line");
            let (xshape, yshape) = shape.split_once("x").expect("Count not split tree shape");
            let shape = [
                xshape.parse().expect("could not parse x shape"),
                yshape.parse().expect("could not parse y shape"),
            ];
            let shape_counts = shape_counts
                .split_whitespace()
                .map(|s| s.parse().expect("coult not parse count"))
                .collect();
            Tree {
                shape,
                shape_counts,
            }
        })
        .collect();
    Puzzle { shapes, trees }
}

fn shapes_fit(tree: &Tree, shapes: &[Shape]) -> bool {
    let shape_sizes: Vec<u8> = shapes
        .iter()
        .map(|s| s.grid.iter().map(|&x| if x { 1 } else { 0 }).sum::<u8>())
        .collect();
    let total_required = shape_sizes
        .iter()
        .zip(tree.shape_counts.iter())
        .map(|(&shape_size, &nshapes)| shape_size as u16 * nshapes as u16)
        .sum::<u16>();
    let tree_nelem: u16 = tree.shape.iter().map(|&x| x as u16).product();
    if tree_nelem < total_required {
        return false;
    }

    true
}

pub struct Day12;

impl AocSolution for Day12 {
    fn part1(&self, input: &str) -> String {
        let Puzzle { shapes, trees } = parse(input);
        trees
            .iter()
            .map(|tree| if shapes_fit(tree, &shapes) { 1 } else { 0 })
            .sum::<u64>()
            .to_string()
    }

    fn part2(&self, _input: &str) -> String {
        "Not implemented".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";

    #[test]
    fn test_part1_example() {
        assert_eq!(Day12.part1(EXAMPLE), "2");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2025, 12).expect("Failed to get input");
        assert_eq!(Day12.part1(&input), "REPLACE_WITH_PART1_FULL_RESULT");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(Day12.part2(EXAMPLE), "REPLACE_WITH_PART2_EXAMPLE_RESULT");
    }

    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2025, 12).expect("Failed to get input");
        assert_eq!(Day12.part2(&input), "REPLACE_WITH_PART2_FULL_RESULT");
    }
}
