//! Users provide queries that generate constraints that are used as filters
use crate::constraint::{Constraint};
use crate::parser::log_line_parse_result::LogLineParseResult;
use crate::query::query_ast::{parse_query, QueryTree};
use crate::query::constraint_factory::constraint_factory;
use simple_error::SimpleError;

/// User provided parsed query that understands what predicates exist for filtering
/// Query Grammar:
/// ```yac
/// query := atom op query
/// op := &&
/// atom := key="value"
/// key := [\w]+
/// value := [\w]+
/// ```
///
pub struct Query {
    constraints: Box<dyn Constraint>,
}

impl Query {

    /**
     * Create a basic query parser
     * 
     * Currently a query takes the form:
     *      field1="value1" && field2="value2"
     * 
     * For now, only equality is supported, though this is changing.
     */
    pub fn new(raw_query: &str) -> Result<Self, SimpleError> {
        let parse_tree = parse_query(raw_query)?;
        Ok(Self{
            constraints: Self::generate_constraints(parse_tree),
        })
    }

    /// Perform the mapping from a query ast to a constraint program
    fn generate_constraints(parse_tree: QueryTree<&[u8]>) -> Box<dyn Constraint> {
        constraint_factory(parse_tree)
    }

    /**
     * Check that the given log line passes constraints specified in the query
     */
    pub fn check(&self, log_line: &dyn LogLineParseResult) -> bool {
        self.constraints.check(log_line)
    }
}