use super::{
    parse_nodes::{expressions::CExpression, Constant, NumberLike},
    CParser,
};
use crate::{
    lexer::Lexer,
    parser::{parse_nodes::Identifier, span::Spanned},
};
use crate::{lexer::OriginalLocation, parser::tests::CExpression::DirectMemberAccess};

#[cfg(test)]
use pretty_assertions::assert_eq;

fn run_lexer_with_return_that_init_parser(code: &str) -> CParser {
    let lexed = Lexer::new().string_to_token_arr(code.to_string());

    CParser::new(lexed)
}

fn expresion_test_helper(
    c_expression: &str,
    expected_yaml: &str,
    expr_func: &dyn Fn(&mut CParser) -> Box<Spanned<CExpression>>,
) {
    let mut simple_parser = run_lexer_with_return_that_init_parser(c_expression);
    let expected_result = serde_yaml::from_str(expected_yaml).unwrap();
    let got_result = expr_func(&mut simple_parser);

    println!("{}", serde_yaml::to_string(&got_result).unwrap());

    assert_eq!(got_result, expected_result);
}

#[test]
fn primary_expression_constant() {
    let expr = r#"666.010101"#;

    let expected_result = "
Constant:
    Number:
        666.010101";

    expresion_test_helper(expr, expected_result, &CParser::parse_expr_primary);
}

#[test]
fn postfix_expression_direct_access_simple() {
    let expr = r#"may_struct.member0.member1.member2.member3"#;

    let expected_result = "
DirectMemberAccess:
    member:
      identifier: member3
    to_access:
      DirectMemberAccess:
        member:
          identifier: member2
        to_access:
          DirectMemberAccess:
            member:
              identifier: member1
            to_access:
              DirectMemberAccess:
                member:
                  identifier: member0
                to_access:
                  Identifier:
                    identifier: may_struct
    ";

    expresion_test_helper(expr, expected_result, &CParser::parse_expr_postfix);
}

#[test]
fn postfix_expression_direct_access_simple_with_increment() {
    let expr = r#"may_struct.member0.member1.member2.member3++"#;

    let expected_result = "
PostfixIncrement:
    increment_type: Increment
    value:
        DirectMemberAccess:
            member:
                identifier: member3
            to_access:
                DirectMemberAccess:
                    member:
                        identifier: member2
                    to_access:
                        DirectMemberAccess:
                            member:
                                identifier: member1
                            to_access:
                                DirectMemberAccess:
                                    member:
                                        identifier: member0
                                    to_access:
                                        Identifier:
                                            identifier: may_struct
    ";

    expresion_test_helper(expr, expected_result, &CParser::parse_expr_postfix);
}

#[test]
fn empty_func_call_expr() {
    let expr = r#"may_struct.member0()++"#;

    let expected_result = "
PostfixIncrement:
    increment_type: Increment
    value:
        FunctionCall:
            function:
                DirectMemberAccess:
                    member:
                        identifier: member0
                    to_access:
                        Identifier:
                            identifier: may_struct
            arguments: []
    ";

    expresion_test_helper(expr, expected_result, &CParser::parse_expr_postfix);
}
