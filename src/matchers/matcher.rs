use crate::parsers::log_line::LogLine;

/// Check if a log line matches some predicate.
pub trait Matcher<T: LogLine<J>, J> {
    fn matches(&self, log_line: &T) -> bool;
}