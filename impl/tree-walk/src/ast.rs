#[derive(Debug, PartialEq)]
pub enum Statement {
    Let {
        name: String,
        value: Option<Expression>,
    }
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    StringLiteral(String),
}