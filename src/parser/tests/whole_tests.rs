use crate::parser::{span::Spanned, parse_nodes::ExternalDeclaration};

use super::run_lexer_with_return_that_init_parser;

fn parse_test_helper(code: &str, expected_yaml: &str) {
    let mut simple_parser = run_lexer_with_return_that_init_parser(code);
    let got_result = simple_parser.parse();
    println!("{}", serde_yaml::to_string(&got_result).unwrap());

    let expected_result: Vec<Spanned<ExternalDeclaration>> = serde_yaml::from_str(expected_yaml).unwrap();

    assert_eq!(got_result, expected_result);
}

#[test]
fn simple_hello_world_no_include(){
    // remember: no comments in here :)
    let code = r#"
int main() {
    printf("hello world\n");
    if (8 == 9){
        return 1;
    } else {
        return 0;
    }
}"#;

    let expected_yaml = r#"
- FunctionDefinition:
    function_specifiers:
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
    declarator:
      base:
        identifier: main
      derive:
        FunctionType:
          parameter_type_list:
            parameter_list: []
            ellipsis: false
          to: Base
    declarations: []
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
                      value: "hello world\\n"
        - Statement:
            If:
              controlling_expr:
                Equality:
                  left_piece:
                    Constant:
                      Number: "8"
                  equality_op: Equal
                  right_piece:
                    Constant:
                      Number: "9"
              true_body:
                Compound:
                  - Statement:
                      Return:
                        Constant:
                          Number: "1"
              else_body:
                Compound:
                  - Statement:
                      Return:
                        Constant:
                          Number: "0"
    
"#;

    parse_test_helper(code, expected_yaml);
}

