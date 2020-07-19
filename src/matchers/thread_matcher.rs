use crate::parsers::log_line::{DefaultLogLine, LogLine};
use crate::matchers::matcher::Matcher;

struct ThreadMatcher {
    thread_name: String,
}

impl ThreadMatcher {
    fn new(thread_name: &str) -> Self {
        ThreadMatcher {
            thread_name: thread_name.into(),
        }
    }
}

impl<T> Matcher<DefaultLogLine<T>, T> for ThreadMatcher {
    fn matches(&self, log_line: &DefaultLogLine<T>) -> bool {
        *log_line.get_thread() == self.thread_name
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::{timestamp, verbosity};

    #[test]
    fn test_java_log_line_matches_thread() {
        let thread_name = "Thread";
        let matcher = ThreadMatcher::new(thread_name);
        let timestamp = timestamp::Timestamp::new("2020", "07", "10", "10", "23", "02", "111"); 
        let log_line = DefaultLogLine::new(timestamp.clone(), String::from("Class"), thread_name.to_string(), verbosity::Verbosity::Debug, String::from("test"));
        assert!(matcher.matches(&log_line));

        let log_line = DefaultLogLine::new(timestamp.clone(), String::from("Class"), String::from("Thread2"), verbosity::Verbosity::Debug, String::from("test"));
        assert!(!matcher.matches(&log_line));
    }
}