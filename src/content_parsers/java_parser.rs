//! Parse log and store indexed information into structure.

use lazy_static::lazy_static;
use regex::Regex;
use simple_error::{require_with, try_with, SimpleError};
use std::str::FromStr;

use crate::content_parsers::{timestamp, log_line, verbosity};

lazy_static! {
    static ref JAVA_LOG_REGEX: Regex = Regex::new(r"(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})\s(?P<hour>\d{2}):(?P<minute>\d{2}):(?P<second>\d{2}).(?P<millisecond>\d{0,3})\s\[(?P<thread>[^\]]+)\]\s\[\]\s(?P<class>\w+)\s\[(?P<verbosity>\w+)\]\s\[(?P<client_id>[^\]]+)\]\s(?P<content>.*)").unwrap();
}

pub fn parse_java(log: &str) -> Result<log_line::LogLine, SimpleError> {
    let matches = require_with!(JAVA_LOG_REGEX.captures(log), "Unable to parse log");
    let timestamp = timestamp::Timestamp::from_match(&matches);
    let verbosity = try_with!(verbosity::Verbosity::from_str(&matches["verbosity"]), "Unable to parse verbosity");
    Ok(log_line::LogLine {
        timestamp,
        class: matches["class"].into(),
        thread: matches["thread"].into(),
        verbosity,
        content: matches["content"].into(),
    })
}

