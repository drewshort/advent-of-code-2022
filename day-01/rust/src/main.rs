use std::{env, error::Error, fs::File, fmt, io::{self, BufRead, BufReader, Lines}, path::Path};

// Override the alias to use `Box<error::Error>`.
type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug, Clone)]
struct RuntimeError {
    message: String
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for RuntimeError {}

#[derive(Debug, Clone, Copy)]
struct Elf {
    id: u8,
    calories: u32
}

fn read_lines<P>(file_path: P) -> io::Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(file_path)?;
    Ok(BufReader::new(file).lines())
}

fn parse_elves(input_file_path: &str) -> Result<Vec<Elf>> {
    let input_file = Path::new(input_file_path);
    if ! input_file.exists() {
        let error_message = format!("Path {} does not appear to exist", input_file_path);
        return Err(Box::new(RuntimeError{message: error_message}));
    }
    
    let mut elves: Vec<Elf> = Vec::new();
    if let Ok(lines) = read_lines(input_file) {
        let mut current_elf_id: u8 = 1;
        let mut current_elf_calories: u32 = 0;

        for line in lines {
            match line {
                Ok(line) => {
                    let trimmed_line = line.trim();
                    if trimmed_line.len() < 1 {
                        elves.push(Elf{id: current_elf_id, calories: current_elf_calories});
                        current_elf_id += 1;
                        current_elf_calories = 0;
                        continue;
                    }
                    let line_calories: u32 = trimmed_line.parse()?;
                    current_elf_calories += line_calories;
                },
                Err(err) => {
                    return Err(Box::new(err));
                }
            }
        }

        // Capture the last elf in the event that the file does not end in a newline
        if current_elf_calories != 0 {
            elves.push(Elf{id: current_elf_id, calories: current_elf_calories});
        }
    }
    
    Ok(elves)
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(Box::new(RuntimeError{message: String::from("Must provide input file path")}));
    }
    let input_path = &args[1];
    let elves = parse_elves(input_path)?;

    let mut elf_with_most_calories: Option<Elf> = None;
    for elf in elves {
        match elf_with_most_calories {
            None => {
                elf_with_most_calories = Some(elf);
                continue;
            },
            Some(_) => ()
        }
        if elf.calories > elf_with_most_calories.unwrap().calories {
            elf_with_most_calories = Some(elf);
        }
    }

    println!("{:?}", elf_with_most_calories.unwrap());
    Ok(())
}
