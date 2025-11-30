pub mod year_2024;
pub mod year_2025;

type DaySolutionsMap = std::collections::HashMap<u8, Box<dyn crate::AocSolution>>;
type YearSolutionsFn = fn() -> DaySolutionsMap;
pub type YearSolutionsMap = std::collections::HashMap<u16, YearSolutionsFn>;

pub fn available_years() -> YearSolutionsMap {
    let mut map = std::collections::HashMap::new();
    map.insert(2024, year_2024::get_solutions as YearSolutionsFn);
    map.insert(2025, year_2025::get_solutions as YearSolutionsFn);
    map
}
