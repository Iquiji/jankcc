use crate::parser::{
    span::Spanned, tests::run_lexer_with_return_that_init_parser, types::CTypeName, CParser,
};

fn type_name_test_helper(c_expression: &str, expected_yaml: &str) {
    let mut simple_parser = run_lexer_with_return_that_init_parser(c_expression);

    let got_result = simple_parser.parse_type_name();
    println!("{}", serde_yaml::to_string(&got_result).unwrap());

    let expected_result = serde_yaml::from_str(expected_yaml).unwrap();

    assert_eq!(got_result, expected_result);
}

#[test]
fn simple_signed_short_type_name() {
    let expr = r#"signed short"#;

    let expected_result = "
base:
    qualifiers:
      const_q: false
      restrict_q: false
      volatile_q: false
      atomic_q: false
    specifier:
      Basic: Short
declarator: Base";

    type_name_test_helper(expr, expected_result);
}

#[test]
fn type_name_6_7_7_3_a() {
    let expr = r#"int"#;

    let expected_result = "
base:
    qualifiers:
      const_q: false
      restrict_q: false
      volatile_q: false
      atomic_q: false
    specifier:
      Basic: Int
declarator: Base";

    type_name_test_helper(expr, expected_result);
}

#[test]
fn type_name_6_7_7_3_b() {
    let expr = r#"int *"#;

    let expected_result = "
base:
    qualifiers:
        const_q: false
        restrict_q: false
        volatile_q: false
        atomic_q: false
    specifier:
        Basic: Int
declarator:
    Pointer:
        qualifiers:
            const_q: false
            restrict_q: false
            volatile_q: false
            atomic_q: false
        to: Base
  ";

    type_name_test_helper(expr, expected_result);
}

#[test]
fn type_name_6_7_7_3_c() {
    let expr = r#"int *[3]"#;

    let expected_result = "
base:
    qualifiers:
        const_q: false
        restrict_q: false
        volatile_q: false
        atomic_q: false
    specifier:
        Basic: Int
declarator:
    Array:
        qualifiers:
            const_q: false
            restrict_q: false
            volatile_q: false
            atomic_q: false
        is_static: false
        size_expr:
            Constant:
                Number: 3
        vla: false
        to:
            Pointer:
                qualifiers:
                    const_q: false
                    restrict_q: false
                    volatile_q: false
                    atomic_q: false
                to: Base
  ";

    type_name_test_helper(expr, expected_result);
}

#[test]
fn type_name_6_7_7_3_d() {
    let expr = r#"int (*)[3]"#;

    let expected_result = "
base:
    qualifiers:
      const_q: false
      restrict_q: false
      volatile_q: false
      atomic_q: false
    specifier:
      Basic: Int
declarator:
    Pointer:
      qualifiers:
        const_q: false
        restrict_q: false
        volatile_q: false
        atomic_q: false
      to:
        Array:
            qualifiers:
                const_q: false
                restrict_q: false
                volatile_q: false
                atomic_q: false
            is_static: false
            vla: false
            size_expr:
                Constant:
                    Number: 3
            to: Base";

    type_name_test_helper(expr, expected_result);
}

#[test]
fn type_name_6_7_7_3_e() {
    let expr = r#"int (*)[*]"#;

    let expected_result = "
base:
    qualifiers:
      const_q: false
      restrict_q: false
      volatile_q: false
      atomic_q: false
    specifier:
      Basic: Int
declarator:
    Pointer:
      qualifiers:
        const_q: false
        restrict_q: false
        volatile_q: false
        atomic_q: false
      to:
        Array:
          qualifiers:
            const_q: false
            restrict_q: false
            volatile_q: false
            atomic_q: false
          is_static: false
          size_expr: ~
          vla: true
          to: Base
  ";

    type_name_test_helper(expr, expected_result);
}

#[test]
fn type_name_6_7_7_3_f() {
    let expr = r#"int *()"#;

    let expected_result = "
base:
    qualifiers:
      const_q: false
      restrict_q: false
      volatile_q: false
      atomic_q: false
    specifier:
      Basic: Int
declarator:
    FunctionType:
      parameter_type_list:
        parameter_list: []
        ellipsis: false
      to:
        Pointer:
          qualifiers:
            const_q: false
            restrict_q: false
            volatile_q: false
            atomic_q: false
          to: Base
  ";

    type_name_test_helper(expr, expected_result);
}

#[test]
fn type_name_6_7_7_3_g() {
    let expr = r#"int (*)(void)"#;

    let expected_result = "
base:
    qualifiers:
      const_q: false
      restrict_q: false
      volatile_q: false
      atomic_q: false
    specifier:
      Basic: Short
declarator: Base";

    type_name_test_helper(expr, expected_result);
}

#[test]
fn type_name_6_7_7_3_h() {
    let expr = r#"int (*const []) (unsigned int, ...)"#;

    let expected_result = "
base:
    qualifiers:
      const_q: false
      restrict_q: false
      volatile_q: false
      atomic_q: false
    specifier:
      Basic: Short
declarator: Base";

    type_name_test_helper(expr, expected_result);
}
