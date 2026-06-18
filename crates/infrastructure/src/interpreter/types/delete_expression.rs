use std::fmt::Display;

use crate::interpreter::types::{Expr, GetExpression, Ident};

pub struct DeleteExpression<Expression: Expr + Display, UpdateIdentifier: Ident + Display> {
    delete_key: UpdateIdentifier,
    statement: GetExpression<Expression>,
}

impl<Expression: Expr + Display, UpdateIdentifier: Ident + Display>
    DeleteExpression<Expression, UpdateIdentifier>
{
    pub fn new(delete_key: UpdateIdentifier, statement: GetExpression<Expression>) -> Self {
        Self {
            delete_key,
            statement,
        }
    }
}

impl<Expression: Expr + Display, UpdateIdentifier: Ident + Display> Display
    for DeleteExpression<Expression, UpdateIdentifier>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "DELETE {} FOR {})", self.delete_key, self.statement)
    }
}
