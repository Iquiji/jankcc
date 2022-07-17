use super::super::{parse_nodes::expressions::CExpression, CParser};
use super::run_lexer_with_return_that_init_parser;
use crate::parser::span::Spanned;

#[test]
fn static_assert_basic() {
    let code = r#"_Static_assert(8*8 == 64,"Math engine bad?!")"#;

    let expected_result = "
expression:
    internal:
        Equality:
            left_piece:
                Multiplicative:
                    left_value:
                        Constant:
                            Number: 8
                    op: Mult
                    right_value:
                        Constant:
                            Number: 8
            equality_op: Equal
            right_piece:
                Constant:
                    Number: 64
string_literal:
    value: Math engine bad?!
  ";

    let mut simple_parser = run_lexer_with_return_that_init_parser(code);
    let got_result = simple_parser.parse_static_assert();
    println!("{}", serde_yaml::to_string(&got_result).unwrap());

    let expected_result = serde_yaml::from_str(expected_result).unwrap();

    assert_eq!(got_result, expected_result);
}
