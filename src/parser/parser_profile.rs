use serde::{Deserialize, Serialize};
use std::io::Error;

#[derive(Serialize, Deserialize)]
pub struct ParserProfile {
    pub parser_name: String,
    pub line_format: String,
}

impl ParserProfile {
    pub fn new(parser_name: &str, line_format: &str) -> ParserProfile {
        ParserProfile {
            parser_name: String::from(parser_name),
            line_format: String::from(line_format),
        }
    }

    /// Create a new parser profile given a json string spec for the parser
    pub fn from_str(profile_str: &str) -> Result<ParserProfile, Error> {
        match serde_json::from_str(profile_str) {
            Ok(profile) => Ok(profile),
            Err(parser_error) => Err(Error::from(parser_error)),
        }
    }
    
    ///  Get the name of the parser define by this profile
    pub fn get_name(&self) -> &str {
        &self.parser_name
    }
}