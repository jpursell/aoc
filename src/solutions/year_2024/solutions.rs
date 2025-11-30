
pub fn get_solutions() -> std::collections::HashMap<u8, Box<dyn crate::AocSolution>> {
    let mut map: std::collections::HashMap<u8, Box<dyn crate::AocSolution>> =
        std::collections::HashMap::new();
    map.insert(3, Box::new(super::day03::Day03));
    map.insert(1, Box::new(super::day01::Day01));
    map.insert(2, Box::new(super::day02::Day02));
    map
}
