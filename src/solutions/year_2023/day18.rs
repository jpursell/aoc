use crate::AocSolution;

pub struct Day18;

mod a {
    use std::{fmt::Display, str::FromStr};

    use ndarray::Array2;

    #[derive(Debug)]
    enum Direction {
        U,
        D,
        L,
        R,
    }

    impl FromStr for Direction {
        type Err = &'static str;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "U" => Ok(Direction::U),
                "D" => Ok(Direction::D),
                "L" => Ok(Direction::L),
                "R" => Ok(Direction::R),
                _ => Err("Unknown direction"),
            }
        }
    }

    #[derive(Debug)]
    struct DigInstruction {
        direction: Direction,
        distance: usize,
        _color: String,
    }

    impl FromStr for DigInstruction {
        type Err = &'static str;
        /// Read in something like "R 10 (#ffffff)"
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut s = s.split_whitespace();
            let direction = s.next().unwrap().parse::<Direction>().unwrap();
            let distance = s.next().unwrap().parse::<usize>().unwrap();
            let color = s.next().unwrap().to_string();
            Ok(DigInstruction {
                direction,
                distance,
                _color: color,
            })
        }
    }

    #[derive(Debug)]
    struct DigPlan {
        instructions: Vec<DigInstruction>,
    }

    impl FromStr for DigPlan {
        type Err = &'static str;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(DigPlan {
                instructions: s
                    .lines()
                    .map(|line| line.parse::<DigInstruction>().unwrap())
                    .collect::<Vec<_>>(),
            })
        }
    }

    #[derive(Debug, Clone, Copy)]
    struct Position {
        index: [i64; 2],
    }

    impl Display for Position {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            writeln!(f, "({}, {})", self.index[0], self.index[1])
        }
    }

    impl Position {
        fn new(row: i64, col: i64) -> Position {
            Position { index: [row, col] }
        }

        fn row(&self) -> i64 {
            self.index[0]
        }

        fn col(&self) -> i64 {
            self.index[1]
        }

        fn index_from(&self, position: &Position) -> [usize; 2] {
            [
                (self.row() - position.row()) as usize,
                (self.col() - position.col()) as usize,
            ]
        }

        fn move_by(&self, direction: &Direction, distance: &usize) -> Position {
            let distance = *distance as i64;
            match direction {
                Direction::D => Position::new(self.row() + distance, self.col()),
                Direction::U => Position::new(self.row() - distance, self.col()),
                Direction::L => Position::new(self.row(), self.col() - distance),
                Direction::R => Position::new(self.row(), self.col() + distance),
            }
        }

        fn max(&self, other: &Position) -> Position {
            Position::new(self.row().max(other.row()), self.col().max(other.col()))
        }

        fn min(&self, other: &Position) -> Position {
            Position::new(self.row().min(other.row()), self.col().min(other.col()))
        }
    }

    #[derive(Debug)]
    struct Lagoon {
        dug: Array2<bool>,
        offset: Position,
    }

    enum ElementType {
        Empty,
        UpperLeft,
        UpperRight,
        BottomLeft,
        BottomRight,
        Virtical,
        Horizontal,
    }

    impl Lagoon {
        fn check_area(dug: &Array2<bool>, irow: usize, icol: usize) -> ElementType {
            if !dug[[irow, icol]] {
                return ElementType::Empty;
            }
            let left = dug[[irow, icol - 1]];
            let right = dug[[irow, icol + 1]];
            let top = dug[[irow - 1, icol]];
            let bottom = dug[[irow + 1, icol]];

            if top && bottom {
                return ElementType::Virtical;
            }
            if left && right {
                return ElementType::Horizontal;
            }
            if top {
                if right {
                    return ElementType::BottomLeft;
                }
                if left {
                    return ElementType::BottomRight;
                }
            }
            if bottom {
                if right {
                    return ElementType::UpperLeft;
                }
                if left {
                    return ElementType::UpperRight;
                }
            }
            panic!()
        }
        fn new(dig_plan: &DigPlan) -> Lagoon {
            // find boundaries
            let (min_position, max_position) = {
                let mut position = Position::new(0, 0);
                let mut min_position = position;
                let mut max_position = position;
                dig_plan.instructions.iter().for_each(|instruction| {
                    position = position.move_by(&instruction.direction, &instruction.distance);
                    min_position = min_position.min(&position);
                    max_position = max_position.max(&position);
                });

                // pad out a little
                (
                    Position::new(min_position.row() - 1, min_position.col() - 1),
                    Position::new(max_position.row() + 1, max_position.col() + 1),
                )
            };

            // init data
            let nrows = usize::try_from(max_position.row() - min_position.row() + 1).unwrap();
            let ncols = usize::try_from(max_position.col() - min_position.col() + 1).unwrap();
            let mut dug = Array2::from_elem((nrows, ncols), false);

            // dig trench
            {
                let mut position = Position::new(0, 0);
                dig_plan.instructions.iter().for_each(|instruction| {
                    (1..=instruction.distance).for_each(|_| {
                        position = position.move_by(&instruction.direction, &1);
                        dug[position.index_from(&min_position)] = true;
                    })
                });
            }

            let trench = dug.clone();

            // fill trench
            for irow in 1..(nrows - 1) {
                let mut inside = false;
                let mut last = None;
                for icol in 1..(ncols - 1) {
                    match Lagoon::check_area(&trench, irow, icol) {
                        ElementType::BottomLeft => {
                            // println!("{} {} BL", irow, icol);
                            assert!(last.is_none());
                            last = Some(ElementType::BottomLeft);
                        }
                        ElementType::Empty => (),
                        ElementType::UpperLeft => {
                            // println!("{} {} UL", irow, icol);
                            assert!(last.is_none());
                            last = Some(ElementType::UpperLeft)
                        }
                        ElementType::UpperRight => {
                            // println!("{} {} UR", irow, icol);
                            match last {
                                Some(ElementType::BottomLeft) => {
                                    inside = !inside;
                                    last = None;
                                }
                                Some(ElementType::UpperLeft) => {
                                    last = None;
                                }
                                _ => panic!(),
                            }
                        }
                        ElementType::BottomRight => {
                            // println!("{} {} BR", irow, icol);
                            match last {
                                Some(ElementType::BottomLeft) => {
                                    last = None;
                                }
                                Some(ElementType::UpperLeft) => {
                                    inside = !inside;
                                    last = None;
                                }
                                _ => panic!(),
                            }
                        }
                        ElementType::Virtical => {
                            // println!("{} {} V", irow, icol);
                            inside = !inside;
                        }
                        ElementType::Horizontal => (),
                    }
                    if inside {
                        dug[[irow, icol]] = true;
                    }
                }
            }

            Lagoon {
                dug,
                offset: min_position,
            }
        }

        fn count(&self) -> usize {
            self.dug.iter().filter(|e| **e).count()
        }
    }

    impl Display for Lagoon {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            writeln!(f, "Lagoon: Offset: {}", self.offset).unwrap();
            self.dug.rows().into_iter().for_each(|row| {
                row.iter().for_each(|elem| match elem {
                    true => write!(f, "#").unwrap(),
                    false => write!(f, ".").unwrap(),
                });
                writeln!(f, "").unwrap();
            });
            Ok(())
        }
    }

    pub fn run(input: &str) -> usize {
        let plan = input.parse::<DigPlan>().unwrap();
        let lagoon = Lagoon::new(&plan);
        println!("{}", lagoon);
        lagoon.count()
    }
}

