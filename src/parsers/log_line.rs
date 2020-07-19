use crate::parsers::timestamp;
use crate::parsers::verbosity;


pub trait LogLine<T> {
    /// Create a new log line.
    fn new(timestamp: timestamp::Timestamp, class: String, thread: String, verbosity: verbosity::Verbosity, content: T) -> Self;

    /// Get a reference to the timestamp for this log object
    fn get_timestamp(&self) -> &timestamp::Timestamp;

    /// Get a reference to the name of this class for this log object
    fn get_class(&self) -> &String;

    /// Get a reference to the name of this thread for this log object
    fn get_thread(&self) -> &String;

    /// Get a reference to the verbosity of this log object
    fn get_verbosity(&self) -> &verbosity::Verbosity;

    /// Get a reference to the content of this log object
    fn get_content(&self) -> &T;
}

pub struct DefaultLogLine<T> {
    pub timestamp: timestamp::Timestamp,
    pub class: String,
    pub thread: String,
    pub verbosity: verbosity::Verbosity,
    pub content: T,
}

impl<T> LogLine<T> for DefaultLogLine<T> {
    fn new(timestamp: timestamp::Timestamp, class: String, thread: String, verbosity: verbosity::Verbosity, content: T) -> Self {
        DefaultLogLine {
            timestamp,
            class,
            thread,
            verbosity,
            content,
        }
    }

    fn get_content(&self) -> &T {
        &self.content
    }

    fn get_class(&self) -> &String {
        &self.class
    }

    fn get_timestamp(&self) -> &timestamp::Timestamp {
        &self.timestamp
    }

    fn get_thread(&self) -> &String {
        &self.thread
    }

    fn get_verbosity(&self) -> &verbosity::Verbosity {
        &self.verbosity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_log_line_construction() {
        let timestamp = timestamp::Timestamp::new("2020", "07", "10", "10", "23", "02", "111"); 
        let log_line = DefaultLogLine::new(timestamp.clone(), String::from("Class"), String::from("Thread"), verbosity::Verbosity::Debug, String::from("test content"));
        assert_eq!(*log_line.get_timestamp(), timestamp);
        assert_eq!(*log_line.get_class(), "Class");
        assert_eq!(*log_line.get_thread(), "Thread");
        assert_eq!(*log_line.get_verbosity(), verbosity::Verbosity::Debug);
        assert_eq!(*log_line.get_content(), "test content");
    }
}