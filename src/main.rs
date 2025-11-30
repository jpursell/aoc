use clap::Parser;
use std::process;
// Import your library functions
use aoc::{get_latest_day, run_all_solutions, run_single_solution, solutions};
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
    dotenv::from_filename(".aoc_config").ok();
    let args = Args::parse();
    let current_year = std::env::var("YEAR")
        .expect("YEAR not set in .aoc_config")
        .parse::<u16>()
        .unwrap();
    let target_year = args.year.unwrap_or(current_year);

    // Map all available year solutions
    let available_years = solutions::available_years();

    match available_years.get(&target_year) {
        Some(solutions_fn) => {
            let solutions = solutions_fn();
            let run_all = args.all || (args.year.is_some() && args.day.is_none());
            if run_all {
                // Logic for `cargo run --year 2024 --all`
                run_all_solutions(target_year, &solutions);
            } else {
                // Logic for `cargo run` or `cargo run --year 2024 -d 1`
                let target_day = args.day.unwrap_or(get_latest_day(&solutions));
                run_single_solution(target_year, target_day, &solutions);
            }
        }
        None => {
            eprintln!("❌ No solutions found for year {target_year}.");
            process::exit(1);
        }
    }
}
