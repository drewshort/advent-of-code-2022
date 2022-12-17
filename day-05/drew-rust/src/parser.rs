use pest::Parser;

use std::error::Error;

use aoc_common_lib::{error::RuntimeError, utility::read_lines};

use crate::model::{CargoBay, CargoCrate, MoveCommand, MoveCommands};

// Override the alias to use `Box<error::Error>`.
type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Parser)]
#[grammar = "grammar.pest"] // relative to src
pub struct CargoManifestParser;

enum ManifestLine {
    CrateRow(Vec<Option<CargoCrate>>),
    StackCount(usize),
    MoveCommand(MoveCommand),
}

fn parse_crate_row(crate_row_pairs: pest::iterators::Pairs<Rule>) -> Result<ManifestLine> {
    let mut crate_row = Vec::new();

    for pair in crate_row_pairs {
        match pair.as_rule() {
            Rule::cargo_crate => crate_row.push(CargoCrate::parse(pair.as_str())),
            Rule::cargo_crate_row
            | Rule::cargo_crate_stack
            | Rule::cargo_crate_stack_row
            | Rule::move_command
            | Rule::manifest_line => unreachable!(),
        }
    }

    Ok(ManifestLine::CrateRow(crate_row))
}

fn parse_stack_row(stack_row_pairs: pest::iterators::Pairs<Rule>) -> Result<ManifestLine> {
    Ok(ManifestLine::StackCount(stack_row_pairs.count()))
}

fn parse_move_command(
    mut move_command_pairs: pest::iterators::Pairs<Rule>,
) -> Result<ManifestLine> {
    let size;
    let origin;
    let destination;

    match move_command_pairs.next() {
        Some(pair) => {
            size = pair.as_str().parse::<usize>()?;
        }
        None => {
            return Err(Box::new(RuntimeError::new(String::from(
                "unexpected missing pair",
            ))))
        }
    }

    match move_command_pairs.next() {
        Some(pair) => {
            origin = pair.as_str().parse::<usize>()? - 1;
        }
        None => {
            return Err(Box::new(RuntimeError::new(String::from(
                "unexpected missing pair",
            ))))
        }
    }

    match move_command_pairs.next() {
        Some(pair) => {
            destination = pair.as_str().parse::<usize>()? - 1;
        }
        None => {
            return Err(Box::new(RuntimeError::new(String::from(
                "unexpected missing pair",
            ))))
        }
    }

    Ok(ManifestLine::MoveCommand(MoveCommand::new(
        size,
        origin,
        destination,
    )))
}

fn parse_manifest_line(manifest_line: pest::iterators::Pair<Rule>) -> Result<ManifestLine> {
    match manifest_line.into_inner().next() {
        Some(pair) => match pair.as_rule() {
            Rule::cargo_crate_row => parse_crate_row(pair.into_inner()),
            Rule::cargo_crate_stack_row => parse_stack_row(pair.into_inner()),
            Rule::move_command => parse_move_command(pair.into_inner()),
            Rule::manifest_line | Rule::cargo_crate | Rule::cargo_crate_stack => unreachable!(),
        },
        None => Err(Box::new(RuntimeError::new(String::from(
            "unexpected missing pair",
        )))),
    }
}

pub fn parse_cargo_bay_and_move_commands(
    input_file_path: &str,
) -> Result<(CargoBay, MoveCommands)> {
    let mut cargo_crate_rows: Vec<Vec<Option<CargoCrate>>> = Vec::new();
    let mut cargo_crate_row_count: usize = 0;
    let mut move_commands: Vec<MoveCommand> = Vec::new();

    if let Ok(lines) = read_lines(input_file_path) {
        for line in lines {
            match line {
                Ok(line) => {
                    if line.is_empty() {
                        continue;
                    }

                    let cargo_manifest_contents =
                        match CargoManifestParser::parse(Rule::manifest_line, &line) {
                            Ok(manifest_contents) => manifest_contents,
                            Err(err) => return Err(Box::new(err)),
                        };

                    for manifest_line in cargo_manifest_contents {
                        match manifest_line.as_rule() {
                            Rule::manifest_line => match parse_manifest_line(manifest_line)? {
                                ManifestLine::CrateRow(crate_row) => {
                                    cargo_crate_rows.push(crate_row)
                                }
                                ManifestLine::StackCount(stack_count) => {
                                    cargo_crate_row_count = stack_count
                                }
                                ManifestLine::MoveCommand(move_command) => {
                                    move_commands.push(move_command)
                                }
                            },
                            Rule::cargo_crate
                            | Rule::cargo_crate_row
                            | Rule::cargo_crate_stack
                            | Rule::cargo_crate_stack_row
                            | Rule::move_command => unreachable!(),
                        }
                    }
                }
                Err(err) => return Err(Box::new(err)),
            }
        }
    }

    let cargo_bay = CargoBay::new(cargo_crate_row_count, cargo_crate_rows);

    Ok((cargo_bay, move_commands))
}
