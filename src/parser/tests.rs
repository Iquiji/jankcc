use super::{
    parse_nodes::{expressions::CExpression, Constant, NumberLike},
    CParser,
};
use crate::{lexer::OriginalLocation, parser::tests::CExpression::DirectMemberAccess};
use crate::{
    lexer::{CToken, Lexer},
    parser::{parse_nodes::Identifier, span::Spanned},
};

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
    let mut simple_parser =
        run_lexer_with_return_that_init_parser(r#"may_struct.member0.member1.member2.member3"#);

    let start = OriginalLocation {
        file: String::new(),
        line: 0,
        collumn: 0,
    };
    let end = start.clone();

    let expected_result = Spanned::boxed_new(
        DirectMemberAccess {
            to_access: Spanned::boxed_new(
                DirectMemberAccess {
                    to_access: Spanned::boxed_new(
                        DirectMemberAccess {
                            to_access: Spanned::boxed_new(
                                DirectMemberAccess {
                                    to_access: Spanned::boxed_new(
                                        CExpression::Identifier(Identifier {
                                            string: "may_struct".to_string(),
                                        }),
                                        start.clone(),
                                        end.clone(),
                                    ),
                                    member: Identifier {
                                        string: String::from("member0"),
                                    },
                                },
                                start.clone(),
                                end.clone(),
                            ),
                            member: Identifier {
                                string: String::from("member1"),
                            },
                        },
                        start.clone(),
                        end.clone(),
                    ),
                    member: Identifier {
                        string: String::from("member2"),
                    },
                },
                start.clone(),
                end.clone(),
            ),
            member: Identifier {
                string: String::from("member3"),
            },
        },
        start,
        end,
    );

    let result = simple_parser.parse_expr_postfix();

    println!("{:#?}", result);

    assert_eq!(result, expected_result);
}
