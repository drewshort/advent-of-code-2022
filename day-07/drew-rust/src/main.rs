/*
Basic rust bin with runtime error and arg parsing
*/
use std::path::Path;
use std::{ env, error::Error };

use aoc_common_lib::error::RuntimeError;
use aoc_common_lib::utility::read_lines;

// Override the alias to use `Box<error::Error>`.
type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn parse_spam(input_file_path: &str) -> Result<()> {
    let input_file = Path::new(input_file_path);
    if !input_file.exists() {
        let error_message = format!("Path {} does not appear to exist", input_file_path);
        return Err(Box::new(RuntimeError::new(error_message)));
    }
    let mut objs: Vec<Option<()>> = Vec::new();
    if let Ok(lines) = read_lines(input_file) {
        for line in lines {
            match line {
                Ok(line) => {
                    line;
                }
                Err(err) => {
                    return Err(Box::new(err));
                }
            }
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(Box::new(RuntimeError::new(String::from("Must provide input file path"))));
    }
    let input_path = &args[1];
    let results = parse_spam(input_path);

    Ok(())
}