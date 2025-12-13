use crate::AocSolution;
use itertools::{iproduct, Itertools};
use ndarray::prelude::*;

const NPIECES: u8 = 6;

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
    shape_sizes: Vec<u8>,
    shapes: Vec<Shape>,
    trees: Vec<Tree>,
}

#[derive(Debug, PartialEq, Eq)]
struct State {
    flip: bool,
    piece: u8,
    position: u16,
    rotation: u8,
}

#[derive(Debug, Clone, Copy)]
struct Placer {
    ended: bool,
    flip: bool,
    piece: u8,
    position: u16,
    rotation: u8,
    tree_size: u16,
}
impl Iterator for Placer {
    type Item = State;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ended {
            return None;
        }

        let state = State {
            flip: self.flip,
            piece: self.piece,
            position: self.position,
            rotation: self.rotation,
        };

        // flip [false, true]
        self.flip = !self.flip;
        if self.flip {
            return Some(state);
        }

        // piece: 0..NPIECES
        self.piece += 1;
        self.piece %= NPIECES;
        if self.piece > 0 {
            return Some(state);
        }

        // position: 0..tree_size
        self.position += 1;
        self.position %= self.tree_size;
        if self.position > 0 {
            return Some(state);
        }

        // rotation: 0..4
        self.rotation += 1;
        if self.rotation == 4 {
            self.ended = true;
        }

        Some(state)
    }
}
impl Placer {
    fn new(tree_size: u16) -> Self {
        Placer {
            tree_size,
            ended: false,
            flip: false,
            piece: 0,
            position: 0,
            rotation: 0,
        }
    }
    fn reset(&mut self) {
        self.ended = false;
        self.flip = false;
        self.piece = 0;
        self.position = 0;
        self.rotation = 0;
    }
}
impl Puzzle {
    fn new(shapes: Vec<Shape>, trees: Vec<Tree>) -> Self {
        let shape_sizes: Vec<u8> = shapes
            .iter()
            .map(|s| s.grid.iter().map(|&x| if x { 1 } else { 0 }).sum::<u8>())
            .collect();

        Puzzle {
            shapes,
            trees,
            shape_sizes,
        }
    }
    fn shapes_fit(&self, itree: usize) -> bool {
        let tree = &self.trees[itree];
        let total_required = self
            .shape_sizes
            .iter()
            .zip(tree.shape_counts.iter())
            .map(|(&shape_size, &nshapes)| shape_size as u16 * nshapes as u16)
            .sum::<u16>();
        let tree_nelem: u16 = tree.shape.iter().map(|&x| x as u16).product();
        if tree_nelem < total_required {
            return false;
        }
        let n_extra = tree_nelem - total_required;
        // TODO : going to have Vec<Placer> that is initialized once with len equal to num pieces
        // i.e max of about 50*6
        // They are iterators and they yield a state, but maybe they should yield the values for the next deep iterator?
        // I'm going to go with state for now.
        //
        // There will be an indicator as to how deep into the iterator stack we are
        //
        // There will be a gird of bool the same size as tree and when a piece is selected it will get updated
        //
        // Iterator will return None when all posibilities are exausted
        //
        // State must be checked. If valid, move stack pointer over and keep going, else get another state from iterator.
        //
        // To check a new state
        // - make sure its location is not off the board
        // - make new piece is next to an existing piece with 4-connectivity? I'm not sure this is a good idea
        // What if last piece only fits in 8-connectivity?
        // - make sure it won't intersec an already place piece
        // position is flat location of upper left.
        // - not using too many of 1 piece
        //
        let mut stack =
            vec![Placer::new(tree_nelem); tree.shape_counts.iter().map(|&c| c as usize).product()];
        loop {
            break;
        }

        true
    }
    fn part1(&self) -> String {
        (0..self.trees.len())
            .into_iter()
            .map(|itree| if self.shapes_fit(itree) { 1 } else { 0 })
            .sum::<u64>()
            .to_string()
    }
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
    Puzzle::new(shapes, trees)
}

pub struct Day12;

impl AocSolution for Day12 {
    fn part1(&self, input: &str) -> String {
        parse(input).part1()
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
    fn test_placer() {
        let tree_size = 3;
        let mut placer = Placer::new(tree_size);
        let placer_vec: Vec<State> = (&mut placer).collect();
        placer.reset();
        let placer_vec_2: Vec<State> = placer.collect();
        let product_vec: Vec<State> =
            iproduct!(0..4, 0..tree_size, 0..NPIECES, [false, true].iter())
                .map(|(rotation, position, piece, &flip)| State {
                    flip,
                    piece,
                    position,
                    rotation,
                })
                .collect();
        if placer_vec.len() != product_vec.len() {
            for i in 0..placer_vec.len().max(product_vec.len()) {
                eprintln!(
                    "[{}] equal: {} placer: {:?}, product: {:?}",
                    i,
                    placer_vec.get(i) == product_vec.get(i),
                    placer_vec.get(i),
                    product_vec.get(i)
                );
            }
        }
        assert_eq!(placer_vec.len(), product_vec.len());
        assert_eq!(placer_vec_2.len(), product_vec.len());
        for (placer_state, product_state) in placer_vec.iter().zip(product_vec.iter()) {
            assert_eq!(placer_state, product_state);
        }
        for (placer_state, product_state) in placer_vec_2.iter().zip(product_vec.iter()) {
            assert_eq!(placer_state, product_state);
        }
    }

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
