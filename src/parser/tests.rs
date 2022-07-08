use super::{parse_nodes::expressions::CExpression, CParser};
use crate::{lexer::Lexer, parser::span::Spanned};

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

    expresion_test_helper(expr, expected_result, &CParser::parse_expr_and);
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

    expresion_test_helper(expr, expected_result, &CParser::parse_expr_and);
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

    expresion_test_helper(expr, expected_result, &CParser::parse_expr_and);
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

    expresion_test_helper(expr, expected_result, &CParser::parse_expr_and);
}

#[test]
fn simple_mult() {
    let expr = r#"func()++ * --!x"#;

    let expected_result = "
Multiplicative:
    left_value:
      PostfixIncrement:
        increment_type: Increment
        value:
          FunctionCall:
            function:
              Identifier:
                identifier: func
            arguments: []
    op: Mult
    right_value:
      PrefixIncrement:
        increment_type: Decrement
        value:
          Unary:
            unary_op: BOOLEANINVERT
            value:
              Identifier:
                identifier: x
    ";

    expresion_test_helper(expr, expected_result, &CParser::parse_expr_and);
}

#[test]
fn simple_add() {
    let expr = r#"42 + func()++ * --!x - -var"#;

    let expected_result = "
Additive:
    left_value:
      Additive:
        left_value:
          Constant:
            Number: 42
        op: Plus
        right_value:
          Multiplicative:
            left_value:
              PostfixIncrement:
                increment_type: Increment
                value:
                  FunctionCall:
                    function:
                      Identifier:
                        identifier: func
                    arguments: []
            op: Mult
            right_value:
              PrefixIncrement:
                increment_type: Decrement
                value:
                  Unary:
                    unary_op: BOOLEANINVERT
                    value:
                      Identifier:
                        identifier: x
    op: Minus
    right_value:
      Unary:
        unary_op: NEGATIVE
        value:
          Identifier:
            identifier: var
    ";

    expresion_test_helper(expr, expected_result, &CParser::parse_expr_and);
}

#[test]
fn simple_relational_equality() {
    let expr = r#"a<b == c<d"#;

    let expected_result = "
Equality:
    left_piece:
      Relational:
        left_piece:
          Identifier:
            identifier: a
        equality_op: Lesser
        right_piece:
          Identifier:
            identifier: b
    equality_op: Equal
    right_piece:
      Relational:
        left_piece:
          Identifier:
            identifier: c
        equality_op: Lesser
        right_piece:
          Identifier:
            identifier: d
    ";

    expresion_test_helper(expr, expected_result, &CParser::parse_expr_and);
}
