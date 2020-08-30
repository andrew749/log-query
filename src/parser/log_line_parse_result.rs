use std::collections::HashMap;

/// Stores the result of parsing a log line
pub trait LogLineParseResult {
    /// Get a reference to the map of parsed fields in this log line 
    fn get_content(&self) -> &HashMap<String, String>;

    /// Gets the value of specific parsed field for this log line
    fn get_field(&self, field: &str) -> Option<&String>;
}