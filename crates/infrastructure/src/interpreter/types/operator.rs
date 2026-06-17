use std::fmt::Display;

pub enum Operator {
    Equal,
    Bang,
    GreaterThan,
    LessThan,
    Divide,
    Star,
    Plus,
    Minus,
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Equal => writeln!(f, "="),
            Operator::Bang => writeln!(f, "!"),
            Operator::GreaterThan => writeln!(f, ">"),
            Operator::LessThan => writeln!(f, "<"),
            Operator::Divide => writeln!(f, "/"),
            Operator::Star => writeln!(f, "*"),
            Operator::Plus => writeln!(f, "+"),
            Operator::Minus => writeln!(f, "-"),
        }
    }
}
