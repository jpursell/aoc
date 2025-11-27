use crate::AocSolution;

// Day 1: Separate struct for each day
pub struct Day01;

impl AocSolution for Day01 {
    fn part1(&self, input: &str) -> String {
        // Your existing solution logic for 2024 Day 1 Part 1
        input.lines().count().to_string() 
    }

    fn part2(&self, input: &str) -> String {
        // Your existing solution logic for 2024 Day 1 Part 2
        "Still figuring this out!".to_string()
    }
}

// Map of all solutions for a given year
pub fn get_solutions() -> std::collections::HashMap<u8, Box<dyn AocSolution>> {
    let mut map: std::collections::HashMap<u8, Box<dyn AocSolution>> = std::collections::HashMap::new();
    // Register every day here
    map.insert(1, Box::new(Day01));
    // map.insert(2, Box::new(Day02));
    map
}
