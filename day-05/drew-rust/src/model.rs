use std::{error::Error, fmt::Display};

use aoc_common_lib::error::RuntimeError;

// Override the alias to use `Box<error::Error>`.
type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct CargoCrate {
    id: char,
}
impl CargoCrate {
    pub fn parse(as_str: &str) -> Option<CargoCrate> {
        if as_str.trim().is_empty() {
            None
        } else {
            Some(CargoCrate {
                id: as_str.trim()[1..2].chars().collect::<Vec<char>>()[0],
            })
        }
    }

    pub fn id(&self) -> char {
        self.id
    }
}

impl Display for CargoCrate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[{}]", self.id))
    }
}

#[derive(Debug)]
pub struct CargoStack {
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
pub struct CargoBay {
    stacks: Vec<CargoStack>,
}

impl CargoBay {
    pub fn new(
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

    pub fn apply(
        &mut self,
        move_command: &MoveCommand,
        move_stacks_together: bool,
    ) -> Result<bool> {
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

    pub fn top(&self) -> Vec<CargoCrate> {
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
pub struct MoveCommand {
    size: usize,
    origin: usize,
    destination: usize,
}

impl MoveCommand {
    pub fn new(size: usize, origin: usize, destination: usize) -> Self {
        Self {
            size,
            origin,
            destination,
        }
    }
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

pub type MoveCommands = Vec<MoveCommand>;
