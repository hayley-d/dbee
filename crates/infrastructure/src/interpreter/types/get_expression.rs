use std::fmt::Display;

use crate::interpreter::types::Identifier;

pub struct GetExpression {
    collection: Identifier,
    condition: Expression,
    sort_order: Option<String>,
    limit: Option<u32>,
}

impl GetExpression {
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

impl Display for GetExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let condition = match self.condition {};

        let  = match self.condition {};

        writeln!(
            f,
            "GetExpression({}: {} {})",
            self.collection, self.condition, self.sort_order
        )
    }
}
