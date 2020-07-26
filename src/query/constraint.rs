use crate::parser::{log_line, timestamp, verbosity};
use crate::parser::log_line::LogLine;
use std::collections::HashMap;

trait Constraint {
    fn check(&self, log_line: log_line::DefaultLogLine) -> bool;
}

/// A simple equality constraint
pub struct SimpleEqualityConstraint {
    pub field_name: String,
    pub field_value: String,
}

impl SimpleEqualityConstraint {
    /// Create a new equality constraint
    pub fn new(field_name: &str, field_value: &str) -> Self {
        SimpleEqualityConstraint {
            field_name: String::from(field_name),
            field_value: String::from(field_value),
        }
    }
}

impl Constraint for SimpleEqualityConstraint {
    fn check(&self, log_line: log_line::DefaultLogLine) -> bool {
        if let Some(field) = log_line.get_field(&self.field_name) {
            return *field == self.field_value
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_constraint() {
        let constraint = SimpleEqualityConstraint::new("test_field", "test_value");
        let timestamp = timestamp::Timestamp::new("2020", "07", "10", "10", "23", "02", Some("111")); 
        let log_line = log_line::DefaultLogLine::new(
            timestamp.clone(),
            Some(String::from("Class")), 
            Some(String::from("Thread")), 
            verbosity::Verbosity::Debug, 
            [(String::from("test_field"), String::from("test_value"))].iter().cloned().collect::<HashMap<String, String>>(),
        );
        assert_eq!(constraint.check(log_line), true, "Equality of field to expected value");
    }
}