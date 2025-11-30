use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    let solutions_dir = Path::new("src/solutions");
    let mut year_mods = Vec::new();
    let mut year_map = HashMap::new();

    for entry in fs::read_dir(solutions_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            let year_str = path.file_name().unwrap().to_str().unwrap();
            if year_str.starts_with("year_") {
                if let Ok(year) = year_str[5..].parse::<u16>() {
                    year_mods.push(format!("pub mod {};", year_str));
                    let mut day_mods = Vec::new();
                    let mut day_map = Vec::new();

                    for day_entry in fs::read_dir(&path).unwrap() {
                        let day_entry = day_entry.unwrap();
                        let day_path = day_entry.path();
                        if day_path.is_file() {
                            let day_file_name = day_path.file_name().unwrap().to_str().unwrap();
                            if day_file_name.starts_with("day") && day_file_name.ends_with(".rs") {
                                let day_mod_name = &day_file_name[..day_file_name.len() - 3];
                                let day_num_str = &day_mod_name[3..];
                                if let Ok(day_num) = day_num_str.parse::<u8>() {
                                    let day_struct = format!("Day{:02}", day_num);
                                    day_mods.push(format!("pub mod {};", day_mod_name));
                                    day_map.push(format!(
                                        "    map.insert({}, Box::new(super::{}::{}));",
                                        day_num, day_mod_name, day_struct
                                    ));
                                }
                            }
                        }
                    }

                    let mod_path = path.join("mod.rs");
                    let mut mod_file = fs::File::create(&mod_path).unwrap();
                    writeln!(mod_file, "{}", day_mods.join("\n")).unwrap();

                    let solutions_fn = format!(
                        "
pub fn get_solutions() -> std::collections::HashMap<u8, Box<dyn crate::AocSolution>> {{
    let mut map: std::collections::HashMap<u8, Box<dyn crate::AocSolution>> =
        std::collections::HashMap::new();
{}
    map
}}",
                        day_map.join("\n")
                    );

                    let year_solutions_path = path.join("solutions.rs");
                    let mut year_solutions_file = fs::File::create(&year_solutions_path).unwrap();
                    writeln!(year_solutions_file, "{}", solutions_fn).unwrap();
                    writeln!(mod_file, "mod solutions;").unwrap();
                    writeln!(mod_file, "pub use solutions::get_solutions;").unwrap();

                    year_map.insert(year, format!("{}::get_solutions", year_str));
                }
            }
        }
    }

    let solutions_mod_path = solutions_dir.join("mod.rs");
    let mut solutions_mod_file = fs::File::create(&solutions_mod_path).unwrap();
    writeln!(solutions_mod_file, "{}", year_mods.join("\n")).unwrap();

    let available_years_fn = format!(
        "\npub fn available_years() -> std::collections::HashMap<u16, fn() -> std::collections::HashMap<u8, Box<dyn crate::AocSolution>>> {{
    let mut map = std::collections::HashMap::new();
{}
    map
}}",
        year_map
            .into_iter()
            .map(|(y, f)| format!("    map.insert({}, {} as fn() -> std::collections::HashMap<u8, Box<dyn crate::AocSolution>>);", y, f))
            .collect::<Vec<_>>()
            .join("\n")
    );

    writeln!(solutions_mod_file, "{}", available_years_fn).unwrap();

    println!("cargo:rerun-if-changed=src/solutions");
}
