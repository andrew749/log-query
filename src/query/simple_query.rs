//! Users provide queries that generate constraints that are used as filters
 
use crate::constraint::SimpleEqualityConstraint;
use crate::constraint::Constraint;
use crate::parser::log_line_parse_result::LogLineParseResult;
use chomp::prelude::{parse_only, U8Input, SimpleResult, token, take_till, scan, string};


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
     *      field1="value" and field2="value2"
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

#[derive(Debug, Eq, PartialEq, Clone)]
struct QueryAtom<B> {
    query_key: B,
    query_value: B,
}


#[derive(Debug, Eq, PartialEq, Clone)]
enum QueryOp {
    AND,
    OR,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct QueryTree<T> {
    atom: QueryAtom<T>,
    op: Option<QueryOp>,
    tree: Option<Box<QueryTree<T>>>,
}

/// Parse a single query atom which si a constraint to use in query processing
fn query_atom<I: U8Input>(i: I) -> SimpleResult<I, QueryAtom<I::Buffer>> {
    parse!{i;
        // take the first token up until =
        let query_key = take_till(|c| c == b'=');
        let _ = token(b'=');
        let _ = token(b'"');
        let query_value = scan(false, |s, c| if s { Some(false) }
                                             else if c == b'"' { None }
                                             else { Some(c == b'\\') });
        let _ = token(b'"');
        ret QueryAtom {
            query_key,
            query_value,
        }
    }
}

fn query_op<I: U8Input>(i: I) -> SimpleResult<I, QueryOp> {
    let op = |i, b, r| string(i, b).map(|_| r);
    parse!{i;
        op(b"||", QueryOp::OR) <|>
        op(b"&&", QueryOp::AND)
    }
}

fn tree_right<I: U8Input>(i: I) -> SimpleResult<I, QueryTree<I::Buffer>> {
    parse! {i;
        let atom = query_atom();
        let op = query_op();
        let tree = query_tree();
        ret QueryTree {
            atom,
            op: Some(op),
            tree: Some(Box::new(tree)),
        }
    }
}

fn tree_left<I: U8Input>(i: I) -> SimpleResult<I, QueryTree<I::Buffer>> {
    parse! {i;
        let atom = query_atom();
        ret QueryTree {
            atom,
            op: None,
            tree: None,
        }
    }
}

fn query_tree<I: U8Input>(i: I) -> SimpleResult<I, QueryTree<I::Buffer>> {
    parse!{i;
        tree_right() <|>
        tree_left()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chomp::parsers::Error as CError;

    #[test]
    fn test_simple_query_parse() {
        let query = "field1=value1&field2=value2";
        let query = Query::new(query);
        assert_eq!(query.constraints[0].field_name, "field1");
        assert_eq!(query.constraints[0].field_value, "value1");
        assert_eq!(query.constraints[1].field_name, "field2");
        assert_eq!(query.constraints[1].field_value, "value2");
    }

    #[test]
    fn test_parse_atom() -> Result<(), CError<u8>> {
        let parse_result = parse_only(|i| query_atom(i), b"key=\"value\"").unwrap();
        assert!(
            parse_result == QueryAtom{query_key: "key".as_bytes(), query_value: "value".as_bytes()}
        );
        Ok(())
    }

    #[test]
    fn test_parse_operator() -> Result<(), CError<u8>> {
        let and = parse_only(|i| query_op(i), b"&&").unwrap();
        assert_eq!(and, QueryOp::AND);

        let or = parse_only(|i| query_op(i), b"||").unwrap();
        assert_eq!(or, QueryOp::OR);
        Ok(())
    }

    #[test]
    fn test_parse_tree() {
        let tree = parse_only(|i| query_tree(i), b"a=\"test\"&&b=\"what\"").unwrap();
        assert_eq!(
            tree,
            QueryTree{
                atom: QueryAtom {query_key: "a".as_bytes(), query_value: "test".as_bytes()},
                op: Some(QueryOp::AND),
                tree: Some(Box::new(QueryTree {
                    atom: QueryAtom {query_key: "b".as_bytes(), query_value: "what".as_bytes()},
                    op: None,
                    tree: None
                }))
            });
    }
}