use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct DomainError {
    pub code: String,
    pub message: String,
}

impl DomainError {
    pub fn new(code: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: Default::default(),
        }
    }
}

impl Display for DomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Code:\t{}\nMessage:\t{}", self.code, self.message)
    }
}

impl Error for DomainError {}
