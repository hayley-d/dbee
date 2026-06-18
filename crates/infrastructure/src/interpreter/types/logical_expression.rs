use std::fmt::Display;

use crate::interpreter::types::{ComparisonExpression, Expr, Ident, Operator};

pub struct LogicalExpression<IdentifierLeft: Ident + Display, IdentifierRigt: Ident + Display> {
    left: ComparisonExpression<IdentifierLeft>,
    operator: Operator,
    right: ComparisonExpression<IdentifierRigt>,
}

impl<IdentifierLeft: Ident + Display, IdentifierRigt: Ident + Display>
    LogicalExpression<IdentifierLeft, IdentifierRigt>
{
    pub fn new(
        left: ComparisonExpression<IdentifierLeft>,
        operator: Operator,
        right: ComparisonExpression<IdentifierRigt>,
    ) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}

impl<IdentifierLeft: Ident + Display, IdentifierRigt: Ident + Display> Display
    for LogicalExpression<IdentifierLeft, IdentifierRigt>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "LogicalExpression({} {} {})",
            self.left, self.operator, self.right
        )
    }
}

impl<IdentifierLeft: Ident + Display, IdentifierRigt: Ident + Display> Expr
    for LogicalExpression<IdentifierLeft, IdentifierRigt>
{
}
