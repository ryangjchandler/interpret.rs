mod ast;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(parser);

fn main() {
    let ast = parser::ProgramParser::new()
        .parse("let foo")
        .unwrap();

    dbg!(ast);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;

    fn expect_ast(code: &str, expected: Vec<Statement>) {
        assert_eq!(
            parser::ProgramParser::new().parse(code).unwrap(),
            expected
        );
    }

    #[test]
    fn it_can_parse_a_simple_let_statement() {
        expect_ast("let foo", vec![
            Statement::Let {
                name: String::from("foo"),
                value: None,
            }
        ]);
    }
}