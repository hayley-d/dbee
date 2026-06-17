use std::fmt::Display;

use crate::interpreter::types::{Literal, NestedIdentifier, operator::Operator};

pub struct BinaryExpression {
    left: NestedIdentifier,
    operator: Operator,
    right: Literal,
}

impl BinaryExpression {
    pub fn new(left: NestedIdentifier, operator: Operator, right: Literal) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}

impl Display for BinaryExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "BinaryExpression({} {} {})",
            self.left, self.operator, self.right
        )
    }
}
