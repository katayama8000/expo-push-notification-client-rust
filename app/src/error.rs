use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub enum CustomError {
    InvalidArgument(String),
    DeserializeErr(String),
    ServerErr(String),
}
