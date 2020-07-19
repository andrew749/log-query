//! Parse log and store indexed information into structure.

use lazy_static::lazy_static;
use regex::Regex;
use simple_error::{require_with, try_with, SimpleError};
use std::str::FromStr;

use crate::parsers::{timestamp, log_line, verbosity};
use crate::parsers::log_line::Content;

lazy_static! {
    static ref JAVA_LOG_REGEX: Regex = Regex::new(r"(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})\s(?P<hour>\d{2}):(?P<minute>\d{2}):(?P<second>\d{2}).(?P<millisecond>\d{0,3})\s\[(?P<thread>[^\]]+)\]\s\[\]\s(?P<class>\w+)\s\[(?P<verbosity>\w+)\]\s\[(?P<client_id>[^\]]+)\]\s(?P<content>.*)").unwrap();
}

pub struct DefaultContent {
    raw_content: String,
}

impl log_line::Content for DefaultContent {
    fn construct(s: String) -> Self {
        Self {
            raw_content: s,
        }
    }
}

pub fn parse_java(log: &str) -> Result<log_line::LogLine<DefaultContent>, SimpleError> {
    let matches = require_with!(JAVA_LOG_REGEX.captures(log), "Unable to parse log");
    let timestamp = timestamp::Timestamp::from_match(&matches);
    let verbosity = try_with!(verbosity::Verbosity::from_str(&matches["verbosity"]), "Unable to parse verbosity");
    let content = DefaultContent::construct(matches["content"].into());
    Ok(log_line::LogLine {
        timestamp,
        class: matches["class"].into(),
        thread: matches["thread"].into(),
        verbosity,
        content: content,
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parsers::{java_parser, timestamp, verbosity};

    #[test]
    fn test_java_parse() {
        let log_line = "2020-03-11 07:19:21.542 [kafka-admin-client-thread | admin-adminClient] [] NetworkClient [WARN] [AdminClient clientId=admin-adminClient] Error connecting to node ltx1-app2113.stg.linkedin.com:16637 (id: 2113 rack: 405)";
        let parsed_log = match java_parser::parse_java(&log_line) {
            Ok(log) => log,
            Err(err) => panic!(err),
        };
        assert_eq!(parsed_log.timestamp, timestamp::Timestamp::new("2020", "3", "11", "7", "19", "21", "542"));
        assert_eq!(parsed_log.thread, "kafka-admin-client-thread | admin-adminClient");
        assert_eq!(parsed_log.class, "NetworkClient");
        assert_eq!(parsed_log.verbosity, verbosity::Verbosity::Warn);
        assert_eq!(parsed_log.get_content().raw_content, "Error connecting to node ltx1-app2113.stg.linkedin.com:16637 (id: 2113 rack: 405)");
    }
}