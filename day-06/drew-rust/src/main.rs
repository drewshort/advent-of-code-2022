use std::collections::{ BTreeMap };
use std::path::Path;
use std::{ env, error::Error };

use aoc_common_lib::error::RuntimeError;
use aoc_common_lib::utility::read_lines;

// Override the alias to use `Box<error::Error>`.
type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
struct ParseBuffer<T> {
    size: usize,
    data_buffer: BTreeMap<T, usize>,
    data_stack: Vec<T>,
}

impl ParseBuffer<char> {
    fn new(size: usize) -> Self {
        ParseBuffer { size, data_buffer: BTreeMap::new(), data_stack: Vec::with_capacity(size) }
    }

    fn add(&mut self, next_character: char) {
        if self.data_stack.len() < self.size {
            // insert or increment the count
            self.data_stack.insert(0, next_character);
            match self.data_buffer.contains_key(&next_character) {
                true => {
                    let next_count = self.data_buffer.get(&next_character).unwrap() + 1;
                    self.data_buffer.insert(next_character, next_count);
                }
                false => {
                    self.data_buffer.insert(next_character, 1);
                }
            }
        } else {
            // Remove or decrement last key in map
            let last_character = self.data_stack.pop().unwrap();
            let last_count = match self.data_buffer.get(&last_character) {
                Some(last_count) => last_count - 1,
                None => 0,
            };
            match last_count {
                0 => {
                    self.data_buffer.remove(&last_character);
                }
                _ => {
                    self.data_buffer.insert(last_character, last_count);
                }
            }

            // Insert or increment the count
            self.data_stack.insert(0, next_character);
            match self.data_buffer.contains_key(&next_character) {
                true => {
                    let next_count = self.data_buffer.get(&next_character).unwrap() + 1;
                    self.data_buffer.insert(next_character, next_count);
                }
                false => {
                    self.data_buffer.insert(next_character, 1);
                }
            }
        }
    }

    fn has_duplicates(&self) -> bool {
        let max_count = *self.data_buffer.values().max().unwrap();
        max_count > 1
    }

    fn is_start_of_packet(&mut self, next_character: &char) -> bool {
        if self.data_stack.len() < self.size || self.has_duplicates() {
            self.add(*next_character);
            false
        } else {
            true
        }
    }
}

fn determine_start_of_packet(datastream: std::str::Chars, buffer_width: usize) -> usize {
    let mut parse_buffer: ParseBuffer<char> = ParseBuffer::new(buffer_width);
    let mut index: usize = 0;
    for character in datastream {
        if parse_buffer.is_start_of_packet(&character) {
            break;
        }
        index += 1;
    }
    index
}

fn parse_message_stream(input_file_path: &str, buffer_width: usize) -> Result<Vec<usize>> {
    let input_file = Path::new(input_file_path);
    if !input_file.exists() {
        let error_message = format!("Path {} does not appear to exist", input_file_path);
        return Err(Box::new(RuntimeError::new(error_message)));
    }
    let mut objs: Vec<usize> = Vec::new();
    if let Ok(lines) = read_lines(input_file) {
        for line in lines {
            match line {
                Ok(line) => {
                    objs.push(determine_start_of_packet(line.chars(), buffer_width));
                }
                Err(err) => {
                    return Err(Box::new(err));
                }
            }
        }
    }
    Ok(objs)
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(Box::new(RuntimeError::new(String::from("Must provide input file path"))));
    }
    let input_path = &args[1];
    let buffer_width: usize = args.get(2).unwrap_or(&String::from("4")).parse::<usize>()?;
    let results = parse_message_stream(input_path, buffer_width);
    println!("{:#?}", results);

    Ok(())
}