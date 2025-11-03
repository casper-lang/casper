use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use crate::errors::CasperError;

#[derive(Debug)]
pub(crate) struct InvalidCommandError {
    explanation: String,
}

impl InvalidCommandError {
    pub fn new(explanation: &str) -> Self {
        Self {
            explanation: explanation.to_string(),
        }
    }
}

impl CasperError for InvalidCommandError {}

impl Error for InvalidCommandError {}

impl Display for InvalidCommandError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Casper Error :: Invalid Command : {}", self.explanation)
    }   
}
