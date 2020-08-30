mod parser;
mod query;
mod output;

#[macro_use]
extern crate chomp;

use std::fs;
use simple_error::{SimpleError, try_with};
use crate::parser::*;
use crate::query::*;
pub use crate::parser::simple_parser::SimpleParser;
pub use crate::parser::log_line_parse_result::LogLineParseResult;
pub use crate::query::simple_query::Query;
pub use crate::parser::parser::Parser;
pub use crate::output::output_generator::OutputGenerator;
pub use crate::output::handlebars_output_generator::HandlebarsOutputGenerator;
pub use crate::output::json_output_generator::JSONOutputGenerator;

/// Get a parser profile, describing how the parser should be constructed, from a file
pub fn load_parser_profile_from_file(path: &str) -> Result<parser_profile::ParserProfile, SimpleError>  {
    let data = fs::read_to_string(path).expect("Unable to read file");
    let parser_profile = try_with!(parser_profile::ParserProfile::from_str(&data), "Unable to parse profile");
    Ok(parser_profile) 
}

/// Create parser from a file specifing the parser's properties
pub fn load_parser_from_file(path: &str) -> Result<SimpleParser, SimpleError> {
    let profile = load_parser_profile_from_file(path)?;
    let parser = try_with!(SimpleParser::from_profile(profile), "Unable to construct parser");
    Ok(parser)
}

/// Create output generator from a template file
pub fn load_output_generator_from_file(path: &str) -> Result<Box<dyn OutputGenerator>, SimpleError> {
    Ok(try_with!(HandlebarsOutputGenerator::from_file(path), "Unable to construct handlebars output generator"))
}

pub fn process_query_on_log_line(query: &simple_query::Query, log_line: &dyn LogLineParseResult) -> bool {
    query.check_constraints(log_line).iter().all(|x| *x == true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parser::Parser;

    fn sample_log() -> String {
        String::from("2020/07/17 23:12:30.037 INFO [ImageManagerImpl] [ImageManagerImpl-dispatcher] [liquid-server-war] [] Process snapshot: Snapshotting not enabled")
    }

    fn toy_parser() -> SimpleParser {
        let parser_config = r#"{
                "parser_name": "test_parser_1",
                "line_format": "(?P<year>\\d{4})/(?P<month>\\d{2})/(?P<day>\\d{2})\\s(?P<hour>\\d{2}):(?P<minute>\\d{2}):(?P<second>\\d{2})\\.(?P<millisecond>\\d{0,3})\\s(?P<verbosity>\\w+)\\s\\[(?P<class>\\w+)\\]\\s\\[(?P<thread>[^\\]]+)\\]\\s\\[(?P<application>[^\\]]+)\\]\\s\\[(?P<client_id>[^\\]]*)\\]\\s(?P<content>.*)"
            }"#;
        let profile = parser_profile::ParserProfile::from_str(parser_config).unwrap();
        
        SimpleParser::from_profile(profile).unwrap()
    }

    #[test]
    fn test_process_query_on_log_line() {
        let parser = toy_parser();
        let parsed_log = match parser.parse(&sample_log()) {
            Ok(log) => log,
            Err(err) => panic!(err),
        };
        let query = Query::new("class=ImageManagerImpl&thread=ImageManagerImpl-dispatcher");
        assert_eq!(process_query_on_log_line(&query, &*parsed_log), true);
    }
    
    #[test]
    fn test_java_parse() -> Result<(), SimpleError> {
        let log_line = sample_log();
        let parser = toy_parser(); 
        let parsed_log = match parser.parse(&log_line) {
            Ok(log) => log,
            Err(err) => panic!(err),
        };
        assert_eq!(parsed_log.get_field("thread").unwrap(), "ImageManagerImpl-dispatcher");
        assert_eq!(parsed_log.get_field("class").unwrap(), "ImageManagerImpl");
        assert_eq!(parsed_log.get_field("verbosity").unwrap(), "INFO");
        Ok(())
    }

}