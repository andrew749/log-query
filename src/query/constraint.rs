use crate::parser::log_line_parse_result::LogLineParseResult;

pub trait Constraint {
    fn check(&self, log_line: &dyn LogLineParseResult) -> bool;
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
    fn check(&self, log_line: &dyn LogLineParseResult) -> bool {
        if let Some(field) = log_line.get_field(&self.field_name) {
            return *field == self.field_value
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{default_log_line_parse_result::DefaultLogLineParseResult};
    use std::collections::HashMap;

    #[test]
    fn test_simple_constraint() {
        let constraint = SimpleEqualityConstraint::new("test_field", "test_value");
        let log_line = DefaultLogLineParseResult::new(
            [(String::from("test_field"), String::from("test_value"))].iter().cloned().collect::<HashMap<String, String>>(),
        );
        assert_eq!(constraint.check(&log_line), true, "Equality of field to expected value");
    }
}