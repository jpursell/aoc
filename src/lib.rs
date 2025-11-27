use anyhow::{Context, Result};
use dotenv::dotenv;
use reqwest::blocking::Client;
use reqwest::header::COOKIE;
use std::collections::HashMap;
use std::env;
use std::fs;

// 1. Define the interface for every AoC solution
pub trait AocSolution {
    /// Solves Part 1 and returns the result as a String.
    fn part1(&self, input: &str) -> String;
    /// Solves Part 2 and returns the result as a String.
    fn part2(&self, input: &str) -> String;
}

// 2. Export the year modules (where your logic lives)
// We'll create these files shortly.
pub mod solutions;

/// Downloads the puzzle input for a specific day, utilizing a local cache.
///
/// The input is cached at `~/.cache/adventOfCode/{year}/day{day}/input.txt`.
pub fn get_input_for_day(year: u16, day: u8) -> Result<String> {
    dotenv().ok(); // Load the .env file

    // --- 1. Define Cache Path ---
    let cache_dir_root = dirs::home_dir()
        .context("Could not find user's home directory")?
        .join(".cache")
        .join("adventOfCode");

    let cache_file_path = cache_dir_root
        .join(year.to_string())
        .join(format!("day{}", day))
        .join("input.txt");

    // --- 2. Check Cache ---
    if cache_file_path.exists() {
        println!("✅ Input found in cache: {}", cache_file_path.display());
        return fs::read_to_string(cache_file_path).context("Failed to read input from cache file");
    }

    // --- 3. Download Logic (if not cached) ---
    println!("⬇️ Downloading input for {year}, Day {day}...");

    // Get the session cookie from the environment
    let session_cookie = env::var("AOC_SESSION")
        .context("AOC_SESSION environment variable not set. Please set it in your .env file.")?;

    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let cookie_header = format!("session={}", session_cookie);

    let client = Client::new();
    let input_content = client
        .get(&url)
        .header(COOKIE, cookie_header)
        .send()?
        // AoC returns a 404/500 if the day is not available or the session is bad
        .error_for_status()?
        .text()
        .context("Failed to get text content from AoC response")?;

    // --- 4. Cache and Return ---

    // Create parent directories for the cache file
    if let Some(parent) = cache_file_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create cache directory: {}", parent.display()))?;
    }

    // Write the downloaded content to the cache file
    fs::write(&cache_file_path, &input_content).with_context(|| {
        format!(
            "Failed to write input to cache file: {}",
            cache_file_path.display()
        )
    })?;

    println!(
        "💾 Input cached successfully at {}",
        cache_file_path.display()
    );

    Ok(input_content)
}

// --- Helper Functions ---

/// Gets the latest solved day for the current year.
pub fn get_latest_day(solutions: &HashMap<u8, Box<dyn AocSolution>>) -> u8 {
    for day in (1..=31).rev() {
        if solutions.contains_key(&day) {
            return day;
        }
    }
    panic!("Did not find any present solutions!");
}

pub fn run_single_solution(year: u16, day: u8, solutions: &HashMap<u8, Box<dyn AocSolution>>) {
    println!("🚀 Running {year} Day {day}...");
    if let Some(solution) = solutions.get(&day) {
        let input = get_input_for_day(year, day).unwrap();
        println!("  Part 1: {}", solution.part1(&input));
        println!("  Part 2: {}", solution.part2(&input));
    } else {
        eprintln!("  Solution for Day {day} not found.");
    }
}

pub fn run_all_solutions(year: u16, solutions: &HashMap<u8, Box<dyn AocSolution>>) {
    println!("🌟 Running ALL solutions for {year}...");
    let mut days: Vec<_> = solutions.keys().copied().collect();
    days.sort();

    for day in days {
        run_single_solution(year, day, solutions);
    }
}
