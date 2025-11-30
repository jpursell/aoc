pub mod year_2024;

pub fn available_years() -> std::collections::HashMap<u16, fn() -> std::collections::HashMap<u8, Box<dyn crate::AocSolution>>> {
    let mut map = std::collections::HashMap::new();
    map.insert(2024, year_2024::get_solutions as fn() -> std::collections::HashMap<u8, Box<dyn crate::AocSolution>>);
    map
}
