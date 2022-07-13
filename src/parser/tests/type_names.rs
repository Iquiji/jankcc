use crate::parser::{
    span::Spanned, tests::run_lexer_with_return_that_init_parser, types::CType, CParser,
};

fn type_name_test_helper(c_expression: &str, expected_yaml: &str) {
    let mut simple_parser = run_lexer_with_return_that_init_parser(c_expression);

    let got_result = simple_parser.parse_ctypename();
    println!("{}", serde_yaml::to_string(&got_result).unwrap());

    let expected_result = serde_yaml::from_str(expected_yaml).unwrap();

    assert_eq!(got_result, expected_result);
}

#[test]
fn simple_signed_short_type_name() {
    let expr = r#"signed short"#;

    let expected_result = "
Basic:
    qualifiers:
        const_q: false
        restrict_q: false
        volatile_q: false
        atomic_q: false
    specifier:
        Basic: Short";

    type_name_test_helper(expr, expected_result);
}
