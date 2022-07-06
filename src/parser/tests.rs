use crate::lexer::{CToken, Lexer};
use super::{CParser, parse_nodes::{expressions::CExpression, NumberLike, Constant}};

fn run_lexer_with_return_that_init_parser(code: &str) -> CParser{
    let lexed = Lexer::new().string_to_token_arr(code.to_string());
    
    CParser::new(lexed)
}

#[test]
fn primary_expression_constant() {
    let mut simple_parser = run_lexer_with_return_that_init_parser(r#"666.010101"#);
    
    let expected_result = CExpression::Constant(Constant::Number(NumberLike{ from: "666.010101".to_string()}));

    let result = simple_parser.parse_expr_primary();

    assert_eq!(*(*result),expected_result);
}