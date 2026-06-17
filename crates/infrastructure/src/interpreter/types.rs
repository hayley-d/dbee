use std::fmt::Display;

pub struct Identifier {
    pub name: String,
}

pub struct NestedIdentifier {
    pub keys: std::collections::VecDeque<Identifier>,
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Identifier {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
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
        todo!()
    }
}
