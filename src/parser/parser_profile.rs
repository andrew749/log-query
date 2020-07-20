use serde::{Deserialize, Serialize};
use std::io::Error;

#[derive(Serialize, Deserialize)]
pub struct ParserProfile {
    pub line_format: String,
}

impl ParserProfile {
    pub fn new(line_format: &str) -> ParserProfile {
        ParserProfile {
            line_format: String::from(line_format),
        }
    }
    pub fn from_str(profile_str: &str) -> Result<ParserProfile, Error> {
        match serde_json::from_str(profile_str) {
            Ok(profile) => Ok(profile),
            Err(parser_error) => Err(Error::from(parser_error)),
        }
    }
}