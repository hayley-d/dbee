use std::fmt::Display;

use crate::interpreter::types::{Ident, Literal, get_expression::Expr, operator::Operator};

pub struct ComparisonExpression<Identifier: Ident + Display> {
    left: Identifier,
    operator: std::collections::VecDeque<Operator>,
    right: Literal,
}

impl<Identifier: Ident + Display> ComparisonExpression<Identifier> {
    pub fn new(
        left: Identifier,
        operator: std::collections::VecDeque<Operator>,
        right: Literal,
    ) -> Self {
        todo!("Update return type to a result with custom implemented results");
        todo!("Ensure a user cannot pass Operator::And and Operator::Or into this expression");
        Self {
            left,
            operator,
            right,
        }
    }
}

impl<Identifier: Ident + Display> Display for ComparisonExpression<Identifier> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let operators = self
            .operator
            .iter()
            .map(|op| op.to_string())
            .collect::<Vec<String>>()
            .join("");

        writeln!(
            f,
            "ComparisonExpression({} {} {})",
            self.left, operators, self.right
        )
    }
}

impl<Identifier: Ident + Display> Expr for ComparisonExpression<Identifier> {}
