//! Parsers to parse log line and store indexed information into structure.
use regex::{Captures, Regex};
use simple_error::{require_with, try_with, SimpleError};

use crate::parser::default_log_line_parse_result::DefaultLogLineParseResult;
use crate::parser::{parser_profile, parser::Parser};
use crate::parser::parser_profile::ParserProfile;

use std::collections::HashMap;


pub struct SimpleParser {
    profile: parser_profile::ParserProfile,
    compiled_line_regex: regex::Regex,
}

impl SimpleParser {
    /// Create a new parser with the given settings
    pub fn new(profile_name: &str, regex_str: &str) -> Result<Self, SimpleError> {
        Ok(SimpleParser{
            profile: ParserProfile::new(profile_name, regex_str),
            compiled_line_regex: try_with!(Regex::new(regex_str), "Unable to compile provided spec"),
        })
    }

    pub fn from_profile(profile: parser_profile::ParserProfile) -> Result<Self, SimpleError> {
        Self::new(&profile.parser_name, &profile.line_format)
    }
}

impl<'a> Parser for SimpleParser {
    type ParserResult = DefaultLogLineParseResult;

    /// Parse a single log line and marshall data into a struct
    fn parse(&self, log: &str) -> Result<Box<DefaultLogLineParseResult>, SimpleError> {
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
        Ok(Box::new(DefaultLogLineParseResult::new(matches)))
    }

    fn get_name(&self) -> &str {
        self.profile.get_name()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::log_line_parse_result::LogLineParseResult;

    #[test]
    fn test_simple_parse() -> Result<(), SimpleError> {
        let parser = SimpleParser::new("simple_parser", r"(?P<test_capture_group>test_[a-z]+)").unwrap();
        let log = "test_key";
        assert_eq!(parser.parse(log)?.get_field("test_capture_group").unwrap(), "test_key");
        Ok(())
    }
}