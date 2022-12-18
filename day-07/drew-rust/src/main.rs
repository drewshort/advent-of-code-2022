extern crate r3bl_rs_utils;
extern crate pest;
#[macro_use]
extern crate pest_derive;

mod model;
mod parser;

use std::path::Path;
use std::{ env, error::Error };

use aoc_common_lib::error::RuntimeError;
use aoc_common_lib::utility::read_lines;
use model::{ Tree, FilesystemNode, FilesystemLeaf };
use parser::{ parse_shell_line, ShellLine };

// Override the alias to use `Box<error::Error>`.
type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn convert_shell_lines_to_filesystem_tree(filesystem_root: &mut FilesystemNode, lines: Vec<Option<ShellLine>>) {
    let mut current_filesystem: &mut FilesystemNode = filesystem_root;

    for shell_line in lines.iter() {
        match shell_line {
            Some(shell_line) =>
                match shell_line {
                    ShellLine::ShellCommand(shell_command) => {
                        current_filesystem = handle_shell_command(current_filesystem, shell_command);
                    }
                    ShellLine::ShellDirectory(shell_directory) => {
                        current_filesystem = handle_shell_directory(current_filesystem, shell_directory);
                    }
                    ShellLine::ShellFile(shell_file) => {
                        current_filesystem = handle_shell_file(current_filesystem, shell_file);
                    }
                }
            None => {
                continue;
            }
        }
    }
}

fn handle_shell_file<'a>(
    current_filesystem: &'a mut FilesystemNode<'a>,
    shell_file: &'a parser::ShellFile
) -> &'a mut FilesystemNode<'a> {
    // current_filesystem.add_file(FilesystemLeaf::new(shell_file.name(), shell_file.size()));
    current_filesystem
}

fn handle_shell_directory<'a>(
    current_filesystem: &'a mut FilesystemNode<'a>,
    shell_directory: &'a parser::ShellDirectory
) -> &'a mut FilesystemNode<'a> {
    // current_filesystem.add_directory(&shell_directory.name());
    current_filesystem
}

fn handle_shell_command<'a>(
    current_filesystem: &'a mut FilesystemNode<'a>,
    shell_command: &'a parser::ShellCommand
) -> &'a mut FilesystemNode<'a> {
    current_filesystem
}

fn parse_spam(input_file_path: &str) -> Result<Vec<Option<ShellLine>>> {
    let input_file = Path::new(input_file_path);
    if !input_file.exists() {
        let error_message = format!("Path {} does not appear to exist", input_file_path);
        return Err(Box::new(RuntimeError::new(error_message)));
    }
    let mut shell_lines: Vec<Option<ShellLine>> = Vec::new();
    if let Ok(lines) = read_lines(input_file) {
        for line in lines {
            match line {
                Ok(line) => {
                    shell_lines.push(parse_shell_line(&line));
                }
                Err(err) => {
                    return Err(Box::new(err));
                }
            }
        }
    }
    Ok(shell_lines)
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(Box::new(RuntimeError::new(String::from("Must provide input file path"))));
    }
    let input_path = &args[1];
    let results = parse_spam(input_path)?;
    let mut filesystem_root = Tree::new();
    let filesystem = convert_shell_lines_to_filesystem_tree(&mut filesystem_root, results);

    Ok(())
}