mod parser;
mod query;

use std::fs;
use simple_error::{SimpleError, try_with};
use crate::parser::*;
use crate::query::*;
pub use crate::parser::simple_parser::SimpleParser;

/// Get a parser profile, describing how the parser should be constructed, from a file
pub fn load_profile_from_file(path: &str) -> Result<parser_profile::ParserProfile, SimpleError>  {
    let data = fs::read_to_string(path).expect("Unable to read file");
    let parser_profile = try_with!(parser_profile::ParserProfile::from_str(&data), "Unable to parse profile");
    Ok(parser_profile) 
}

/// Create parser from a file specifing the parser's properties
pub fn load_parser_from_file(path: &str) -> Result<SimpleParser, SimpleError> {
    let profile = load_profile_from_file(path)?;
    let parser = try_with!(SimpleParser::new(profile), "Unable to construct parser");
    Ok(parser)
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
        let profile = try_with!(parser_profile::ParserProfile::from_str(parser_config), "Unable to construct parser profile");
        let parser = try_with!(
            SimpleParser::new(profile), 
            "Unable to construct parser"
        );
        let parsed_log = match parser.parse(&log_line) {
            Ok(log) => log,
            Err(err) => panic!(err),
        };
        assert_eq!(*parsed_log.get_timestamp(), timestamp::Timestamp::new("2020", "7", "17", "23", "12", "30", Some("037")));
        assert_eq!(parsed_log.get_thread().unwrap(), "ImageManagerImpl-dispatcher");
        assert_eq!(parsed_log.get_class().unwrap(), "ImageManagerImpl");
        assert_eq!(*parsed_log.get_verbosity(), verbosity::Verbosity::Info);
        Ok(())
    }

}