mod b {
    use std::{fmt::Display, str::FromStr};

    #[derive(Debug)]
    enum Direction {
        U,
        D,
        L,
        R,
    }

    impl Display for Direction {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Direction::U => write!(f, "U"),
                Direction::D => write!(f, "D"),
                Direction::L => write!(f, "L"),
                Direction::R => write!(f, "R"),
            }
        }
    }

    impl FromStr for Direction {
        type Err = &'static str;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "0" => Ok(Direction::R),
                "1" => Ok(Direction::D),
                "2" => Ok(Direction::L),
                "3" => Ok(Direction::U),
                _ => Err("Unknown direction"),
            }
        }
    }

    #[derive(Debug)]
    struct DigInstruction {
        direction: Direction,
        distance: usize,
    }

    impl Display for DigInstruction {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} {}", self.direction, self.distance)
        }
    }

    impl FromStr for DigInstruction {
        type Err = &'static str;
        /// Read in something like "R 10 (#ffffff)"
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut s = s.split_whitespace().skip(2).next().unwrap();
            s = &s[2..8];
            let (distance, direction) = s.split_at(5);
            let distance = usize::from_str_radix(distance, 16).unwrap();
            let direction = direction.parse::<Direction>().unwrap();
            Ok(DigInstruction {
                direction,
                distance,
            })
        }
    }

    #[derive(Debug)]
    struct DigPlan {
        instructions: Vec<DigInstruction>,
    }

    impl Display for DigPlan {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.instructions
                .iter()
                .for_each(|x| writeln!(f, "{}", x).unwrap());
            Ok(())
        }
    }

    impl FromStr for DigPlan {
        type Err = &'static str;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(DigPlan {
                instructions: s
                    .lines()
                    .map(|line| line.parse::<DigInstruction>().unwrap())
                    .collect::<Vec<_>>(),
            })
        }
    }

    #[derive(Debug, Clone, Copy)]
    struct Position {
        index: [i64; 2],
    }

    impl Display for Position {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({}, {})", self.index[0], self.index[1])
        }
    }

    impl Position {
        fn new(row: i64, col: i64) -> Position {
            Position { index: [row, col] }
        }

        fn row(&self) -> i64 {
            self.index[0]
        }

        fn col(&self) -> i64 {
            self.index[1]
        }

        fn move_by(&self, direction: &Direction, distance: &usize) -> Position {
            let distance = *distance as i64;
            match direction {
                Direction::D => Position::new(self.row() + distance, self.col()),
                Direction::U => Position::new(self.row() - distance, self.col()),
                Direction::L => Position::new(self.row(), self.col() - distance),
                Direction::R => Position::new(self.row(), self.col() + distance),
            }
        }
    }

    #[derive(Debug)]
    struct PolyLagoon {
        segments: Vec<Segment>,
    }

    #[derive(Debug)]
    struct Segment {
        points: [Position; 2],
    }

    impl Display for Segment {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}:{}", self.points[0], self.points[1])
        }
    }

    impl Segment {
        fn new(a: Position, b: Position) -> Segment {
            let points = [a, b];
            Segment { points }
        }
        fn rows(&self) -> [i64; 2] {
            [self.points[0].row(), self.points[1].row()]
        }
        fn cols(&self) -> [i64; 2] {
            [self.points[0].col(), self.points[1].col()]
        }
        fn vertical(&self) -> bool {
            let rows = self.rows();
            let out = rows[0] != rows[1];
            if out {
                let cols = self.cols();
                assert_eq!(cols[0], cols[1]);
            }
            out
        }
        fn contains(&self, row: i64) -> bool {
            let rows = {
                let mut rows = self.rows();
                if rows[1] < rows[0] {
                    rows.swap(1, 0);
                }
                rows
            };
            if row < rows[0] || row >= rows[1] {
                return false;
            }
            true
        }
    }

    impl PolyLagoon {
        fn new(dig_plan: &DigPlan) -> PolyLagoon {
            // create vertices
            let mut position = Position::new(0, 0);
            let mut vertices = Vec::new();
            dig_plan
                .instructions
                .iter()
                .enumerate()
                .for_each(|(i, instruction)| {
                    let next_instruction = match dig_plan.instructions.get(i + 1) {
                        Some(ni) => ni,
                        None => dig_plan.instructions.first().unwrap(),
                    };
                    position = position.move_by(&instruction.direction, &instruction.distance);
                    match instruction.direction {
                        Direction::L => match next_instruction.direction {
                            Direction::U => {
                                vertices.push(Position::new(position.row() + 1, position.col()));
                            }
                            Direction::D => {
                                vertices
                                    .push(Position::new(position.row() + 1, position.col() + 1));
                            }
                            _ => panic!(),
                        },
                        Direction::U => match next_instruction.direction {
                            Direction::L => {
                                vertices.push(Position::new(position.row() + 1, position.col()));
                            }
                            Direction::R => {
                                vertices.push(Position::new(position.row(), position.col()));
                            }
                            _ => panic!(),
                        },
                        Direction::D => match next_instruction.direction {
                            Direction::L => {
                                vertices
                                    .push(Position::new(position.row() + 1, position.col() + 1));
                            }
                            Direction::R => {
                                vertices.push(Position::new(position.row(), position.col() + 1));
                            }
                            _ => panic!(),
                        },
                        Direction::R => match next_instruction.direction {
                            Direction::U => {
                                vertices.push(Position::new(position.row(), position.col()));
                            }
                            Direction::D => {
                                vertices.push(Position::new(position.row(), position.col() + 1));
                            }
                            _ => panic!(),
                        },
                    }
                });

            // vertices.iter().for_each(|v| println!("{}", v));

            let mut segments = vertices
                .windows(2)
                .map(|positions| Segment::new(positions[0], positions[1]))
                .collect::<Vec<_>>();
            segments.push(Segment::new(
                *vertices.first().unwrap(),
                *vertices.last().unwrap(),
            ));
            let segments = segments
                .into_iter()
                .filter(|s| s.vertical())
                .collect::<Vec<_>>();

            PolyLagoon { segments }
        }

        fn sorted_rows(&self) -> Vec<i64> {
            let mut rows = self
                .segments
                .iter()
                .map(|s| s.rows())
                .collect::<Vec<_>>()
                .concat();
            rows.sort();
            rows
        }

        fn count(&self) -> usize {
            let rows = self.sorted_rows();
            let mut area = 0;
            for row_slice in rows.windows(2) {
                if row_slice[0] == row_slice[1] {
                    continue;
                }
                let cols = {
                    let mut cols = self
                        .segments
                        .iter()
                        .filter(|s| s.contains(row_slice[0]))
                        .map(|s| s.cols()[0])
                        .collect::<Vec<_>>();
                    cols.sort();
                    cols
                };
                assert_eq!(cols.len() % 2, 0);
                let width = cols
                    .chunks(2)
                    .map(|c| c[1] - c[0])
                    .inspect(|w| assert!(*w > 0))
                    .map(|w| w as usize)
                    .sum::<usize>();
                let length = row_slice[1] - row_slice[0];
                assert!(length > 0);
                area += width * length as usize;
            }
            area
        }
    }

    impl Display for PolyLagoon {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.segments
                .iter()
                .for_each(|x| writeln!(f, "{}", x).unwrap());
            Ok(())
        }
    }

    pub fn run(input: &str) -> usize {
        let plan = input.parse::<DigPlan>().unwrap();
        let lagoon = PolyLagoon::new(&plan);
        // println!("{}", lagoon);
        lagoon.count()
    }
}

impl AocSolution for Day18 {
    fn part1(&self, input: &str) -> String {
        a::run(input).to_string()
    }

    fn part2(&self, input: &str) -> String {
        b::run(input).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"#;

    #[test]
    fn test_part1_example() {
        assert_eq!(Day18.part1(EXAMPLE), "62");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2023, 18).expect("Failed to get input");
        assert_eq!(Day18.part1(&input), "48503");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(Day18.part2(EXAMPLE), "952408144115");
    }

    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2023, 18).expect("Failed to get input");
        assert_eq!(Day18.part2(&input), "148442153147147");
    }
}
