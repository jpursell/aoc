use crate::AocSolution;

pub struct Day15;

fn hash_str(s: &str) -> usize {
    let mut val = 0;
    for &b in s.as_bytes() {
        val += b as usize;
        val *= 17;
        val %= 256;
    }
    val
}

#[derive(Clone, Debug)]
struct Lens<'a> {
    label: &'a str,
    focal_length: usize,
}

impl AocSolution for Day15 {
    fn part1(&self, input: &str) -> String {
        let total: usize = input.trim().split(',').map(|s| hash_str(s.trim())).sum();
        total.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut boxes: Vec<Vec<Lens>> = vec![Vec::new(); 256];

        for step in input
            .trim()
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
        {
            if let Some(label) = step.strip_suffix('-') {
                let box_idx = hash_str(label);
                if let Some(pos) = boxes[box_idx].iter().position(|l| l.label == label) {
                    boxes[box_idx].remove(pos);
                }
            } else if let Some((label, focal_str)) = step.split_once('=') {
                let focal_length = focal_str.parse::<usize>().unwrap();
                let box_idx = hash_str(label);
                if let Some(pos) = boxes[box_idx].iter().position(|l| l.label == label) {
                    boxes[box_idx][pos].focal_length = focal_length;
                } else {
                    boxes[box_idx].push(Lens {
                        label,
                        focal_length,
                    });
                }
            }
        }

        let total: usize = boxes
            .iter()
            .enumerate()
            .map(|(box_idx, b)| {
                b.iter()
                    .enumerate()
                    .map(|(slot_idx, lens)| (box_idx + 1) * (slot_idx + 1) * lens.focal_length)
                    .sum::<usize>()
            })
            .sum();

        total.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_part1_example() {
        assert_eq!(Day15.part1(EXAMPLE), "1320");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2023, 15).expect("Failed to get input");
        assert_eq!(Day15.part1(&input), "510388");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(Day15.part2(EXAMPLE), "145");
    }

    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2023, 15).expect("Failed to get input");
        assert_eq!(Day15.part2(&input), "291774");
    }
}
