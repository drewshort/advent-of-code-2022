/*
Basic rust bin with runtime error and arg parsing
*/
use std::{env, error::Error, fmt};

use aoc_common_lib::error::RuntimeError;
use aoc_common_lib::utility::read_lines;

// Override the alias to use `Box<error::Error>`.
type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(Box::new(RuntimeError {
            message: String::from("Must provide input file path"),
        }));
    }
    let input_path = &args[1];

    Ok(())
}
