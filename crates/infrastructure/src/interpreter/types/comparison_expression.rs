use std::fmt::Display;

use crate::interpreter::types::{Literal, NestedIdentifier, operator::Operator};

pub struct ComparisonExpression {
    left: NestedIdentifier,
    operator: Operator,
    right: Literal,
}

impl ComparisonExpression {
    pub fn new(left: NestedIdentifier, operator: Operator, right: Literal) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}

impl Display for ComparisonExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "ComparisonExpression({} {} {})",
            self.left, self.operator, self.right
        )
    }
}
