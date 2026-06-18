use std::fmt::Display;

use crate::interpreter::types::Ident;

use super::identifier::Identifier;

pub struct NestedIdentifier {
    pub keys: std::collections::VecDeque<Identifier>,
}

impl NestedIdentifier {
    pub fn new(keys: &std::collections::VecDeque<&str>) -> Self {
        let mut local_keys = std::collections::VecDeque::new();
        for key in keys {
            local_keys.push_back(Identifier::new(key))
        }
        Self { keys: local_keys }
    }

    pub fn add_ident(&mut self, identifier: &str) {
        self.keys.push_back(Identifier::new(identifier));
    }
}

impl Display for NestedIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let children_display: String = self
            .keys
            .iter()
            .map(|ident| ident.to_string())
            .collect::<Vec<String>>()
            .join(".");

        writeln!(f, "NestedIdentifier({})", children_display)
    }
}

impl Ident for NestedIdentifier {}
