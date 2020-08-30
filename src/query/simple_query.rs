//! Users provide queries that generate constraints that are used as filters
 
use crate::constraint::SimpleEqualityConstraint;
use crate::constraint::Constraint;
use crate::parser::log_line_parse_result::LogLineParseResult;


/// User provided parsed query that understands what predicates exist for filtering
pub struct Query {
    constraints: Vec<SimpleEqualityConstraint>,
}

impl Query {

    /**
     * Basic query parser where the query takes a form of a simple url parameter
     * 
     * Currently a query takes the form:
     *      field1=value1&field2=value2
     * 
     * For now, only equality is supported, though this is changing.
     */
    pub fn new(raw_query: &str) -> Self {
        // Parse into components separated by "&"
        let raw_constraints: Vec<&str> = raw_query.split('&').collect();

        fn split_key_value(facet: &str) -> (&str, &str) {
            let components: Vec<&str> = facet.split('=').collect();
            assert_eq!(components.len(), 2);
            (components[0], components[1])
        }

        // Parse key value pairs
        let splits: Vec<(&str, &str)> = raw_constraints.into_iter().map(split_key_value).collect();
        Query {
            constraints: splits.into_iter().map(|(x, y)| SimpleEqualityConstraint::new(x, y)).collect()
        }
    }

    /**
     * Check that the given log line passes all predicates provided in this query
     */
    pub fn check_constraints(&self, log_line: &dyn LogLineParseResult) -> Vec<bool> {
        self.constraints.iter().map(|x| x.check(log_line)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_query_parse() {
        let query = "field1=value1&field2=value2";
        let query = Query::new(query);
        assert_eq!(query.constraints[0].field_name, "field1");
        assert_eq!(query.constraints[0].field_value, "value1");
        assert_eq!(query.constraints[1].field_name, "field2");
        assert_eq!(query.constraints[1].field_value, "value2");
    }
}