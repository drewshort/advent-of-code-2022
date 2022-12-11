use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub struct RuntimeError {
    message: String,
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for RuntimeError {}

impl RuntimeError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}
