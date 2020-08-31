use chomp::prelude::{parse_only, U8Input, SimpleResult, token, take_till, scan, string};
use simple_error::{bail, try_with, SimpleError};

#[derive(Debug, Eq, PartialEq, Clone)]
/// Constraint on relationships between a key and a value parsed from a log line
pub struct QueryAtom<B> {
    pub query_key: B,
    pub query_constraint: QueryConstraint,
    pub query_value: B,
}

#[derive(Debug, Eq, PartialEq, Clone)]
/// Function describing relationship between key and value in constraint
pub enum QueryConstraint {
    EQ,
}

#[derive(Debug, Eq, PartialEq, Clone)]
/// Combinator for constraints with the behviour of a logical AND
pub enum QueryOp {
    AND,
}

#[derive(Debug, Eq, PartialEq, Clone)]
/// Parse tree for a set of constraints
pub struct QueryTree<T> {
    pub atom: QueryAtom<T>,
    pub op: Option<QueryOp>,
    pub tree: Option<Box<QueryTree<T>>>,
}

/// Parse a single query atom which is a constraint to use in query processing
fn query_atom<I: U8Input>(i: I) -> SimpleResult<I, QueryAtom<I::Buffer>> {
    parse!{i;
        // take the first token up until =
        let query_key = take_till(|c| c == b'=');
        let query_constraint = query_constraint();
        let _ = token(b'"');
        let query_value = scan(false, |s, c| if s { Some(false) }
                                             else if c == b'"' { None }
                                             else { Some(c == b'\\') });
        let _ = token(b'"');
        ret QueryAtom {
            query_key,
            query_constraint,
            query_value,
        }
    }
}

fn query_constraint<I: U8Input>(i: I) -> SimpleResult<I, QueryConstraint> {
    let op = |i, b, r| string(i, b).map(|_| r);
    parse!{i;
       op(b"=", QueryConstraint::EQ)
    }
}

fn query_op<I: U8Input>(i: I) -> SimpleResult<I, QueryOp> {
    let op = |i, b, r| string(i, b).map(|_| r);
    parse!{i;
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

/// Parse a query into a result, if valid
pub fn parse_query(query: &str) -> Result<QueryTree<&[u8]>, SimpleError> {
    let parse_result = parse_only(|i| query_tree(i), query.as_bytes());
    match parse_result {
        Ok(x) => Ok(x),
        Err(x) => bail!("Unable to parse query {}", x.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chomp::parsers::Error as CError;

    #[test]
    fn test_parse_atom() -> Result<(), CError<u8>> {
        let parse_result = parse_only(|i| query_atom(i), b"key=\"value\"").unwrap();
        assert!(
            parse_result == QueryAtom{query_key: "key".as_bytes(), query_constraint: QueryConstraint::EQ, query_value: "value".as_bytes()}
        );
        Ok(())
    }

    #[test]
    fn test_parse_operator() -> Result<(), CError<u8>> {
        let and = parse_only(|i| query_op(i), b"&&").unwrap();
        assert_eq!(and, QueryOp::AND);
        Ok(())
    }

    #[test]
    fn test_parse_tree() {
        let tree = parse_only(|i| query_tree(i), b"a=\"test\"&&b=\"what\"").unwrap();
        assert_eq!(
            tree,
            QueryTree{
                atom: QueryAtom {query_key: "a".as_bytes(), query_constraint: QueryConstraint::EQ, query_value: "test".as_bytes()},
                op: Some(QueryOp::AND),
                tree: Some(Box::new(QueryTree {
                    atom: QueryAtom {query_key: "b".as_bytes(), query_constraint: QueryConstraint::EQ, query_value: "what".as_bytes()},
                    op: None,
                    tree: None
                }))
            });
    }
}