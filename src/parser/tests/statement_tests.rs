use super::run_lexer_with_return_that_init_parser;

fn statement_test_helper(code: &str, expected_yaml: &str) {
    let mut simple_parser = run_lexer_with_return_that_init_parser(code);
    let got_result = simple_parser.parse_statement();
    println!("{}", serde_yaml::to_string(&got_result).unwrap());

    let expected_result = serde_yaml::from_str(expected_yaml).unwrap();

    assert_eq!(got_result, expected_result);
}

#[test]
fn if_else_simple_statement() {
    let code = r#"
if(a%2==1){
    int b = 5*5;
    printf("a is odd number");
} else a = 5;"#;

    let expected_yaml = r#"
If:
    controlling_expr:
      Equality:
        left_piece:
          Multiplicative:
            left_value:
              Identifier:
                identifier: a
            op: Mod
            right_value:
              Constant:
                Number: "2"
        equality_op: Equal
        right_piece:
          Constant:
            Number: "1"
    true_body:
      Compound:
        - Declaration:
            Declaration:
              specifiers:
                storage:
                  typedef_c: false
                  extern_c: false
                  static_c: false
                  thread_local_c: false
                  auto_c: false
                  register_c: false
                qualifiers:
                  const_q: false
                  restrict_q: false
                  volatile_q: false
                  atomic_q: false
                specifiers:
                  Basic: Int
                function:
                  inline: false
                  no_return: false
                alignment: ~
              init:
                - - base:
                      identifier: b
                    derive: Base
                  - Single:
                      Multiplicative:
                        left_value:
                          Constant:
                            Number: "5"
                        op: Mult
                        right_value:
                          Constant:
                            Number: "5"
        - Statement:
            CExpression:
              FunctionCall:
                function:
                  Identifier:
                    identifier: printf
                arguments:
                  - StringLiteral:
                      value: a is odd number
    else_body:
      CExpression:
        Assignment:
          to_assign:
            Identifier:
              identifier: a
          operator: Assign
          value:
            Constant:
              Number: "5"
  
  "#;

    statement_test_helper(code, expected_yaml)
}

#[test]
fn switch_statement_simple() {
    let code = r#"
switch(b) {
    case 0: a = 0;
    case 1: printf("b == 1");
    case 2: ;
    default: return;
}"#;

    let expected_yaml = r#"
Switch:
    controlling_expr:
      Identifier:
        identifier: b
    body:
      Compound:
        - Statement:
            SwitchCase:
              const_expr:
                internal:
                  Constant:
                    Number: "0"
              statement:
                CExpression:
                  Assignment:
                    to_assign:
                      Identifier:
                        identifier: a
                    operator: Assign
                    value:
                      Constant:
                        Number: "0"
        - Statement:
            SwitchCase:
              const_expr:
                internal:
                  Constant:
                    Number: "1"
              statement:
                CExpression:
                  FunctionCall:
                    function:
                      Identifier:
                        identifier: printf
                    arguments:
                      - StringLiteral:
                          value: b == 1
        - Statement:
            SwitchCase:
              const_expr:
                internal:
                  Constant:
                    Number: "2"
              statement: NoneExpr
        - Statement:
            SwitchDefault:
              statement:
                Return: ~
  
  "#;

    statement_test_helper(code, expected_yaml)
}

#[test]
fn nested_while_and_while_do_simple() {
    let code = r#"
while (b+5 == 7) {
    b += 1;
    int c = b;
    do {
        b += c;
        --c;
    } while (c != 0);
}"#;

    let expected_yaml = r#"
While:
    while_type: false
    controlling_expr:
      Equality:
        left_piece:
          Additive:
            left_value:
              Identifier:
                identifier: b
            op: Plus
            right_value:
              Constant:
                Number: "5"
        equality_op: Equal
        right_piece:
          Constant:
            Number: "7"
    body:
      Compound:
        - Statement:
            CExpression:
              Assignment:
                to_assign:
                  Identifier:
                    identifier: b
                operator: AssignPlus
                value:
                  Constant:
                    Number: "1"
        - Declaration:
            Declaration:
              specifiers:
                storage:
                  typedef_c: false
                  extern_c: false
                  static_c: false
                  thread_local_c: false
                  auto_c: false
                  register_c: false
                qualifiers:
                  const_q: false
                  restrict_q: false
                  volatile_q: false
                  atomic_q: false
                specifiers:
                  Basic: Int
                function:
                  inline: false
                  no_return: false
                alignment: ~
              init:
                - - base:
                      identifier: c
                    derive: Base
                  - Single:
                      Identifier:
                        identifier: b
        - Statement:
            While:
              while_type: true
              controlling_expr:
                Equality:
                  left_piece:
                    Identifier:
                      identifier: c
                  equality_op: NotEqual
                  right_piece:
                    Constant:
                      Number: "0"
              body:
                Compound:
                  - Statement:
                      CExpression:
                        Assignment:
                          to_assign:
                            Identifier:
                              identifier: b
                          operator: AssignPlus
                          value:
                            Identifier:
                              identifier: c
                  - Statement:
                      CExpression:
                        PrefixIncrement:
                          increment_type: Decrement
                          value:
                            Identifier:
                              identifier: c
  
  "#;

    statement_test_helper(code, expected_yaml)
}

#[test]
fn labeled_statement_and_jump_statements() {
    let code = r#"
{
    label0: printf("printing label0");

    goto label0;
    continue;
    break;
    return b++/5;

}"#;

    let expected_yaml = r#"
Compound:
    - Statement:
        Labeled:
          label:
            identifier: label0
          body:
            CExpression:
              FunctionCall:
                function:
                  Identifier:
                    identifier: printf
                arguments:
                  - StringLiteral:
                      value: printing label0
    - Statement:
        Goto:
          identifier: label0
    - Statement: Continue
    - Statement: Break
    - Statement:
        Return:
          Multiplicative:
            left_value:
              PostfixIncrement:
                increment_type: Increment
                value:
                  Identifier:
                    identifier: b
            op: Div
            right_value:
              Constant:
                Number: "5"
  "#;

    statement_test_helper(code, expected_yaml)
}

#[test]
fn for_loop_no_decl_simple() {
    let code = r#"
for( a = 10; a < 20; a = a + 1 ){
    printf("value of a: %d\n", a);
}
  "#;

    let expected_yaml = r#"
For:
    decl_clause: ~
    expr_clause:
      Assignment:
        to_assign:
          Identifier:
            identifier: a
        operator: Assign
        value:
          Constant:
            Number: "10"
    controlling_expr:
      Relational:
        left_piece:
          Identifier:
            identifier: a
        equality_op: Lesser
        right_piece:
          Constant:
            Number: "20"
    after_expr:
      Assignment:
        to_assign:
          Identifier:
            identifier: a
        operator: Assign
        value:
          Additive:
            left_value:
              Identifier:
                identifier: a
            op: Plus
            right_value:
              Constant:
                Number: "1"
    body:
      Compound:
        - Statement:
            CExpression:
              FunctionCall:
                function:
                  Identifier:
                    identifier: printf
                arguments:
                  - StringLiteral:
                      value: "value of a: %d\n"
                  - Identifier:
                      identifier: a

  "#;

    statement_test_helper(code, expected_yaml)
}
