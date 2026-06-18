use std::fmt::Display;

use crate::interpreter::types::{Expr, GetExpression, Ident, Identifier, Literal};

pub struct UpdateExpression<Expression: Expr + Display, UpdateIdentifier: Ident + Display> {
    collection: Identifier,
    update_key: UpdateIdentifier,
    update_value: Literal,
    statement: GetExpression<Expression>,
}

impl<Expression: Expr + Display, UpdateIdentifier: Ident + Display>
    UpdateExpression<Expression, UpdateIdentifier>
{
    pub fn new(
        collection: Identifier,
        update_key: UpdateIdentifier,
        update_value: Literal,
        statement: GetExpression<Expression>,
    ) -> Self {
        Self {
            collection,
            update_key,
            update_value,
            statement,
        }
    }
}

impl<Expression: Expr + Display, UpdateIdentifier: Ident + Display> Display
    for UpdateExpression<Expression, UpdateIdentifier>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "UPDATE {} IN {} FOR {} WRITE {})",
            self.update_key, self.collection, self.statement, self.update_value
        )
    }
}
