use std::{collections::BTreeMap, str::FromStr};

use crate::AocSolution;

#[derive(Clone, Copy, Debug)]
struct Point {
    x: u64,
    y: u64,
}
impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(",").unwrap();
        let x = x.parse().unwrap();
        let y = y.parse().unwrap();
        Ok(Point { x, y })
    }
}
#[derive(Clone, Copy, Debug)]
struct VirtSeg {
    x: u64,
    y0: u64,
    y1: u64,
}
impl Point {
    fn area(&self, other: &Point) -> u64 {
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }
    fn virt_seg(&self, other: &Point) -> VirtSeg {
        assert_eq!(self.x, other.x);
        let y0 = self.y.min(other.y);
        let y1 = self.y.max(other.y);
        VirtSeg { x: self.x, y0, y1 }
    }
}
impl VirtSeg {
    fn intersects_y(&self, y: u64) -> bool {
        y >= self.y0 && y <= self.y1
    }
    fn intersects(&self, other: &VirtSeg) -> bool {
        self.x == other.x
            && (self.intersects_y(other.y0)
                || self.intersects_y(other.y1)
                || other.intersects_y(self.y0)
                || other.intersects_y(self.y1))
    }
}
fn parse(input: &str) -> Vec<Point> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}
fn find_largest_rectangle(points: &[Point], segs: Option<&SegMap>) -> u64 {
    let mut largest = 0;
    let seg_keys: Option<Vec<u64>> = segs.map(|s| {
        let mut seg_keys: Vec<u64> = s.keys().cloned().collect();
        seg_keys.sort();
        seg_keys
    });

    for point_a in points.iter().enumerate() {
        for point_b in points.iter().skip(point_a.0 + 1) {
            if let Some(segs) = segs {
                let corners = &[*point_a.1, *point_b];
                if !test_inside(corners, segs, seg_keys.as_ref().unwrap()) {
                    continue;
                }
            }
            let area = point_a.1.area(point_b);
            largest = largest.max(area)
        }
    }
    largest
}
type SegMap = BTreeMap<u64, Vec<VirtSeg>>;
fn find_virt_segments(points: &[Point]) -> SegMap {
    let mut segs = BTreeMap::new();
    let mut process_pair = |pair: &[Point]| {
        if pair[0].x != pair[1].x {
            return;
        }
        let seg = pair[0].virt_seg(&pair[1]);
        segs.entry(pair[0].x)
            .and_modify(|vec: &mut Vec<_>| vec.push(seg))
            .or_insert(vec![seg]);
    };
    for pair in points.windows(2) {
        process_pair(pair);
    }
    process_pair(&[points[0], points[points.len() - 1]]);
    //see if any segments are overlapping
    for vec in segs.values() {
        for seg0 in vec.iter().enumerate() {
            for seg1 in vec.iter().skip(seg0.0 + 1) {
                if seg0.1.intersects(seg1) {
                    panic!("Segments intersect!");
                }
            }
        }
    }
    segs
}
fn test_inside(corners: &[Point], segs: &SegMap, seg_keys: &[u64]) -> bool {
    // TODO this function is too simple
    // It needs to deal with literal corner cases
    let box_x_min = corners.iter().map(|p| p.x).min().unwrap();
    let box_y_min = corners.iter().map(|p| p.y).min().unwrap();
    let box_x_max = corners.iter().map(|p| p.x).max().unwrap();
    let box_y_max = corners.iter().map(|p| p.y).max().unwrap();
    for y in box_y_min..=box_y_max {
        let mut inside = false;
        for x in seg_keys {
            if segs[x].iter().any(|s| s.intersects_y(y)) {
                inside = !inside;
            }
            if !inside && *x >= box_x_min && *x < box_x_max {
                eprintln!("reject: corners: {:?} x: {} y: {}", corners, x, y);
                return false;
            }
            if *x > box_x_max {
                break;
            }
        }
    }
    true
}
pub struct Day09;

impl AocSolution for Day09 {
    fn part1(&self, input: &str) -> String {
        let points = parse(input);
        find_largest_rectangle(&points, None).to_string()
    }

    fn part2(&self, input: &str) -> String {
        let points = parse(input);
        let segs = find_virt_segments(&points);
        find_largest_rectangle(&points, Some(&segs)).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_part1_example() {
        assert_eq!(Day09.part1(EXAMPLE), "50");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2025, 9).expect("Failed to get input");
        assert_eq!(Day09.part1(&input), "4725826296");
    }

    #[test]
    fn test_limits() {
        let input = crate::get_input_for_day(2025, 9).expect("Failed to get input");
        let points = parse(&input);
        let minx = points.iter().map(|p| p.x).min().unwrap();
        let miny = points.iter().map(|p| p.y).min().unwrap();
        let maxx = points.iter().map(|p| p.x).max().unwrap();
        let maxy = points.iter().map(|p| p.y).max().unwrap();
        eprintln!("{},{} -> {},{}", minx, miny, maxx, maxy);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(Day09.part2(EXAMPLE), "24");
    }

    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2025, 9).expect("Failed to get input");
        assert_eq!(Day09.part2(&input), "REPLACE_WITH_PART2_FULL_RESULT");
    }
}
