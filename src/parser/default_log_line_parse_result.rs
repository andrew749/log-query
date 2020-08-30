use std::collections::HashMap;
use crate::parser::log_line_parse_result::LogLineParseResult;

#[derive(Debug)]
pub struct DefaultLogLineParseResult {
    pub content: HashMap<String, String>,
}

/// The result of parsing a log line
impl DefaultLogLineParseResult {
    pub fn new(content: HashMap<String, String>) -> Self {
        DefaultLogLineParseResult {
            content,
        }
    }
}

impl LogLineParseResult for DefaultLogLineParseResult {
    fn get_content(&self) -> &HashMap<String, String> {
        &self.content
    }

    fn get_field(&self, field: &str) -> Option<&String> {
        self.content.get(field)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_log_line_construction() {
        let log_line = DefaultLogLineParseResult::new(
            [(String::from("test_field"), String::from("test_value"))]
                .iter()
                .cloned()
                .collect::<HashMap<String, String>>(),
        );
        assert_eq!(*log_line.get_field("test_field").unwrap(), "test_value");
    }
}