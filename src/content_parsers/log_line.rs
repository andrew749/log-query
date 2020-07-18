use crate::content_parsers::timestamp;
use crate::content_parsers::verbosity;

pub struct LogLine {
    pub timestamp: timestamp::Timestamp,
    pub class: String,
    pub thread: String,
    pub verbosity: verbosity::Verbosity,
    pub content: String,
}

impl LogLine {
    fn new(timestamp: timestamp::Timestamp, class: String, thread: String, verbosity: verbosity::Verbosity, content: String) -> Self {
        LogLine{
            timestamp,
            class,
            thread,
            verbosity,
            content,
        }
    }
}