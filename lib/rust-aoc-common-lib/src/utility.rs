use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::Path,
};

pub fn read_lines<P>(file_path: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(file_path)?;
    Ok(BufReader::new(file).lines())
}
