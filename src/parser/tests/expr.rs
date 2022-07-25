use super::super::{parse_nodes::expressions::CExpression, CParser};
use super::run_lexer_with_return_that_init_parser;
use crate::parser::span::Spanned;

#[cfg(test)]
use pretty_assertions::assert_eq;

fn expresion_test_helper(
    c_expression: &str,
    expected_yaml: &str,
    expr_func: &dyn Fn(&mut CParser) -> Spanned<CExpression>,
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

    expresion_test_helper(expr, expected_result, &CParser::parse_expression);
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

    expresion_test_helper(expr, expected_result, &CParser::parse_expression);
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

    expresion_test_helper(expr, expected_result, &CParser::parse_expression);
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

    expresion_test_helper(expr, expected_result, &CParser::parse_expression);
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

    expresion_test_helper(expr, expected_result, &CParser::parse_expression);
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

    expresion_test_helper(expr, expected_result, &CParser::parse_expression);
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

    expresion_test_helper(expr, expected_result, &CParser::parse_expression);
}

#[test]
fn and_expr() {
    let expr = r#"a<b & 5 == b & u()"#;

    let expected_result = r#"
And:
    - Relational:
        left_piece:
          Identifier:
            identifier: a
        equality_op: Lesser
        right_piece:
          Identifier:
            identifier: b
    - Equality:
        left_piece:
          Constant:
            Number: "5"
        equality_op: Equal
        right_piece:
          Identifier:
            identifier: b
    - FunctionCall:
        function:
          Identifier:
            identifier: u
        arguments: []
"#;

    expresion_test_helper(expr, expected_result, &CParser::parse_expression);
}

#[test]
fn simple_assign_expr() {
    let expr = r#"me.inner->age += 1"#;

    let expected_result = r#"
Assignment:
  to_assign:
    IndirectMemberAccess:
      to_access:
        DirectMemberAccess:
          to_access:
            Identifier:
              identifier: me
          member:
            identifier: inner
      member:
        identifier: age
  operator: AssignPlus
  value:
    Constant:
      Number: "1"
"#;

    expresion_test_helper(expr, expected_result, &CParser::parse_expression);
}

#[test]
fn unified_expression_test_simple() {
    let expr = r#"ideas[8+(offset/64)] += function((42*1.005) % modulator,7,beta)"#;

    let expected_result = r#"
Assignment:
  to_assign:
    ArraySubscription:
      array:
        Identifier:
          identifier: ideas
      index:
        Additive:
          left_value:
            Constant:
              Number: "8"
          op: Plus
          right_value:
            Paranthesised:
              Multiplicative:
                left_value:
                  Identifier:
                    identifier: offset
                op: Div
                right_value:
                  Constant:
                    Number: "64"
  operator: AssignPlus
  value:
    FunctionCall:
      function:
        Identifier:
          identifier: function
      arguments:
        - Multiplicative:
            left_value:
              Paranthesised:
                Multiplicative:
                  left_value:
                    Constant:
                      Number: "42"
                  op: Mult
                  right_value:
                    Constant:
                      Number: "1.005"
            op: Mod
            right_value:
              Identifier:
                identifier: modulator
        - Constant:
            Number: "7"
        - Identifier:
            identifier: beta
"#;

    expresion_test_helper(expr, expected_result, &CParser::parse_expression);
}

#[test]
fn unified_expression_test_multi_assign() {
    let expr = r#"ideas[8+(offset/64)] = war = --b + a / 8"#;

    let expected_result = r#"
Assignment:
  to_assign:
    ArraySubscription:
      array:
        Identifier:
          identifier: ideas
      index:
        Additive:
          left_value:
            Constant:
              Number: "8"
          op: Plus
          right_value:
            Paranthesised:
              Multiplicative:
                left_value:
                  Identifier:
                    identifier: offset
                op: Div
                right_value:
                  Constant:
                    Number: "64"
  operator: Assign
  value:
    Assignment:
      to_assign:
        Identifier:
          identifier: war
      operator: Assign
      value:
        Additive:
          left_value:
            PrefixIncrement:
              increment_type: Decrement
              value:
                Identifier:
                  identifier: b
          op: Plus
          right_value:
            Multiplicative:
              left_value:
                Identifier:
                  identifier: a
              op: Div
              right_value:
                Constant:
                  Number: "8"
"#;

    expresion_test_helper(expr, expected_result, &CParser::parse_expression);
}

