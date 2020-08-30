use simple_error::{SimpleError};

/// A simple interface for a entity that can parse a log line and produce a result of generic type.
pub trait Parser {
    type ParserResult;

    /// Parse a single log line and produce a result
    fn parse(&self, log: &str) -> Result<Box<Self::ParserResult>, SimpleError>;

    /// The canonical name of this parser. 
    fn get_name(&self) -> &str;
}