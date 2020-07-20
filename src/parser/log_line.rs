use crate::parser::timestamp;
use crate::parser::verbosity;
use std::collections::HashMap;

pub trait LogLine {
    /// Get relative position of this log line in the file since we don't store the actual data
    fn get_position(&self) -> u64;

    /// Create a new log line.
    fn new(timestamp: timestamp::Timestamp, class: Option<String>, thread: Option<String>, verbosity: verbosity::Verbosity, content: HashMap<String, String>) -> Self;

    /// Get a reference to the timestamp for this log object
    fn get_timestamp(&self) -> &timestamp::Timestamp;

    /// Get a reference to the name of this class for this log object
    fn get_class(&self) -> Option<&String>;

    /// Get a reference to the name of this thread for this log object
    fn get_thread(&self) -> Option<&String>;

    /// Get a reference to the verbosity of this log object
    fn get_verbosity(&self) -> &verbosity::Verbosity;

    /// Get a reference to the map of parsed fields in this log line 
    fn get_content(&self) -> &HashMap<String, String>;

    /// Gets the value of specific parsed field for this log line
    fn get_field(&self, field: &str) -> Option<&String>;
}

pub struct DefaultLogLine {
    pub timestamp: timestamp::Timestamp,
    pub class: Option<String>,
    pub thread: Option<String>,
    pub verbosity: verbosity::Verbosity,
    pub content: HashMap<String, String>,
}

impl LogLine for DefaultLogLine {
    fn new(timestamp: timestamp::Timestamp, class: Option<String>, thread: Option<String>, verbosity: verbosity::Verbosity, content: HashMap<String, String>) -> Self {
        DefaultLogLine {
            timestamp,
            class,
            thread,
            verbosity,
            content,
        }
    }

    fn get_position(&self) -> u64 {
        unimplemented!();
    }

    fn get_class(&self) -> Option<&String> {
        match &self.class {
            Some(class) => Some(&class),
            None => None,
        }
    }

    fn get_thread(&self) -> Option<&String> {
        match &self.thread {
            Some(thread) => Some(&thread),
            None => None,
        }
    }

    fn get_timestamp(&self) -> &timestamp::Timestamp {
        &self.timestamp
    }

    fn get_verbosity(&self) -> &verbosity::Verbosity {
        &self.verbosity
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
        let timestamp = timestamp::Timestamp::new("2020", "07", "10", "10", "23", "02", "111"); 
        let log_line = DefaultLogLine::new(
            timestamp.clone(), 
            Some(String::from("Class")), 
            Some(String::from("Thread")), 
            verbosity::Verbosity::Debug, 
            [(String::from("test_field"), String::from("test_value"))].iter().cloned().collect::<HashMap<String, String>>(),
        );
        assert_eq!(*log_line.get_timestamp(), timestamp);
        assert_eq!(log_line.get_class().unwrap(), "Class");
        assert_eq!(log_line.get_thread().unwrap(), "Thread");
        assert_eq!(*log_line.get_verbosity(), verbosity::Verbosity::Debug);
        assert_eq!(*log_line.get_field("test_field").unwrap(), "test_value");
    }
}