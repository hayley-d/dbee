use std::fmt::Display;

use crate::interpreter::types::{Identifier, Literal};

pub struct SetExpression {
    collection: Identifier,
    json: Literal,
}

impl SetExpression {
    pub fn new(collection: Identifier, json: Literal) -> Self {
        Self { collection, json }
    }
}

impl Display for SetExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "SetExpression({}: {})", self.collection, self.json)
    }
}
