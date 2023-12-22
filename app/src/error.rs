use std::fmt::{Display, Formatter, Result};

use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub enum CustomError {
    InvalidArgument(String),
    DeserializeErr(String),
    ServerErr(String),
}

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            CustomError::InvalidArgument(msg) => write!(f, "Invalid argument: {}", msg),
            CustomError::ServerErr(msg) => write!(f, "Server error: {}", msg),
            CustomError::DeserializeErr(msg) => write!(f, "Deserialize error: {}", msg),
        }
    }
}
