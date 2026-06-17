use std::fmt::Display;

pub struct Identifier {
    pub name: String,
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Identifier({})", self.name)
    }
}

impl Identifier {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}
