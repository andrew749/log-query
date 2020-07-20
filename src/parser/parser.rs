//! Parse log and store indexed information into structure.
use regex::Regex;
use simple_error::{require_with, try_with, SimpleError};

use crate::parser::{log_line, timestamp, verbosity};

use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::collections::HashMap;
use std::io::Error;

#[derive(Serialize, Deserialize)]
struct ParserSpec {
    line_format: String,
}

struct SimpleParser {
    line_format: String,
    compiled_line_regex: regex::Regex,
}

impl SimpleParser {
    /// Create a new parser with the given settings
    pub fn new(spec: &str) -> Result<SimpleParser, SimpleError> {
        let parser_spec: ParserSpec = try_with!(serde_json::from_str(spec), "Unable to parse json spec");
        let compiled_regex: Regex = try_with!(Regex::new(&parser_spec.line_format), "Unable to compile provided spec");
        Ok(SimpleParser{
            line_format: String::from(parser_spec.line_format),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{timestamp, verbosity};
    use crate::parser::log_line::LogLine;

    #[test]
    fn test_java_parse() -> Result<(), SimpleError> {
        let log_line = "2020/07/17 23:12:30.037 INFO [ImageManagerImpl] [ImageManagerImpl-dispatcher] [liquid-server-war] [] Process snapshot: Snapshotting not enabled";
        let parser_config = r#"{
                "line_format": "(?P<year>\\d{4})/(?P<month>\\d{2})/(?P<day>\\d{2})\\s(?P<hour>\\d{2}):(?P<minute>\\d{2}):(?P<second>\\d{2})\\.(?P<millisecond>\\d{0,3})\\s(?P<verbosity>\\w+)\\s\\[(?P<class>\\w+)\\]\\s\\[(?P<thread>[^\\]]+)\\]\\s\\[(?P<application>[^\\]]+)\\]\\s\\[(?P<client_id>[^\\]]*)\\]\\s(?P<content>.*)"
            }"#;
        let parser = try_with!(
            SimpleParser::new(&parser_config), 
            "Unable to construct parser"
        );
        let parsed_log = match parser.parse(&log_line) {
            Ok(log) => log,
            Err(err) => panic!(err),
        };
        assert_eq!(*parsed_log.get_timestamp(), timestamp::Timestamp::new("2020", "7", "17", "23", "12", "30", "037"));
        assert_eq!(parsed_log.get_thread().unwrap(), "ImageManagerImpl-dispatcher");
        assert_eq!(parsed_log.get_class().unwrap(), "ImageManagerImpl");
        assert_eq!(*parsed_log.get_verbosity(), verbosity::Verbosity::Info);
        Ok(())
    }
}