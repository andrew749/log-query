//! Parsers to parse log line and store indexed information into structure.
use regex::{Captures, Regex};
use simple_error::{require_with, try_with, SimpleError};

use crate::parser::{log_line, parser_profile};

use std::collections::HashMap;


pub struct SimpleParser {
    line_format: String,
    compiled_line_regex: regex::Regex,
}

impl SimpleParser {
    /// Create a new parser with the given settings
    pub fn new(regex_str: &str) -> Result<SimpleParser, SimpleError> {
        Ok(SimpleParser{
            line_format: String::from(regex_str),
            compiled_line_regex: try_with!(Regex::new(regex_str), "Unable to compile provided spec"),
        })
    }

    pub fn from_profile(profile: parser_profile::ParserProfile) -> Result<SimpleParser, SimpleError> {
        Self::new(&profile.line_format)
    }

    /// Parse a single log line and marshall data into a struct
    pub fn parse(&self, log: &str) -> Result<log_line::DefaultLogLine, SimpleError> {
        let matches: Captures = require_with!(self.compiled_line_regex.captures(log), "Unable to parse log line");
        let matches: HashMap<String, String> = matches.iter()
            .zip(self.compiled_line_regex.capture_names())
            .filter_map(|(x, y)| match (x,y) {
                (Some(x), Some(y)) => Some((x,y)),
                _ => None,
            } ) // only take matches where the value is captured
            .map(|(capture, name)| (capture.as_str(), name))
            .map(|(capture, name)| (String::from(name), String::from(capture)))
            .collect();
        Ok(log_line::DefaultLogLine::new(matches))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use log_line::LogLine;

    #[test]
    fn test_simple_parse() -> Result<(), SimpleError> {
        let parser = SimpleParser::new(r"(?P<test_capture_group>test_[a-z]+)").unwrap();
        let log = "test_key";
        assert_eq!(parser.parse(log)?.get_field("test_capture_group").unwrap(), "test_key");
        Ok(())
    }
}