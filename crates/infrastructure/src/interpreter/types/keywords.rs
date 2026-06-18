pub enum Keyword {
    Create,
    Select,
    From,
    Set,
    To,
    As,
    Where,
    GroupBy,
    OrderBy,
    Limit,
}

impl Keyword {
    pub fn is_keyword(word: &str) -> Option<Keyword> {
        match word.to_lowercase().as_str() {
            "create" => Some(Keyword::Create),
            "select" => Some(Keyword::Select),
            "from" => Some(Keyword::From),
            "set" => Some(Keyword::Set),
            "to" => Some(Keyword::To),
            "as" => Some(Keyword::As),
            "where" => Some(Keyword::Where),
            "group by" => Some(Keyword::GroupBy),
            "order by" => Some(Keyword::OrderBy),
            "limit" => Some(Keyword::Limit),
            _ => None,
        }
    }
}
