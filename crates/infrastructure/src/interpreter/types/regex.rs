use std::fmt::Display;

use crate::interpreter::types::NestedIdentifier;

pub struct Regex {
    left: NestedIdentifier,
    regex: String,
}

impl Regex {
    pub fn new(left: NestedIdentifier, regex: &str) -> Self {
        Self {
            left,
            regex: regex.to_string(),
        }
    }
}

impl Display for Regex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Regex({} {})", self.left, self.regex)
    }
}
