use crate::output::output_generator::OutputGenerator;
use crate::parser::log_line_parse_result::LogLineParseResult;
use serde_json::json;

pub struct JSONOutputGenerator {
}

impl JSONOutputGenerator {
    pub fn new() -> Self {
        JSONOutputGenerator {

        }
    }
}

impl OutputGenerator for JSONOutputGenerator {
    fn get_str(&self, log_line: &dyn LogLineParseResult) -> String {
        json!(log_line.get_content()).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::default_log_line_parse_result::DefaultLogLineParseResult;

    #[test]
    fn test_json_output() {
        let log_line = DefaultLogLineParseResult::new(vec![(String::from("test"), String::from("test value"))].into_iter().collect());
        let output_generator = JSONOutputGenerator::new();
        assert_eq!(output_generator.get_str(&log_line), "{\"test\":\"test value\"}");
    }
}