use std::collections::HashMap;

pub trait LogLine {
    /// Get relative position of this log line in the file since we don't store the actual data
    fn get_position(&self) -> u64;

    /// Get a reference to the map of parsed fields in this log line 
    fn get_content(&self) -> &HashMap<String, String>;

    /// Gets the value of specific parsed field for this log line
    fn get_field(&self, field: &str) -> Option<&String>;
}

pub struct DefaultLogLine {
    pub content: HashMap<String, String>,
}

impl DefaultLogLine {
    pub fn new(content: HashMap<String, String>) -> Self {
        DefaultLogLine {
            content,
        }
    }
}

impl LogLine for DefaultLogLine {
    
    /// TODO: understand the line number of this log line
    fn get_position(&self) -> u64 {
        unimplemented!();
    }

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
        let log_line = DefaultLogLine::new(
            [(String::from("test_field"), String::from("test_value"))].iter().cloned().collect::<HashMap<String, String>>(),
        );
        assert_eq!(*log_line.get_field("test_field").unwrap(), "test_value");
    }
}