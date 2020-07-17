use std::str::FromStr;
use regex::Regex;
use simple_error::SimpleError;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate simple_error;

lazy_static! {
    static ref JAVA_LOG_REGEX: Regex = Regex::new(r"(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})\s(?P<hour>\d{2}):(?P<minute>\d{2}):(?P<second>\d{2}).(?P<millisecond>\d{0,3})\s\[(?P<thread>[^\]]+)\]\s\[\]\s(?P<class>\w+)\s\[(?P<verbosity>\w+)\]\s\[(?P<client_id>[^\]]+)\]\s(?P<content>.*)").unwrap();
}

#[derive(Debug, Eq, PartialEq)]
struct Timestamp {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
    millisecond: u32,
}

impl Timestamp {
    fn new(year: &str, month: &str, day: &str, hour: &str, minute: &str, second: &str, millisecond: &str) -> Self {
        Timestamp {
            year: year.parse::<u32>().unwrap(),
            month: month.parse::<u32>().unwrap(),
            day: day.parse::<u32>().unwrap(),
            hour: hour.parse::<u32>().unwrap(),
            minute: minute.parse::<u32>().unwrap(),
            second: second.parse::<u32>().unwrap(),
            millisecond: millisecond.parse::<u32>().unwrap(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Verbosity {
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

pub struct LogLine {
    timestamp: Timestamp,
    class: String,
    thread: String,
    verbosity: Verbosity,
    content: String,
}


fn timestamp_from_match(captures: &regex::Captures) -> Timestamp {
    Timestamp::new(
        &captures["year"],
        &captures["month"],
        &captures["day"],
        &captures["hour"],
        &captures["minute"],
        &captures["second"],
        &captures["millisecond"],
    )
}

pub fn parse_java(log: &str) -> Result<LogLine, SimpleError> {
    let matches = require_with!(JAVA_LOG_REGEX.captures(log), "Unable to parse log");
    let timestamp = timestamp_from_match(&matches);
    let verbosity = try_with!(Verbosity::from_str(&matches["verbosity"]), "Unable to parse verbosity");
    Ok(LogLine {
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

    #[test]
    fn test_java_parse() {
        let log_line = "2020-03-11 07:19:21.542 [kafka-admin-client-thread | admin-adminClient] [] NetworkClient [WARN] [AdminClient clientId=admin-adminClient] Error connecting to node ltx1-app2113.stg.linkedin.com:16637 (id: 2113 rack: 405)";
        let parsed_log = match parse_java(&log_line) {
            Ok(log) => log,
            Err(err) => panic!(err),
        };
        assert_eq!(parsed_log.timestamp, Timestamp{year: 2020, month: 3, day: 11, hour: 7, minute: 19, second: 21, millisecond: 542});
        assert_eq!(parsed_log.thread, "kafka-admin-client-thread | admin-adminClient");
        assert_eq!(parsed_log.class, "NetworkClient");
        assert_eq!(parsed_log.verbosity, Verbosity::Warn);
        assert_eq!(parsed_log.content, "Error connecting to node ltx1-app2113.stg.linkedin.com:16637 (id: 2113 rack: 405)");
    }
}