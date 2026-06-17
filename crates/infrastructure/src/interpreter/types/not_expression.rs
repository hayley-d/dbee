use std::fmt::Display;

use crate::interpreter::types::ComparisonExpression;

pub struct NotExpression {
    expression: ComparisonExpression,
}

impl NotExpression {
    pub fn new(expression: ComparisonExpression) -> Self {
        Self { expression }
    }
}

impl Display for NotExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "NotExpression({})", self.expression)
    }
}
