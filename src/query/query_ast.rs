use chomp::prelude::{parse_only, U8Input, SimpleResult, ParseResult, token, take_till, scan, string, skip_while};
use simple_error::{bail, SimpleError};

/// token_to_enum(input, match, return) Returns `return` if tokens on `input` equal `match`.
fn token_to_enum<I: U8Input, R>(i: I, b: &[u8], r: R) -> ParseResult<I, R,  chomp::parsers::Error<u8>> {
    string(i, b).map(|_| r)
}

/// Skip tokens while they match whitespace characters.
fn skip_whitespace<I: U8Input>(i: I) -> ParseResult<I, (),  chomp::parsers::Error<u8>> {
    skip_while(i, |c| (c as char).is_whitespace())
}

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
pub enum QueryOpTerm {
    AND,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum QueryOpExpression{
    OR,
}

/// Higher precedence parse structure
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum QueryTerm<T> {
    Unary(QueryAtom<T>),
    Binary(QueryAtom<T>, QueryOpTerm, Box<QueryTerm<T>>),
}

/// Lower precedence parse structure
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum QueryExpression<T> {
    Unary(QueryTerm<T>),
    Binary(QueryTerm<T>, QueryOpExpression, Box<QueryExpression<T>>),
}

#[derive(Debug, Eq, PartialEq, Clone)]
/// Parse tree for a set of constraints
pub struct Query<T>{
    pub tree: QueryExpression<T>
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

fn query_expression<I: U8Input>(i: I) -> SimpleResult<I, QueryExpression<I::Buffer>> {
    fn binary_parser<I: U8Input>(i: I) -> SimpleResult<I, QueryExpression<I::Buffer>> {
        parse!{i;
            let term = query_term();
            skip_whitespace();
            let op = query_op_expression();
            skip_whitespace();
            let expr = query_expression();
            ret QueryExpression::Binary(term, op, Box::new(expr))
        }
    }
    fn unary_parser<I: U8Input>(i: I) -> SimpleResult<I, QueryExpression<I::Buffer>> {
        parse!{i;
            let term = query_term();
            ret QueryExpression::Unary(term)
        }
    }
    parse!{i;
        binary_parser() <|>
        unary_parser()
    }
}

fn query_term<I: U8Input>(i: I) -> SimpleResult<I, QueryTerm<I::Buffer>> {
    fn query_term_binary<I: U8Input>(i: I) -> SimpleResult<I, QueryTerm<I::Buffer>> {
        parse!{i;
            let atom = query_atom();
            skip_whitespace();
            let op = query_op_term();
            skip_whitespace();
            let term = query_term();
            ret QueryTerm::Binary(atom, op, Box::new(term))
        }
    }

    fn query_term_unary<I: U8Input>(i: I) -> SimpleResult<I, QueryTerm<I::Buffer>> {
        parse!{i;
            let atom = query_atom();
            ret QueryTerm::Unary(atom)
        }
    }   
    parse!{i;
        query_term_binary() <|>
        query_term_unary()   
    }
} 

fn query_constraint<I: U8Input>(i: I) -> SimpleResult<I, QueryConstraint> {
    parse!{i;
       token_to_enum(b"=", QueryConstraint::EQ)
    }
}

fn query_op_term<I: U8Input>(i: I) -> SimpleResult<I, QueryOpTerm> {
    parse!{i;
        token_to_enum(b"&&", QueryOpTerm::AND)
    }
}

fn query_op_expression<I: U8Input>(i: I) -> SimpleResult<I, QueryOpExpression> {
    parse!{i;
        token_to_enum(b"||", QueryOpExpression::OR)
    }
}

fn query<I: U8Input>(i: I) -> SimpleResult<I, Query<I::Buffer>> {
    parse!{i;
        let expr = query_expression();
        ret @ Query<I::Buffer>, _: Query{
            tree: expr,
        }
    }
}

/// Parse a query into a result, if valid
pub fn parse_query(query_raw: &str) -> Result<Query<&[u8]>, SimpleError> {
    let parse_result = parse_only(query, query_raw.as_bytes());
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
        let and = parse_only(|i| query_op_term(i), b"&&").unwrap();
        assert_eq!(and, QueryOpTerm::AND);
        Ok(())
    }

    #[test]
    fn test_parse_tree() {
        let query = parse_only(|i| query(i), b"a=\"test\"&&b=\"what\"").unwrap();
        assert_eq!(
            query,
            Query {
                tree: QueryExpression::Unary(
                    QueryTerm::Binary(
                        QueryAtom {query_key: "a".as_bytes(), query_constraint: QueryConstraint::EQ, query_value: "test".as_bytes()},
                        QueryOpTerm::AND,
                        Box::new(QueryTerm::Unary( QueryAtom {query_key: "b".as_bytes(), query_constraint: QueryConstraint::EQ, query_value: "what".as_bytes()})),
                    )
                )
            }
        );
    }
}