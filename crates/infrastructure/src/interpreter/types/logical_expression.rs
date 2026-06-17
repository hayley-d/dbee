use std::fmt::Display;

use crate::interpreter::types::{ComparisonExpression, Operator};

pub struct LogicalExpression {
    left: ComparisonExpression,
    operator: Operator,
    right: ComparisonExpression,
}

impl LogicalExpression {
    pub fn new(
        left: ComparisonExpression,
        operator: Operator,
        right: ComparisonExpression,
    ) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}

impl Display for LogicalExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "LogicalExpression({} {} {})",
            self.left, self.operator, self.right
        )
    }
}
