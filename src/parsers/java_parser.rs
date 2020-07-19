//! Parse log and store indexed information into structure.

use lazy_static::lazy_static;
use regex::Regex;
use simple_error::{require_with, try_with, SimpleError};
use std::str::FromStr;

use crate::parsers::{timestamp, log_line, verbosity};

lazy_static! {
    static ref JAVA_LOG_REGEX: Regex = Regex::new(r"(?P<year>\d{4})/(?P<month>\d{2})/(?P<day>\d{2})\s(?P<hour>\d{2}):(?P<minute>\d{2}):(?P<second>\d{2})\.(?P<millisecond>\d{0,3})\s(?P<verbosity>\w+)\s\[(?P<class>\w+)\]\s\[(?P<thread>[^\]]+)\]\s\[(?P<application>[^\]]+)\]\s\[(?P<client_id>[^\]]*)\]\s(?P<content>.*)").unwrap();
}

/// Parse java log lines for a given Content type.
pub fn parse_java(log: &str) -> Result<log_line::DefaultLogLine<String>, SimpleError> {
    let matches = require_with!(JAVA_LOG_REGEX.captures(log), "Unable to parse log");
    let timestamp = timestamp::Timestamp::from_match(&matches);
    let verbosity = try_with!(verbosity::Verbosity::from_str(&matches["verbosity"]), "Unable to parse verbosity");
    Ok(log_line::DefaultLogLine {
        timestamp,
        class: matches["class"].into(),
        thread: matches["thread"].into(),
        verbosity,
        content: matches["content"].into(),
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parsers::{java_parser, timestamp, verbosity};
    use crate::parsers::log_line::LogLine;

    #[test]
    fn test_java_parse() {
        let log_line = "2020/07/17 23:12:30.037 INFO [ImageManagerImpl] [ImageManagerImpl-dispatcher] [liquid-server-war] [] Process snapshot: Snapshotting not enabled";
        let parsed_log = match java_parser::parse_java(&log_line) {
            Ok(log) => log,
            Err(err) => panic!(err),
        };
        assert_eq!(*parsed_log.get_timestamp(), timestamp::Timestamp::new("2020", "7", "17", "23", "12", "30", "037"));
        assert_eq!(*parsed_log.get_thread(), "ImageManagerImpl-dispatcher");
        assert_eq!(*parsed_log.get_class(), "ImageManagerImpl");
        assert_eq!(*parsed_log.get_verbosity(), verbosity::Verbosity::Info);
        assert_eq!(*parsed_log.get_content(), "Process snapshot: Snapshotting not enabled");
    }
}