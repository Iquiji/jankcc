use super::{
    parse_nodes::{expressions::CExpression, Constant, NumberLike},
    CParser,
};
use crate::lexer::{CToken, Lexer};

fn run_lexer_with_return_that_init_parser(code: &str) -> CParser {
    let lexed = Lexer::new().string_to_token_arr(code.to_string());

    CParser::new(lexed)
}

#[test]
fn primary_expression_constant() {
    let mut simple_parser = run_lexer_with_return_that_init_parser(r#"666.010101"#);

    let expected_result = CExpression::Constant(Constant::Number(NumberLike {
        from: "666.010101".to_string(),
    }));

    let result = simple_parser.parse_expr_primary();

    assert_eq!(*(*result), expected_result);
}

#[test]
fn postfix_expression_direct_access_simple() {
    let mut simple_parser = run_lexer_with_return_that_init_parser(r#"may_struct.member0.member1.member2.member3"#);

    let expected_result = CExpression::Constant(Constant::Number(NumberLike {
        from: "666.010101".to_string(),
    }));

    let result = simple_parser.parse_expr_postfix();

    assert_eq!(*(*result), expected_result);
}