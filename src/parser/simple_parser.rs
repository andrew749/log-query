//! Parsers to parse log line and store indexed information into structure.
use regex::Regex;
use simple_error::{require_with, try_with, SimpleError};

use crate::parser::{log_line, timestamp, verbosity, parser_profile};

use std::str::FromStr;
use std::collections::HashMap;


pub struct SimpleParser {
    line_format: String,
    compiled_line_regex: regex::Regex,
}

impl SimpleParser {
    /// Create a new parser with the given settings
    pub fn new(profile: parser_profile::ParserProfile) -> Result<SimpleParser, SimpleError> {
        let compiled_regex: Regex = try_with!(Regex::new(&profile.line_format), "Unable to compile provided spec");
        Ok(SimpleParser{
            line_format: String::from(profile.line_format),
            compiled_line_regex: compiled_regex,
        })
    }

    /// Parse a single log line and marshall data into a struct
    pub fn parse(&self, log: &str) -> Result<log_line::DefaultLogLine, SimpleError> {
        let matches = require_with!(self.compiled_line_regex.captures(log), "Unable to parse log line");
        let timestamp = timestamp::Timestamp::from_match(&matches);
        let verbosity = try_with!(verbosity::Verbosity::from_str(&matches["verbosity"]), "Unable to parse verbosity");
        Ok(log_line::DefaultLogLine {
            timestamp,
            class: Some(matches["class"].into()),
            thread: Some(matches["thread"].into()),
            verbosity,
            content: HashMap::new(),
        })
    }
}