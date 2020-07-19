use crate::parsers::timestamp;
use crate::parsers::verbosity;

pub trait Content {
    fn construct(s: String) -> Self;
}

pub struct LogLine<T: Content> {
    pub timestamp: timestamp::Timestamp,
    pub class: String,
    pub thread: String,
    pub verbosity: verbosity::Verbosity,
    pub content: T,
}

impl<T: Content> LogLine<T> {
    fn new(timestamp: timestamp::Timestamp, class: String, thread: String, verbosity: verbosity::Verbosity, content: T) -> Self {
        LogLine{
            timestamp,
            class,
            thread,
            verbosity,
            content,
        }
    }

    pub fn get_content(&self) -> &T {
        &self.content
    }
}