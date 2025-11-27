use clap::Parser;
use std::collections::HashMap;
use std::process;
// Import your library functions
use aoc::{
    AocSolution, get_latest_day, run_all_solutions, run_single_solution, solutions::year_2024,
};
// Assuming your crate name is 'my-aoc-project'

/// Advent of Code Runner
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Specify the year to run (e.g., 2024). Default is the latest year.
    #[arg(short, long)]
    year: Option<u16>,

    /// Run all available solutions for the specified year or all years.
    #[arg(long)]
    all: bool,

    /// Specify the day to run (1-25). Only used if --all is not set.
    #[arg(short, long)]
    day: Option<u8>,
}

// --- Main Dispatcher ---

fn main() {
    let args = Args::parse();
    let current_year = 2024; // Update this each year!
    let target_year = args.year.unwrap_or(current_year);

    // Map all available year solutions
    let mut available_years: HashMap<u16, HashMap<u8, Box<dyn AocSolution>>> = HashMap::new();
    // available_years.insert(2023, year_2023::get_solutions());
    available_years.insert(2024, year_2024::get_solutions());

    match available_years.get(&target_year) {
        Some(solutions) => {
            if args.all {
                // Logic for `cargo run --year 2024 --all`
                run_all_solutions(target_year, solutions);
            } else {
                // Logic for `cargo run` or `cargo run --year 2024 -d 1`
                let target_day = args.day.unwrap_or(get_latest_day(solutions));
                run_single_solution(target_year, target_day, solutions);
            }
        }
        None => {
            eprintln!("❌ No solutions found for year {target_year}.");
            process::exit(1);
        }
    }
}
