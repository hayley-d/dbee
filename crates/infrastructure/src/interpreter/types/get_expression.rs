use std::fmt::Display;

use crate::interpreter::types::Identifier;

pub trait Expr {}

pub struct GetExpression<Expression: Expr + Display> {
    collection: Identifier,
    condition: Expression,
    sort_order: Option<String>,
    limit: Option<u32>,
}

impl<Expression: Expr + Display> GetExpression<Expression> {
    pub fn new(
        collection: Identifier,
        condition: Expression,
        sort_order: Option<String>,
        limit: Option<u32>,
    ) -> Self {
        Self {
            collection,
            condition,
            sort_order,
            limit,
        }
    }
}

impl<Expression: Expr + Display> Display for GetExpression<Expression> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let order = match &self.sort_order {
            Some(order) => format!("SORT BY {order}"),
            None => "".to_string(),
        };

        let limit = match &self.limit {
            Some(limit) => format!("LIMIT {limit}"),
            None => "".to_string(),
        };

        writeln!(
            f,
            "GetExpression({}: {} {} {})",
            self.collection, self.condition, limit, order
        )
    }
}
