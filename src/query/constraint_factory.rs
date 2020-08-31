use crate::query::constraint::*;
use crate::query::query_ast::{QueryAtom, QueryConstraint, QueryOp, QueryTree};
use std::str::from_utf8;

/// Builder for an equality constraint given an atom
fn equality_constraint_from_atom(atom: QueryAtom<&[u8]>) -> SimpleEqualityConstraint {
    SimpleEqualityConstraint::new(from_utf8(atom.query_key).unwrap(), from_utf8(atom.query_value).unwrap())
}

/// Factory to construct a root constraint with appropriate subconstraints, given a parse tree
pub fn constraint_factory(parse_tree: QueryTree<&[u8]>) -> Box<dyn Constraint> {
    let atom_constraint = Box::new(match parse_tree.atom.query_constraint {
        QueryConstraint::EQ => equality_constraint_from_atom(parse_tree.atom),
    });
    match parse_tree.op {
        None => atom_constraint,
        Some(op) => 
            match op {
                QueryOp::AND => Box::new(ConjunctionConstraint::new(atom_constraint, constraint_factory(*parse_tree.tree.unwrap()))),
            }
    }
}