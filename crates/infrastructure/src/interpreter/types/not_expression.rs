use std::fmt::Display;

use crate::interpreter::types::{ComparisonExpression, Ident};

pub struct NotExpression<Identifier: Ident + Display> {
    expression: ComparisonExpression<Identifier>,
}

impl<Identifier: Ident + Display> NotExpression<Identifier> {
    pub fn new(expression: ComparisonExpression<Identifier>) -> Self {
        Self { expression }
    }
}

impl<Identifier: Ident + Display> Display for NotExpression<Identifier> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "NotExpression({})", self.expression)
    }
}
