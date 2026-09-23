use crate::AocSolution;

pub struct Day24;

#[derive(Clone, Copy, Debug)]
struct Hailstone {
    px: f64,
    py: f64,
    pz: f64,
    vx: f64,
    vy: f64,
    vz: f64,
}

fn parse_hailstone(line: &str) -> Hailstone {
    let (p, v) = line.split_once('@').unwrap();
    let pos: Vec<f64> = p.split(',').map(|s| s.trim().parse().unwrap()).collect();
    let vel: Vec<f64> = v.split(',').map(|s| s.trim().parse().unwrap()).collect();

    Hailstone {
        px: pos[0],
        py: pos[1],
        pz: pos[2],
        vx: vel[0],
        vy: vel[1],
        vz: vel[2],
    }
}

fn count_intersections_xy(hailstones: &[Hailstone], min_c: f64, max_c: f64) -> usize {
    let mut count = 0;

    for (i, h1) in hailstones.iter().enumerate() {
        for h2 in &hailstones[(i + 1)..] {
            let det = h2.vx * h1.vy - h1.vx * h2.vy;
            if det == 0.0 {
                continue;
            }

            let dt1 = (h2.py - h1.py) * h2.vx - (h2.px - h1.px) * h2.vy;
            let dt2 = h1.vx * (h2.py - h1.py) - h1.vy * (h2.px - h1.px);

            let t1 = dt1 / det;
            let t2 = dt2 / det;

            if t1 >= 0.0 && t2 >= 0.0 {
                let x = h1.px + h1.vx * t1;
                let y = h1.py + h1.vy * t1;

                if x >= min_c && x <= max_c && y >= min_c && y <= max_c {
                    count += 1;
                }
            }
        }
    }

    count
}

#[allow(clippy::needless_range_loop)]
fn solve_part2(h: &[Hailstone]) -> i64 {
    // Solve 6x6 linear system for [px, py, pz, vx, vy, vz]
    let mut a = vec![vec![0.0f64; 7]; 6];

    let pairs = [(0, 1), (0, 2)];
    for (row_idx, &(i, j)) in pairs.iter().enumerate() {
        let h1 = &h[i];
        let h2 = &h[j];

        let dpx = h1.px - h2.px;
        let dpy = h1.py - h2.py;
        let dpz = h1.pz - h2.pz;

        let dvx = h1.vx - h2.vx;
        let dvy = h1.vy - h2.vy;
        let dvz = h1.vz - h2.vz;

        let r = row_idx * 3;

        // Equation x
        a[r][0] = 0.0;
        a[r][1] = dvz;
        a[r][2] = -dvy;
        a[r][3] = 0.0;
        a[r][4] = -dpz;
        a[r][5] = dpy;
        a[r][6] = (h1.py * h1.vz - h1.pz * h1.vy) - (h2.py * h2.vz - h2.pz * h2.vy);

        // Equation y
        a[r + 1][0] = -dvz;
        a[r + 1][1] = 0.0;
        a[r + 1][2] = dvx;
        a[r + 1][3] = dpz;
        a[r + 1][4] = 0.0;
        a[r + 1][5] = -dpx;
        a[r + 1][6] = (h1.pz * h1.vx - h1.px * h1.vz) - (h2.pz * h2.vx - h2.px * h2.vz);

        // Equation z
        a[r + 2][0] = dvy;
        a[r + 2][1] = -dvx;
        a[r + 2][2] = 0.0;
        a[r + 2][3] = -dpy;
        a[r + 2][4] = dpx;
        a[r + 2][5] = 0.0;
        a[r + 2][6] = (h1.px * h1.vy - h1.py * h1.vx) - (h2.px * h2.vy - h2.py * h2.vx);
    }

    // Gaussian elimination
    for i in 0..6 {
        let mut pivot = i;
        for r in (i + 1)..6 {
            if a[r][i].abs() > a[pivot][i].abs() {
                pivot = r;
            }
        }
        a.swap(i, pivot);

        let factor = a[i][i];
        for col in i..=6 {
            a[i][col] /= factor;
        }

        for r in 0..6 {
            if r != i {
                let f = a[r][i];
                for col in i..=6 {
                    a[r][col] -= f * a[i][col];
                }
            }
        }
    }

    let vx = a[3][6].round() as i64;
    let vy = a[4][6].round() as i64;
    let vz = a[5][6].round() as i64;

    let h0 = &h[0];
    let h1 = &h[1];

    let dx0 = h0.vx as i64 - vx;
    let dy0 = h0.vy as i64 - vy;
    let dx1 = h1.vx as i64 - vx;
    let dy1 = h1.vy as i64 - vy;

    let det = dx0 * (-dy1) - dy0 * (-dx1);
    let p1x_p0x = h1.px as i64 - h0.px as i64;
    let p1y_p0y = h1.py as i64 - h0.py as i64;
    let t0 = (p1x_p0x * (-dy1) - p1y_p0y * (-dx1)) / det;

    let px = h0.px as i64 + t0 * dx0;
    let py = h0.py as i64 + t0 * dy0;
    let pz = h0.pz as i64 + t0 * (h0.vz as i64 - vz);

    px + py + pz
}

impl AocSolution for Day24 {
    fn part1(&self, input: &str) -> String {
        let hailstones: Vec<Hailstone> = input
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(parse_hailstone)
            .collect();

        let (min_c, max_c) = if hailstones.len() < 10 {
            (7.0, 27.0)
        } else {
            (200000000000000.0, 400000000000000.0)
        };

        count_intersections_xy(&hailstones, min_c, max_c).to_string()
    }

    fn part2(&self, input: &str) -> String {
        let hailstones: Vec<Hailstone> = input
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(parse_hailstone)
            .collect();

        solve_part2(&hailstones).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

    #[test]
    fn test_part1_example() {
        assert_eq!(Day24.part1(EXAMPLE), "2");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2023, 24).expect("Failed to get input");
        assert_eq!(Day24.part1(&input), "13965");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(Day24.part2(EXAMPLE), "47");
    }

    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2023, 24).expect("Failed to get input");
        assert_eq!(Day24.part2(&input), "578177720733043");
    }
}
