use crate::parser::log_line_parse_result::LogLineParseResult;

pub trait Constraint {
    fn check(&self, log_line: &dyn LogLineParseResult) -> bool;
}

#[derive(Debug, Eq, PartialEq)]
/// A constraint that just echos a boolean value
pub struct BooleanConstraint {
    val: bool,
}

impl BooleanConstraint {
    fn new(val: bool) -> Self {
        Self {
            val,
        }
    }
}

impl Constraint for BooleanConstraint {
    fn check(&self, _: &dyn LogLineParseResult) -> bool {
        self.val
    }
}

// The logical AND of two other constraints
pub struct ConjunctionConstraint {
    left: Box<dyn Constraint>,
    right: Box<dyn Constraint>,
}

impl ConjunctionConstraint {
    pub fn new(left: Box<dyn Constraint>, right: Box<dyn Constraint>) -> Self {
        Self {
            left,
            right,
        }
    }
}

impl Constraint for ConjunctionConstraint {
    fn check(&self, log_line: &dyn LogLineParseResult) -> bool {
        self.left.check(log_line) && self.right.check(log_line)
    }
}

/// A simple equality constraint for a key-value pair
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

    fn noop_log_line() -> DefaultLogLineParseResult {
        DefaultLogLineParseResult::new(
            [].iter().cloned().collect::<HashMap<String, String>>(),
        )
    }

    #[test]
    fn test_simple_equality_constraint() {
        let constraint = SimpleEqualityConstraint::new("test_field", "test_value");
        let log_line = DefaultLogLineParseResult::new(
            [(String::from("test_field"), String::from("test_value"))].iter().cloned().collect::<HashMap<String, String>>(),
        );
        assert_eq!(constraint.check(&log_line), true, "Equality of field to expected value");
    }

    #[test]
    fn test_simple_boolean_constraint() {
        let constraint = BooleanConstraint::new(true);
        let log_line = noop_log_line();
        assert_eq!(constraint.check(&log_line), true);

        let constraint = BooleanConstraint::new(false);
        let log_line = noop_log_line();
        assert_eq!(constraint.check(&log_line), false);
    }

    #[test]
    fn test_conjunction_constraint() {
        let bool_constraint_left = Box::new(BooleanConstraint::new(false));
        let bool_constraint_right = Box::new(BooleanConstraint::new(false));
        let conj_constraint = ConjunctionConstraint::new(bool_constraint_left, bool_constraint_right);
        let log_line = noop_log_line();
        assert_eq!(conj_constraint.check(&log_line), false);

        let bool_constraint_left = Box::new(BooleanConstraint::new(false));
        let bool_constraint_right = Box::new(BooleanConstraint::new(true));
        let conj_constraint = ConjunctionConstraint::new(bool_constraint_left, bool_constraint_right);
        assert_eq!(conj_constraint.check(&log_line), false);

        let bool_constraint_left = Box::new(BooleanConstraint::new(true));
        let bool_constraint_right = Box::new(BooleanConstraint::new(true));
        let conj_constraint = ConjunctionConstraint::new(bool_constraint_left, bool_constraint_right);
        assert_eq!(conj_constraint.check(&log_line), true);
    }
}