use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
    dotenv::from_filename(".aoc_config").ok();

    let args: Vec<String> = std::env::args().collect();
    let (year, day) = match args.len() {
        3 => (args[1].clone(), args[2].clone()),
        2 => (
            std::env::var("YEAR").expect("YEAR not set in .aoc_config"),
            args[1].clone(),
        ),
        _ => {
            eprintln!("Usage: cargo scaffold [YEAR] <DAY>");
            std::process::exit(1);
        }
    };

    let day_padded = format!("{:02}", day.parse::<u8>().unwrap());

    let template = fs::read_to_string("templates/day_template.rs")?;
    let content = template.replace("{DAY}", &day_padded);

    let dir = format!("src/solutions/year_{}", year);
    fs::create_dir_all(&dir)?;

    let path = Path::new(&dir).join(format!("day{}.rs", day_padded));
    fs::write(path, content)?;

    println!("Created solution for year {}, day {}", year, day);

    Ok(())
}
