use crate::AocSolution;

pub struct Day02;

impl Day02 {
    fn extract(str: &str) -> Vec<Vec<usize>> {
        let count = str.lines().count();
        let mut vecs = Vec::with_capacity(count);
        for line in str.lines() {
            let count = line.split_whitespace().count();
            let mut vec = Vec::with_capacity(count);
            for val in line.split_whitespace() {
                let val: usize = val.parse().unwrap();
                vec.push(val);
            }
            vecs.push(vec);
        }
        vecs
    }
    fn process_line(a: &[usize], increasing: bool) -> bool {
        for chunk in a.windows(2) {
            let d = if increasing {
                chunk[1] as i64 - chunk[0] as i64
            } else {
                chunk[0] as i64 - chunk[1] as i64
            };
            let ok = matches!(d, 1..=3);
            if !ok {
                return false;
            }
        }
        true
    }
    fn process_1(vecs: Vec<Vec<usize>>) -> usize {
        let mut out = 0;
        for a in vecs.iter() {
            let ok = Day02::process_line(a, true) || Day02::process_line(a, false);
            if ok {
                out += 1;
            }
        }
        out
    }
    fn process_line_at(a: &[usize], loc: usize) -> bool {
        let a: Vec<usize> = a
            .iter()
            .enumerate()
            .filter(|(i, _x)| i != &loc)
            .map(|(_i, x)| *x)
            .collect();
        Day02::process_line(&a, true) || Day02::process_line(&a, false)
    }
    fn process_2(vecs: Vec<Vec<usize>>) -> usize {
        let mut out = 0;
        for a in vecs.iter() {
            for i in 0..a.len() {
                let ok = Day02::process_line_at(a, i);
                if ok {
                    out += 1;
                    break;
                }
            }
        }
        out
    }
}

impl AocSolution for Day02 {
    fn part1(&self, input: &str) -> String {
        let test = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        // fn main() {
        // let out = include_str!(test);
        let out = Day02::extract(test);
        let out = Day02::process_1(out);
        assert_eq!(out, 2);

        // let out = include_str!("02.txt");
        let out = Day02::extract(input);
        let out = Day02::process_1(out);
        assert_eq!(out, 356);
        out.to_string()
        // }
    }
    fn part2(&self, input: &str) -> String {
        // fn main() {
        // let out = include_str!("02.txt");
        let out = Day02::extract(input);
        let out = Day02::process_2(out);
        // println!("{out:?}");
        // }
        let out = out.to_string();
        let expect = "413";
        assert_eq!(out, expect);
        out
    }
}
