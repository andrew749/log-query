use crate::constraint::SimpleEqualityConstraint;
use std::collections::HashMap;

/// User provide queries that generate constraints that are used as filters
 
/**
 * Basic query parser where the query takes a form of a simple url parameter
 * 
 * Currently a query takes the form:
 *      field1=value1&field2=value2
 * 
 * For now, only equality is supported, though this is changing.
 */
pub fn parse_query(query: &str) -> Vec<SimpleEqualityConstraint> {
    // Parse into components separated by "&"
    let raw_constraints: Vec<&str> = query.split('&').collect();

    fn split_key_value(facet: &str) -> (&str, &str) {
        let components: Vec<&str> = facet.split('=').collect();
        assert_eq!(components.len(), 2);
        (components[0], components[1])
    }

    // Parse key value pairs
    let splits: Vec<(&str, &str)> = raw_constraints.into_iter().map(split_key_value).collect();
    splits.into_iter().map(|(x, y)| SimpleEqualityConstraint::new(x, y)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_query_parse() {
        let query = "field1=value1&field2=value2";
        let constraints = parse_query(query);
        assert_eq!(constraints[0].field_name, "field1");
        assert_eq!(constraints[0].field_value, "value1");
        assert_eq!(constraints[1].field_name, "field2");
        assert_eq!(constraints[1].field_value, "value2");
    }
}