use crate::AocSolution;

pub struct Day24;

mod a {
    use std::{fmt::Display, str::FromStr};

    #[derive(Clone, Copy)]
    struct Position {
        vec: [usize; 3],
    }
    impl Position {
        fn new(vec: [usize; 3]) -> Position {
            Position { vec }
        }
    }
    impl Display for Position {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}, {}, {}", self.vec[0], self.vec[1], self.vec[2])
        }
    }
    impl FromStr for Position {
        type Err = ();
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let vec = s
                .split(", ")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            assert_eq!(vec.len(), 3);
            let vec = [vec[0], vec[1], vec[2]];
            Ok(Position::new(vec))
        }
    }
    #[derive(Clone, Copy)]
    struct Velocity {
        vec: [i32; 3],
    }
    impl Velocity {
        fn new(vec: [i32; 3]) -> Velocity {
            vec.iter().for_each(|x| {
                assert_ne!(*x, 0);
            });
            Velocity { vec }
        }
        // fn norm_xy(&self) -> [f64; 2] {
        //     let out = [self.vec[0] as f64, self.vec[1] as f64];
        //     let norm = f64::sqrt(out[0].powi(2) + out[1].powi(2));
        //     [out[0] / norm, out[1] / norm]
        // }
    }
    impl Display for Velocity {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}, {}, {}", self.vec[0], self.vec[1], self.vec[2])
        }
    }
    impl FromStr for Velocity {
        type Err = ();
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let vec = s
                .split(", ")
                .map(|x| x.trim().parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            assert_eq!(vec.len(), 3);
            let vec = [vec[0], vec[1], vec[2]];
            Ok(Velocity::new(vec))
        }
    }
    #[derive(Clone, Copy)]
    struct InitialCondition {
        position: Position,
        velocity: Velocity,
    }
    impl InitialCondition {
        fn new(position: Position, velocity: Velocity) -> InitialCondition {
            InitialCondition { position, velocity }
        }
    }
    impl FromStr for InitialCondition {
        type Err = ();
        /// Parse things like 19, 13, 30 @ -2,  1, -2
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (pos, vel) = s.split_once(" @ ").unwrap();
            let pos = pos.parse::<Position>().unwrap();
            let vel = vel.parse::<Velocity>().unwrap();
            Ok(InitialCondition::new(pos, vel))
        }
    }
    impl Display for InitialCondition {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} @ {}", self.position, self.velocity)
        }
    }
    struct Line {
        m: f64,
        b: f64,
    }
    impl Line {
        fn new(m: f64, b: f64) -> Line {
            Line { m, b }
        }
    }
    impl Display for Line {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "y = {} x + {}", self.m, self.b)
        }
    }
    impl TryFrom<InitialCondition> for Line {
        type Error = ();
        fn try_from(value: InitialCondition) -> Result<Self, Self::Error> {
            // pos x,y moving at vel dx,dy
            // y = mx + b
            // when x is x we get
            // y = b
            let x = value.position.vec[0] as f64;
            let y = value.position.vec[1] as f64;
            let dx = value.velocity.vec[0] as f64;
            let dy = value.velocity.vec[1] as f64;
            let b = {
                // we want to find the value for t when x is 0
                // newx = x + dx * t
                // -x = dx * t
                // t = -x / dx
                let t = -x / dx;
                y + dy * t
            };
            if dx == 0_f64 {
                return Err(());
            }
            let m = dy / dx;
            Ok(Line::new(m, b))
        }
    }
    fn find_intersection(
        a_cond: &InitialCondition,
        a_line: &Line,
        b_cond: &InitialCondition,
        b_line: &Line,
    ) -> Option<[f64; 2]> {
        if a_line.m == b_line.m {
            return None;
        }
        // y = mx + b
        // m0 * x + b0 - m1 * x - b1 = 0
        // m0 * x - m1 * x = b1 - b0
        // x (m0 - m1) = b1 - b0
        // x = (b1 - b0) / (m0 - m1)
        let x = (b_line.b - a_line.b) / (a_line.m - b_line.m);
        // x = pos_x + vel_x * t
        // t = (x - pos_x) / vel_x
        let t_a = (x - a_cond.position.vec[0] as f64) / a_cond.velocity.vec[0] as f64;
        let t_b = (x - b_cond.position.vec[0] as f64) / b_cond.velocity.vec[0] as f64;
        if t_a < 0.0 || t_b < 0.0 {
            return None;
        }
        let y_a = a_line.m * x + a_line.b;
        // let y_b = b.m * x + b.b;
        // assert_eq!(y_a, y_b);
        Some([x, y_a])
    }
    struct HailCloud {
        stones: Vec<InitialCondition>,
        stone_lines: Vec<Line>,
    }
    impl HailCloud {
        fn new(stones: Vec<InitialCondition>) -> HailCloud {
            let stone_lines = stones
                .iter()
                .map(|x| Line::try_from(*x).unwrap())
                .collect::<Vec<_>>();
            HailCloud {
                stones,
                stone_lines,
            }
        }
        fn count_intersections_in_test_area(&self, min_test: f64, max_test: f64) -> usize {
            let mut count = 0;
            for i in 0..self.stones.len() {
                for j in i + 1..self.stones.len() {
                    let intersection = find_intersection(
                        &self.stones[i],
                        &self.stone_lines[i],
                        &self.stones[j],
                        &self.stone_lines[j],
                    );
                    if intersection.is_none() {
                        continue;
                    }
                    // println!(
                    //     "\nintersect\n\t{}\n\t{}\n\t{:?}",
                    //     self.stones[i], self.stones[j], intersection
                    // );
                    let intersection = intersection.unwrap();
                    if intersection[0] >= min_test
                        && intersection[0] <= max_test
                        && intersection[1] >= min_test
                        && intersection[1] <= max_test
                    {
                        // println!("inside");
                        count += 1;
                    }
                }
            }
            count
        }
    }
    impl FromStr for HailCloud {
        type Err = ();
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let stones = s
                .lines()
                .map(|line| line.parse::<InitialCondition>().unwrap())
                .collect::<Vec<_>>();
            Ok(HailCloud::new(stones))
        }
    }
    impl Display for HailCloud {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for (stone, line) in self.stones.iter().zip(self.stone_lines.iter()) {
                writeln!(f, "{} -> {}", stone, line)?
            }
            Ok(())
        }
    }
    pub fn run(input: &str, min_test: f64, max_test: f64) -> usize {
        let hail = input.parse::<HailCloud>().unwrap();
        // println!("{}", hail);
        hail.count_intersections_in_test_area(min_test, max_test)
    }
}

