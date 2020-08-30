use crate::parser::log_line_parse_result::LogLineParseResult;

/// Generic trait for being able to 
pub trait OutputGenerator {
    fn get_str(&self, log_line: &dyn LogLineParseResult) -> String;
}