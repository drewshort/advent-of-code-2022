extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::fmt::Display;
use std::{env, error::Error};

use aoc_common_lib::utility::read_lines;
use pest::Parser;

use aoc_common_lib::error::RuntimeError;

#[derive(Parser)]
#[grammar = "grammar.pest"] // relative to src
struct CargoManifestParser;

// Override the alias to use `Box<error::Error>`.
type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
struct CargoCrate {
    id: char,
}
impl CargoCrate {
    fn parse(as_str: &str) -> Option<CargoCrate> {
        if as_str.trim().is_empty() {
            None
        } else {
            Some(CargoCrate {
                id: as_str.trim()[1..2].chars().collect::<Vec<char>>()[0],
            })
        }
    }
}

impl Display for CargoCrate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[{}]", self.id))
    }
}

#[derive(Debug)]
struct CargoStack {
    id: usize,
    stack: Vec<CargoCrate>,
}

impl CargoStack {
    fn add(&mut self, cargo_crate: &Option<CargoCrate>) {
        match cargo_crate {
            Some(cargo_crate) => self.stack.push(CargoCrate { id: cargo_crate.id }),
            None => (),
        }
    }

    fn add_stack(&mut self, cargo_stack: CargoStack) {
        for cargo_crate in cargo_stack.stack {
            self.add(&Some(cargo_crate))
        }
    }

    fn remove(&mut self) -> Option<CargoCrate> {
        self.stack.pop()
    }

    fn remove_stack(&mut self, size: usize, collect_in_place: bool) -> Self {
        let mut cargo_stack: Vec<CargoCrate> = Vec::new();
        for _ in 0..size {
            match self.remove() {
                Some(cargo_crate) => {
                    if collect_in_place {
                        cargo_stack.insert(0, cargo_crate)
                    } else {
                        cargo_stack.push(cargo_crate)
                    }
                }
                None => continue,
            }
        }
        CargoStack {
            id: 0,
            stack: cargo_stack,
        }
    }

    fn get(&self, height: usize) -> Option<&CargoCrate> {
        self.stack.get(height)
    }

    fn top(&self) -> Option<CargoCrate> {
        self.stack.last().map(|last| CargoCrate { id: last.id })
    }

    fn height(&self) -> usize {
        self.stack.len()
    }
}

impl Display for CargoStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}",
            self.stack
                .iter()
                .map(|cargo_crate| format!("{}", cargo_crate))
                .collect::<Vec<String>>()
                .join(" -> ")
        ))
    }
}

#[derive(Debug)]
struct CargoBay {
    stacks: Vec<CargoStack>,
}

impl CargoBay {
    fn new(
        cargo_crate_row_count: usize,
        cargo_crate_rows: Vec<Vec<Option<CargoCrate>>>,
    ) -> CargoBay {
        let mut cargo_bay_stacks: Vec<CargoStack> = Vec::new();
        (0..cargo_crate_row_count).for_each(|index| {
            cargo_bay_stacks.push(CargoStack {
                id: index,
                stack: Vec::new(),
            })
        });

        for cargo_crate_row in cargo_crate_rows.iter().rev() {
            let mut cargo_crates = cargo_crate_row.iter();
            for index in 0..cargo_crate_row_count {
                let cargo_crate = match cargo_crates.next() {
                    Some(cargo_crate) => cargo_crate,
                    None => continue,
                };
                let cargo_bay_stack = cargo_bay_stacks.get_mut(index).unwrap();
                cargo_bay_stack.add(cargo_crate);
            }
        }

        CargoBay {
            stacks: cargo_bay_stacks,
        }
    }

    fn apply(&mut self, move_command: MoveCommand, move_stacks_together: bool) -> Result<bool> {
        let origin_cargo_stack = self.stacks.get_mut(move_command.origin);
        let moved_cargo_stack: CargoStack;

        match origin_cargo_stack {
            Some(origin_cargo_stack) => {
                moved_cargo_stack =
                    origin_cargo_stack.remove_stack(move_command.size, move_stacks_together)
            }
            None => {
                return Err(Box::new(RuntimeError::new(format!(
                    "Error applying move command: {}",
                    move_command
                ))))
            }
        }

        match self.stacks.get_mut(move_command.destination) {
            Some(destination_cargo_stack) => {
                destination_cargo_stack.add_stack(moved_cargo_stack);
            }
            None => {
                return Err(Box::new(RuntimeError::new(format!(
                    "Error applying move command: {}",
                    move_command
                ))))
            }
        };

        Ok(true)
    }

    fn top(&self) -> Vec<CargoCrate> {
        self.stacks
            .iter()
            .filter_map(|cargo_stack| cargo_stack.top())
            .collect()
    }
}

impl Display for CargoBay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_height = self
            .stacks
            .iter()
            .map(|stack| stack.height())
            .max()
            .unwrap_or(0);

        for layer in (0..max_height).rev() {
            let mut layer_crates: Vec<String> = Vec::new();

            for stack in self.stacks.iter() {
                layer_crates.push(match stack.get(layer) {
                    Some(cargo_crate) => format!("{}", cargo_crate),
                    None => String::from("   "),
                });
            }
            f.write_fmt(format_args!("{}\n", layer_crates.join(" ")))?;
        }

        for stack in self.stacks.iter() {
            f.write_fmt(format_args!(" {}  ", stack.id))?;
        }
        f.write_str("\n")?;

        Ok(())
    }
}

#[derive(Debug)]
struct MoveCommand {
    size: usize,
    origin: usize,
    destination: usize,
}

impl Display for MoveCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "move {} from {} to {}",
            self.size,
            self.origin + 1,
            self.destination + 1
        ))
    }
}

type MoveCommands = Vec<MoveCommand>;

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

    Ok(ManifestLine::MoveCommand(MoveCommand {
        size,
        origin,
        destination,
    }))
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

fn parse_cargo_bay_and_move_commands(input_file_path: &str) -> Result<(CargoBay, MoveCommands)> {
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

    println!("{}", &cargo_bay);

    for move_command in move_commands {
        cargo_bay.apply(move_command, move_stacks_together)?;
    }

    println!("{}", &cargo_bay);

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
            .map(|cargo_crate| format!("{}", cargo_crate.id))
            .collect::<Vec<String>>()
            .join("")
    );

    Ok(())
}
