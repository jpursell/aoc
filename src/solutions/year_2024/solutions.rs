
pub fn get_solutions() -> std::collections::HashMap<u8, Box<dyn crate::AocSolution>> {
    let mut map: std::collections::HashMap<u8, Box<dyn crate::AocSolution>> =
        std::collections::HashMap::new();
    map.insert(1, Box::new(super::day01::Day01));
    map.insert(2, Box::new(super::day02::Day02));
    map.insert(3, Box::new(super::day03::Day03));
    map.insert(4, Box::new(super::day04::Day04));
    map.insert(5, Box::new(super::day05::Day05));
    map.insert(6, Box::new(super::day06::Day06));
    map.insert(7, Box::new(super::day07::Day07));
    map.insert(8, Box::new(super::day08::Day08));
    map.insert(9, Box::new(super::day09::Day09));
    map.insert(10, Box::new(super::day10::Day10));
    map.insert(11, Box::new(super::day11::Day11));
    map.insert(12, Box::new(super::day12::Day12));
    map.insert(13, Box::new(super::day13::Day13));
    map.insert(14, Box::new(super::day14::Day14));
    map.insert(15, Box::new(super::day15::Day15));
    map.insert(16, Box::new(super::day16::Day16));
    map.insert(17, Box::new(super::day17::Day17));
    map.insert(18, Box::new(super::day18::Day18));
    map.insert(19, Box::new(super::day19::Day19));
    map.insert(20, Box::new(super::day20::Day20));
    map.insert(21, Box::new(super::day21::Day21));
    map.insert(22, Box::new(super::day22::Day22));
    map.insert(23, Box::new(super::day23::Day23));
    map.insert(24, Box::new(super::day24::Day24));
    map.insert(25, Box::new(super::day25::Day25));
    map
}
