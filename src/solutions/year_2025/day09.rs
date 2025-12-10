use std::str::FromStr;

#[derive(Debug)]
enum Limit {
    Xmax(u64),
    Xmin(u64),
    Ymax(u64),
    Ymin(u64),
}

fn compute_intersection(
    prev_point: &Point,
    current_point: &Point,
    clip_edge: &Limit,
) -> Option<Point> {
    let xsame = prev_point.x == current_point.x;
    let ysame = prev_point.y == current_point.y;
    assert!(xsame || ysame);
    assert!(!(xsame && ysame));
    match clip_edge {
        Limit::Xmin(xmin) => {
            if xsame {
                None
            } else {
                assert!(ysame);
                Some(Point {
                    x: *xmin,
                    y: current_point.y,
                })
            }
        }
        Limit::Ymin(ymin) => {
            if ysame {
                None
            } else {
                assert!(xsame);
                Some(Point {
                    x: current_point.x,
                    y: *ymin,
                })
            }
        }
        Limit::Xmax(xmax) => {
            if xsame {
                None
            } else {
                assert!(ysame);
                Some(Point {
                    x: *xmax,
                    y: current_point.y,
                })
            }
        }
        Limit::Ymax(ymax) => {
            if ysame {
                None
            } else {
                assert!(xsame);
                Some(Point {
                    x: current_point.x,
                    y: *ymax,
                })
            }
        }
    }
}

fn push_if_different(points: &mut Vec<Point>, new: Point) {
    if points.is_empty() {
        points.push(new);
        return;
    }
    if *points.last().unwrap() == new {
        return;
    }
    points.push(new);
}
fn clip_polygon(subject_polygon: &[Point], corners: &[Point]) -> Vec<Point> {
    let mut output_list: Vec<Point> = subject_polygon.to_vec();
    let limits = [
        Limit::Xmin(corners.iter().map(|p| p.x).min().unwrap()),
        Limit::Ymin(corners.iter().map(|p| p.y).min().unwrap()),
        Limit::Xmax(corners.iter().map(|p| p.x).max().unwrap()),
        Limit::Ymax(corners.iter().map(|p| p.y).max().unwrap()),
    ];

    for clip_edge in &limits {
        let input_list = output_list.clone();
        output_list.clear();

        for i in 0..input_list.len() {
            let current_point = &input_list[i];
            let prev_index = if i == 0 { input_list.len() - 1 } else { i - 1 };
            let prev_point = &input_list[prev_index];

            let intersecting_point = compute_intersection(prev_point, current_point, clip_edge);

            if current_point.inside(clip_edge) {
                if !prev_point.inside(clip_edge) {
                    push_if_different(&mut output_list, intersecting_point.unwrap());
                }
                push_if_different(&mut output_list, *current_point);
            } else if prev_point.inside(clip_edge) {
                push_if_different(&mut output_list, intersecting_point.unwrap());
            }
        }
        while output_list.len() > 1 && output_list.first().unwrap() == output_list.last().unwrap() {
            output_list.pop();
        }
    }
    // remove points in straight lines
    loop {
        if output_list.len() <= 2 {
            break;
        }
        let mut found = None;
        for i in 0..output_list.len() {
            let previous = if i == 0 {
                output_list.last().unwrap()
            } else {
                &output_list[i - 1]
            };
            let next = if i == output_list.len() - 1 {
                output_list.first().unwrap()
            } else {
                &output_list[i + 1]
            };
            let current = &output_list[i];
            if previous.x == current.x && current.x == next.x {
                found = Some(i);
                break;
            }
            if previous.y == current.y && current.y == next.y {
                found = Some(i);
                break;
            }
        }
        if let Some(i) = found {
            output_list.remove(i);
        } else {
            break;
        }
    }
    output_list
}

use crate::AocSolution;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl Point {
    fn inside(&self, limit: &Limit) -> bool {
        match limit {
            Limit::Xmin(xmin) => self.x >= *xmin,
            Limit::Ymin(ymin) => self.y >= *ymin,
            Limit::Xmax(xmax) => self.x <= *xmax,
            Limit::Ymax(ymax) => self.y <= *ymax,
        }
    }
    fn area(&self, other: &Point) -> u64 {
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }
}
fn parse(input: &str) -> Vec<Point> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}
enum Mode {
    AnyRectangle,
    InsideRectangle,
}
fn find_largest_rectangle(points: &[Point], mode: Mode) -> u64 {
    let mut largest = 0;
    for point_a in points.iter().enumerate() {
        for point_b in points.iter().skip(point_a.0 + 1) {
            match mode {
                Mode::AnyRectangle => {
                    let area = point_a.1.area(point_b);
                    largest = largest.max(area)
                }
                Mode::InsideRectangle => {
                    let corners = &[*point_a.1, *point_b];
                    let clipped_points = clip_polygon(points, corners);
                    if clipped_points.len() != 4 {
                        continue;
                    }

                    let clipped_area = clipped_points[0].area(&clipped_points[2]);
                    let area = point_a.1.area(point_b);
                    if clipped_area != area {
                        continue;
                    }
                    largest = largest.max(area)
                }
            }
        }
    }
    largest
}

pub struct Day09;

impl AocSolution for Day09 {
    fn part1(&self, input: &str) -> String {
        let points = parse(input);
        find_largest_rectangle(&points, Mode::AnyRectangle).to_string()
    }

    fn part2(&self, input: &str) -> String {
        let points = parse(input);
        find_largest_rectangle(&points, Mode::InsideRectangle).to_string()
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
        assert_eq!(Day09.part2(&input), "1637556834");
    }
}
