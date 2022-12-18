use std::error::Error;

use pest::Parser;

// Override the alias to use `Box<error::Error>`.
type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Parser)]
#[grammar = "grammar.pest"] // relative to src
pub struct ShellParser;

pub struct ShellCommand {
    command: String,
    args: Vec<String>,
}

impl ShellCommand {
    pub fn new(command: String, args: Vec<String>) -> Self {
        Self { command, args }
    }

    pub fn command(&self) -> &str {
        self.command.as_ref()
    }

    pub fn args(&self) -> &[String] {
        self.args.as_ref()
    }
}

pub struct ShellDirectory {
    name: String,
}

impl ShellDirectory {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn name(&self) -> String {
        String::from(self.name)
    }
}

pub struct ShellFile {
    name: String,
    size: usize,
}

impl ShellFile {
    pub fn new(name: String, size: usize) -> Self {
        Self { name, size }
    }

    pub fn name(&self) -> String {
        String::from(&self.name)
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

pub enum ShellLine {
    ShellCommand(ShellCommand),
    ShellDirectory(ShellDirectory),
    ShellFile(ShellFile),
}

pub fn parse_shell_line(line: &str) -> Option<ShellLine> {
    todo!()
}