#[test]
#[ignore = "not yet :)"]
fn unified_expression_test_not_unary() {
    let expr = r#"a + b = c"#;
    expresion_test_helper(
        expr,
        "Constant:
    Number: 8",
        &CParser::parse_expression,
    );
}

#[test]
fn type_cast_simple() {
    let expr = r#"(double) sum / count"#;

    let expected_result = r#"
Multiplicative:
  left_value:
    Cast:
      type_name:
        base:
          qualifiers:
            const_q: false
            restrict_q: false
            volatile_q: false
            atomic_q: false
          specifier:
            Basic: Double
        declarator: Base
      value:
        Identifier:
          identifier: sum
  op: Div
  right_value:
    Identifier:
      identifier: count

"#;

    expresion_test_helper(expr, expected_result, &CParser::parse_expression);
}

#[test]
fn type_cast_simple_explicit_parenthesis() {
    let expr = r#"((char*)heap) + offset"#;

    let expected_result = r#"
Additive:
  left_value:
    Paranthesised:
      Cast:
        type_name:
          base:
            qualifiers:
              const_q: false
              restrict_q: false
              volatile_q: false
              atomic_q: false
            specifier:
              Basic: Char
          declarator:
            Pointer:
              qualifiers:
                const_q: false
                restrict_q: false
                volatile_q: false
                atomic_q: false
              to: Base
        value:
          Identifier:
            identifier: heap
  op: Plus
  right_value:
    Identifier:
      identifier: offset

"#;

    expresion_test_helper(expr, expected_result, &CParser::parse_expression);
}

#[test]
fn type_initializer_double_in_function_call() {
    let expr = r#"\
drawline((struct point){.x=1, .y=1},
  (struct point){.x=3, .y=4});"#;

    let expected_result = r#"
FunctionCall:
  function:
    Identifier:
      identifier: drawline
  arguments:
    - TypeInitializer:
        type_name:
          base:
            qualifiers:
              const_q: false
              restrict_q: false
              volatile_q: false
              atomic_q: false
            specifier:
              StructOrUnion:
                struct_type: Struct
                ident:
                  identifier: point
                declarations: []
          declarator: Base
        initializer_list:
          Compound:
            - - - Member:
                    identifier: x
              - Single:
                  Constant:
                    Number: "1"
            - - - Member:
                    identifier: y
              - Single:
                  Constant:
                    Number: "1"
    - TypeInitializer:
        type_name:
          base:
            qualifiers:
              const_q: false
              restrict_q: false
              volatile_q: false
              atomic_q: false
            specifier:
              StructOrUnion:
                struct_type: Struct
                ident:
                  identifier: point
                declarations: []
          declarator: Base
        initializer_list:
          Compound:
            - - - Member:
                    identifier: x
              - Single:
                  Constant:
                    Number: "3"
            - - - Member:
                    identifier: y
              - Single:
                  Constant:
                    Number: "4"

"#;

    expresion_test_helper(expr, expected_result, &CParser::parse_expression);
}

#[test]
fn sizeof_expression_0() {
    let expr = r#"alloc(sizeof *dp)"#;

    let expected_result = r#"
FunctionCall:
  function:
    Identifier:
      identifier: alloc
  arguments:
    - SizeOf:
        value:
          Unary:
            unary_op: DEREF
            value:
              Identifier:
                identifier: dp
"#;

    expresion_test_helper(expr, expected_result, &CParser::parse_expression);
}

#[test]
fn sizeof_expression_1() {
    let expr = r#"sizeof array / sizeof array[0]"#;

    let expected_result = r#"
Multiplicative:
  left_value:
    SizeOf:
      value:
        Identifier:
          identifier: array
  op: Div
  right_value:
    SizeOf:
      value:
        ArraySubscription:
          array:
            Identifier:
              identifier: array
          index:
            Constant:
              Number: "0"

"#;

    expresion_test_helper(expr, expected_result, &CParser::parse_expression);
}