mod b {
    use std::{fmt::Display, str::FromStr, time::Instant};

    #[derive(PartialEq, Eq, PartialOrd, Ord, Default, Debug, Clone, Copy)]
    struct Position {
        vec: [i64; 3],
    }
    impl Position {
        fn new(vec: [i64; 3]) -> Position {
            Position { vec }
        }
        fn sum(&self) -> i64 {
            self.vec.iter().sum()
        }
    }

    impl Display for Position {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}, {}, {}", self.vec[0], self.vec[1], self.vec[2])
        }
    }
    impl FromStr for Position {
        type Err = ();
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let vec = s
                .split(", ")
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            assert_eq!(vec.len(), 3);
            let vec = [vec[0], vec[1], vec[2]];
            Ok(Position::new(vec))
        }
    }
    #[derive(PartialEq, Eq, PartialOrd, Ord, Default, Debug, Clone, Copy)]
    struct Velocity {
        vec: [i64; 3],
    }
    impl Velocity {
        fn new(vec: [i64; 3]) -> Velocity {
            Velocity { vec }
        }
    }
    impl Display for Velocity {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}, {}, {}", self.vec[0], self.vec[1], self.vec[2])
        }
    }
    impl FromStr for Velocity {
        type Err = ();
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let vec = s
                .split(", ")
                .map(|x| x.trim().parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            assert_eq!(vec.len(), 3);
            let vec = [vec[0], vec[1], vec[2]];
            Ok(Velocity::new(vec))
        }
    }
    #[derive(PartialEq, Eq, PartialOrd, Ord, Default, Clone, Copy, Debug)]
    struct InitialCondition {
        position: Position,
        velocity: Velocity,
    }
    impl InitialCondition {
        fn new(position: Position, velocity: Velocity) -> InitialCondition {
            InitialCondition { position, velocity }
        }
        fn position_sum(&self) -> i64 {
            self.position.sum()
        }
    }
    impl FromStr for InitialCondition {
        type Err = ();
        /// Parse things like 19, 13, 30 @ -2,  1, -2
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (pos, vel) = s.split_once(" @ ").unwrap();
            let pos = pos.parse::<Position>().unwrap();
            let vel = vel.parse::<Velocity>().unwrap();
            Ok(InitialCondition::new(pos, vel))
        }
    }
    impl Display for InitialCondition {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} @ {}", self.position, self.velocity)
        }
    }
    struct HailCloud {
        stones: Vec<InitialCondition>,
        p: Vec<Vec<f64>>,
        v: Vec<Vec<f64>>,
    }
    impl HailCloud {
        fn new(stones: Vec<InitialCondition>) -> HailCloud {
            let p = stones
                .iter()
                .map(|s| s.position.vec.iter().map(|x| *x as f64).collect::<Vec<_>>())
                .collect::<Vec<_>>();
            let v = stones
                .iter()
                .map(|s| s.velocity.vec.iter().map(|x| *x as f64).collect::<Vec<_>>())
                .collect::<Vec<_>>();
            HailCloud { stones, p, v }
        }

        /// Solve for rock that will pass through all hail positions
        /// and return sum of initial position coords
        fn run(&self) -> i64 {
            self.estimate_rock().position_sum()
        }
        fn estimate_error(&self, rp: &[f64; 3], rv: &[f64; 3], t: &[f64]) -> f64 {
            let mut error = 0.0;
            for i in 0..self.stones.len() {
                let p = &self.stones[i].position.vec;
                let v = &self.stones[i].velocity.vec;
                for n in 0..3 {
                    let rock_p = rp[n] + rv[n] * t[i];
                    let stone_p = p[n] as f64 + v[n] as f64 * t[i];
                    error += (stone_p - rock_p).powi(2);
                }
            }
            error
        }
        /// Make a guess at the rock's initial conditions by observing
        /// when in time the stones will be close together
        fn estimate_rock(&self) -> InitialCondition {
            let mut rp = [0_f64; 3];
            let mut rv = [0_f64; 3];
            let mut t = vec![0_f64; self.stones.len()];

            // use gradient descent to refine solution
            let mut rock;
            let mut last_rock = InitialCondition::default();
            let mut it = 0;
            let start = Instant::now();
            let mut debug: bool;
            loop {
                it += 1;
                if it == 500 {
                    dbg!(rp);
                    dbg!(rv);
                    dbg!(t);
                    panic!();
                }
                debug = false;
                if it % 10 == 0 {
                    debug = true;
                    let error = self.estimate_error(&rp, &rv, &t);
                    println!(
                        "it {}, t: {} error: {:.2e} last_rock: {}",
                        it,
                        start.elapsed().as_secs(),
                        error,
                        last_rock
                    );
                }
                // newton's method of finding local minima
                // update time
                let mut max_time_change: f64 = 0.0;
                for i in 0..self.stones.len() {
                    let p = &self.p[i];
                    let v = &self.v[i];
                    let mut ddt: f64 = 0.0;
                    let mut dt: f64 = 0.0;
                    for n in 0..3 {
                        ddt += 2.0 * ((rv[n] - v[n]).powi(2));
                        dt += (-2.0 * rv[n] + 2.0 * v[n])
                            * (p[n] - rp[n] - rv[n] * t[i] + t[i] * v[n]);
                    }
                    if debug {
                        max_time_change = max_time_change.max((dt / ddt).abs())
                    }
                    t[i] -= dt / ddt;
                }
                if debug {
                    print!("dt: {} ", max_time_change);
                }
                // update rp
                {
                    let mut max_pos_change: f64 = 0.0;
                    let ddp: f64 = 2.0 * self.stones.len() as f64;
                    let mut dp = [0.0_f64; 3];
                    for i in 0..self.stones.len() {
                        let p = &self.p[i];
                        let v = &self.v[i];
                        for n in 0..3 {
                            dp[n] +=
                                -2.0 * p[n] + 2.0 * rp[n] + 2.0 * rv[n] * t[i] - 2.0 * t[i] * v[n];
                        }
                    }
                    for n in 0..3 {
                        if debug {
                            max_pos_change = max_pos_change.max((dp[n] / ddp).abs());
                        }
                        rp[n] -= dp[n] / ddp;
                    }
                    if debug {
                        print!("dp: {} ", max_pos_change);
                    }
                }
                // update rv
                {
                    let mut ddv = [0.0_f64; 3];
                    let mut dv = [0.0_f64; 3];
                    let mut max_v_change: f64 = 0.0;
                    for i in 0..self.stones.len() {
                        let p = &self.p[i];
                        let v = &self.v[i];
                        for n in 0..3 {
                            ddv[n] += 2.0 * t[i].powi(2);
                            dv[n] += -2.0 * t[i] * (p[n] - rp[n] - rv[n] * t[i] + t[i] * v[n]);
                        }
                    }
                    for n in 0..3 {
                        if debug {
                            max_v_change = max_v_change.max((dv[n] / ddv[n]).abs());
                        }
                        rv[n] -= dv[n] / ddv[n];
                    }
                    if debug {
                        println!("dv: {} ", max_v_change);
                    }
                }

                if !rp.iter().all(|x| x.is_finite()) {
                    dbg!(&rp);
                    panic!("rp not finite");
                }
                let rock_position = Position::new([
                    rp[0].round() as i64,
                    rp[1].round() as i64,
                    rp[2].round() as i64,
                ]);
                if !rv.iter().all(|x| x.is_finite()) {
                    panic!("rv not finite");
                }
                let rock_velocity = Velocity::new([
                    rv[0].round() as i64,
                    rv[1].round() as i64,
                    rv[2].round() as i64,
                ]);
                rock = InitialCondition::new(rock_position, rock_velocity);
                // if rock != last_rock {
                if self.verify_rock(&rock, it == 499) {
                    println!("Solved after it: {} rock: {}", it, rock);
                    break;
                }
                // }
                last_rock = rock;
            }

            rock
        }
        /// Return true if rock intersects all stones. Assume stones at i and j are already checked
        fn verify_rock(&self, rock: &InitialCondition, debug: bool) -> bool {
            for k in 0..self.stones.len() {
                // stone_loc = stone_pos + stone_vel * t
                // rock_loc = rock_pos + rock_vel * t
                // stone_pos + stone_vel * t = rock_pos + rock_vel * t
                // stone_pos - rock_pos = t * (rock_vel - stone_vel)
                // t = (stone_pos - rock_pos) / (rock_vel - stone_vel)
                let stone = &self.stones[k];
                let bad_x = rock.velocity.vec[0] as i64 == stone.velocity.vec[0] as i64;
                let t = if bad_x {
                    (stone.position.vec[1] as i64 - rock.position.vec[1] as i64)
                        / (rock.velocity.vec[1] as i64 - stone.velocity.vec[1] as i64)
                } else {
                    (stone.position.vec[0] as i64 - rock.position.vec[0] as i64)
                        / (rock.velocity.vec[0] as i64 - stone.velocity.vec[0] as i64)
                };
                if t < 0 {
                    if debug {
                        println!("negative time stone: {}", k);
                    }
                    return false;
                }
                for n in 0..3 {
                    if rock.position.vec[n] as i64 + rock.velocity.vec[n] as i64 * t
                        != stone.position.vec[n] as i64 + stone.velocity.vec[n] as i64 * t
                    {
                        if debug {
                            println!("non-match at stone {} dim {}", k, n);
                        }
                        return false;
                    }
                }
            }
            true
        }
    }
    impl FromStr for HailCloud {
        type Err = ();
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let stones = s
                .lines()
                .map(|line| line.parse::<InitialCondition>().unwrap())
                .collect::<Vec<_>>();
            Ok(HailCloud::new(stones))
        }
    }
    impl Display for HailCloud {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for stone in &self.stones {
                writeln!(f, "{}", stone)?
            }
            Ok(())
        }
    }
    pub fn run(input: &str) -> i64 {
        let hail = input.parse::<HailCloud>().unwrap();
        hail.run()
    }
}

impl AocSolution for Day24 {
    fn part1(&self, input: &str) -> String {
        a::run(input, 200000000000000.0, 400000000000000.0).to_string()
    }

    fn part2(&self, input: &str) -> String {
        b::run(input).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3"#;

    #[test]
    fn test_part1_example() {
        assert_eq!(a::run(EXAMPLE, 7.0, 27.0), 2);
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2023, 24).expect("Failed to get input");
        assert_eq!(Day24.part1(&input), "13965");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(b::run(EXAMPLE), 47);
    }

    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2023, 24).expect("Failed to get input");
        assert_eq!(Day24.part2(&input), "578177720733043");
    }
}
