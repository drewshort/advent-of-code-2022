extern crate pest;
#[macro_use]
extern crate pest_derive;

use crate::parser::parse_cargo_bay_and_move_commands;
use aoc_common_lib::error::RuntimeError;
use std::{env, error::Error};

mod model;
mod parser;

// Override the alias to use `Box<error::Error>`.
type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(Box::new(RuntimeError::new(String::from(
            "Must provide input file path",
        ))));
    }
    let input_path = &args[1];
    let move_stacks_together = args
        .get(2)
        .unwrap_or(&String::from("false"))
        .parse::<bool>()?;

    let results = parse_cargo_bay_and_move_commands(input_path)?;
    let mut cargo_bay = results.0;
    let move_commands = results.1;

    println!("Before:\n{}", &cargo_bay);

    for move_command in move_commands.iter() {
        cargo_bay.apply(move_command, move_stacks_together)?;
    }

    println!("Applied {} move commands...\n", move_commands.len());

    println!("After:\n{}", &cargo_bay);

    println!(
        "Top Crates: {}",
        cargo_bay
            .top()
            .iter()
            .map(|cargo_crate| format!("{}", cargo_crate))
            .collect::<Vec<String>>()
            .join(" ")
    );
    println!(
        "            {}",
        cargo_bay
            .top()
            .iter()
            .map(|cargo_crate| format!("{}", cargo_crate.id()))
            .collect::<Vec<String>>()
            .join("")
    );

    Ok(())
}
