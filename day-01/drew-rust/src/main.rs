use std::{env, error::Error, path::Path};

use aoc_common_lib::error::RuntimeError;
use aoc_common_lib::utility::read_lines;

// Override the alias to use `Box<error::Error>`.
type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug, Clone, Copy)]
struct Elf {
    id: u8,
    calories: u32,
}

fn parse_elves(input_file_path: &str) -> Result<Vec<Elf>> {
    let input_file = Path::new(input_file_path);
    if !input_file.exists() {
        let error_message = format!("Path {} does not appear to exist", input_file_path);
        return Err(Box::new(RuntimeError::new(error_message)));
    }

    let mut elves: Vec<Elf> = Vec::new();
    if let Ok(lines) = read_lines(input_file) {
        let mut current_elf_id: u8 = 1;
        let mut current_elf_calories: u32 = 0;

        for line in lines {
            match line {
                Ok(line) => {
                    let trimmed_line = line.trim();
                    // Record the totals for an elf when encountering a newline
                    if trimmed_line.is_empty() {
                        elves.push(Elf {
                            id: current_elf_id,
                            calories: current_elf_calories,
                        });
                        current_elf_id += 1;
                        current_elf_calories = 0;
                        continue;
                    }
                    let line_calories: u32 = trimmed_line.parse()?;
                    current_elf_calories += line_calories;
                }
                Err(err) => {
                    return Err(Box::new(err));
                }
            }
        }

        // Capture the last elf in the event that the file does not end in a newline
        if current_elf_calories != 0 {
            elves.push(Elf {
                id: current_elf_id,
                calories: current_elf_calories,
            });
        }
    }

    Ok(elves)
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(Box::new(RuntimeError::new(String::from(
            "Must provide input file path",
        ))));
    }
    let input_path = &args[1];
    let mut elves = parse_elves(input_path)?;
    // Sort in reversed direction b > a to get descending values
    elves.sort_by(|a, b| b.calories.cmp(&a.calories));

    let top_3_elves = &elves[0..3];
    println!(
        "{} Calories carried by elf {}",
        top_3_elves[0].calories, top_3_elves[0].id
    );
    println!(
        "{} Calories carried by the top 3 elves",
        top_3_elves.iter().map(|&elf| elf.calories).sum::<u32>()
    );

    Ok(())
}
