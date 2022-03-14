#[derive(Debug)]
pub enum Statement {
    Let {
        ident: String,
        value: Option<Expression>,
    }
}

#[derive(Debug)]
pub enum Expression {
    StringLiteral(String),
}