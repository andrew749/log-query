//! Log verbosity levels.

use simple_error::{bail, SimpleError};
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
pub enum Verbosity {
    Debug,
    Info,
    Error,
    Warn,
}

impl FromStr for Verbosity {
    type Err =  SimpleError;

    fn from_str(s: &str) -> Result<Verbosity, SimpleError> {
        Ok(match s {
            "DEBUG" => Verbosity::Debug,
            "INFO" => Verbosity::Info,
            "ERROR" => Verbosity::Error,
            "WARN" => Verbosity::Warn,
            _ => bail!("Unknown verbosity"),
        })
    }
}