use crate::query::constraint::*;
use crate::query::query_ast::{QueryAtom, QueryConstraint, QueryExpression, QueryTerm, QueryOpTerm, QueryOpExpression, Query};
use std::str::from_utf8;

pub fn atom_converter(atom: QueryAtom<&[u8]>) -> Box<dyn Constraint> {
    match atom.query_constraint {
        QueryConstraint::EQ => Box::new(SimpleEqualityConstraint::new(from_utf8(atom.query_key).unwrap(), from_utf8(atom.query_value).unwrap()))
    }
}

pub fn term_converter(term: QueryTerm<&[u8]>) -> Box<dyn Constraint> {
    match term {
        QueryTerm::Unary(atom) => atom_converter(atom),
        QueryTerm::Binary(atom, op, term) => 
            match op {
                QueryOpTerm::AND => Box::new(ConjunctionConstraint::new(atom_converter(atom), term_converter(*term))),
            }
    }
}

pub fn expr_converter(expr: QueryExpression<&[u8]>) -> Box<dyn Constraint> {
    match expr {
        QueryExpression::Unary(term) => term_converter(term),
        QueryExpression::Binary(term, op, expr) => 
            match op {
                QueryOpExpression::OR => Box::new(DisjunctionConstraint::new(term_converter(term), expr_converter(*expr)))
            }
    }
}

/// Factory to construct a root constraint with appropriate subconstraints, given a parse tree
pub fn constraint_factory(parse_tree: Query<&[u8]>) -> Box<dyn Constraint> {
    expr_converter(parse_tree.tree)
}