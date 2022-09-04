use super::*;

#[test]
fn intern_basic_declaration() {
    let code = r#"const int ptr_to_constant;"#;

    let parsed = *parser_parse_specific(code, CParser::parse_declaration).inner;
    let mut env_controller = make_environment_controller();
    match parsed {
        Declaration::StaticAssertDeclaration(_) => unreachable!(),
        Declaration::Declaration { specifiers, init } => {
            // :/
            let got_result = env_controller
                .extract_pretty_type_from_declaration_specifiers_and_derived_declarator(
                    specifiers,
                    init[0].0.derive.clone(),
                );

            println!("{}", serde_yaml::to_string(&got_result).unwrap());

            let expected_yaml = r#"
inner_type:
  Int:
    is_const: true
    is_volatile: false
    signed: true
    size: 4

            "#;
            let expected_result = serde_yaml::from_str(expected_yaml).unwrap();

            assert_eq!(got_result, expected_result);
        }
    }
}

#[test]
fn intern_pointer_declaration() {
    let code = r#"const int *const const_ptr_to_constant;"#;

    let parsed = *parser_parse_specific(code, CParser::parse_declaration).inner;
    let mut env_controller = make_environment_controller();
    match parsed {
        Declaration::StaticAssertDeclaration(_) => unreachable!(),
        Declaration::Declaration { specifiers, init } => {
            // :/
            let got_result = env_controller
                .extract_pretty_type_from_declaration_specifiers_and_derived_declarator(
                    specifiers,
                    init[0].0.derive.clone(),
                );

            println!("{}", serde_yaml::to_string(&got_result).unwrap());

            let expected_yaml = r#"
inner_type:
    Pointer:
        is_const: true
        is_volatile: false
        to:
            Int:
                is_const: true
                is_volatile: false
                signed: true
                size: 4

            "#;
            let expected_result = serde_yaml::from_str(expected_yaml).unwrap();

            assert_eq!(got_result, expected_result);
        }
    }
}

#[test]
fn intern_float_array() {
    let code = r#"float *afp[17];"#;

    let parsed = *parser_parse_specific(code, CParser::parse_declaration).inner;
    let mut env_controller = make_environment_controller();
    match parsed {
        Declaration::StaticAssertDeclaration(_) => unreachable!(),
        Declaration::Declaration { specifiers, init } => {
            // :/
            let got_result = env_controller
                .extract_pretty_type_from_declaration_specifiers_and_derived_declarator(
                    specifiers,
                    init[0].0.derive.clone(),
                );

            println!("{}", serde_yaml::to_string(&got_result).unwrap());

            let expected_yaml = r#"
inner_type:
    Array:
        is_const: false
        is_volatile: false
        arr_size: 17
        to:
            Pointer:
                is_const: false
                is_volatile: false
                to:
                    Float:
                        is_const: false
                        is_volatile: false
                        size: 4
            "#;
            let expected_result = serde_yaml::from_str(expected_yaml).unwrap();

            assert_eq!(got_result, expected_result);
        }
    }
}

#[test]
fn intern_simple_struct() {
    let code = r#"struct { int __val[2]; } __fsid_t;"#;

    let parsed = *parser_parse_specific(code, CParser::parse_declaration).inner;
    let mut env_controller = make_environment_controller();
    match parsed {
        Declaration::StaticAssertDeclaration(_) => unreachable!(),
        Declaration::Declaration { specifiers, init } => {
            // :/
            let got_result = env_controller
                .extract_pretty_type_from_declaration_specifiers_and_derived_declarator(
                    specifiers,
                    init[0].0.derive.clone(),
                );

            println!("{}", serde_yaml::to_string(&got_result).unwrap());

            let expected_yaml = r#"
          inner_type:
            Struct:
              is_const: false
              is_volatile: false
              tag: ~
              members:
                - ident: __val
                  member_type:
                    Array:
                      is_const: false
                      is_volatile: false
                      arr_size: 2
                      to:
                        Int:
                          is_const: false
                          is_volatile: false
                          signed: true
                          size: 4
          
            "#;
            let expected_result = serde_yaml::from_str(expected_yaml).unwrap();

            assert_eq!(got_result, expected_result);
        }
    }
}
