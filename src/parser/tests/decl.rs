use super::super::{parse_nodes::expressions::CExpression, CParser};
use super::run_lexer_with_return_that_init_parser;
use crate::parser::span::Spanned;

#[test]
fn static_assert_basic() {
    let code = r#"_Static_assert(8*8 == 64,"Math engine bad?!");"#;

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

#[test]
fn declaration_basic_0() {
    let code = r#"extern const volatile int real_time_clock;"#;

    let expected_result = "
Declaration:
    specifiers:
      storage:
        typedef_c: false
        extern_c: true
        static_c: false
        thread_local_c: false
        auto_c: false
        register_c: false
      qualifiers:
        const_q: true
        restrict_q: false
        volatile_q: true
        atomic_q: false
      specifiers:
        Basic: Int
      function:
        inline: false
        no_return: false
      alignment: ~
    init:
      - - base:
            identifier: real_time_clock
          derive: Base
        - ~
  
    
    ";

    let mut simple_parser = run_lexer_with_return_that_init_parser(code);
    let got_result = simple_parser.parse_declaration();
    println!("{}", serde_yaml::to_string(&got_result).unwrap());

    let expected_result = serde_yaml::from_str(expected_result).unwrap();

    assert_eq!(got_result, expected_result);
}

#[test]
fn declaration_basic_1() {
    let code = r#"const int *ptr_to_constant;"#;

    let expected_result = "
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
        const_q: true
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
            identifier: ptr_to_constant
          derive:
            Pointer:
              qualifiers:
                const_q: false
                restrict_q: false
                volatile_q: false
                atomic_q: false
              to: Base
        - ~
  
    ";

    let mut simple_parser = run_lexer_with_return_that_init_parser(code);
    let got_result = simple_parser.parse_declaration();
    println!("{}", serde_yaml::to_string(&got_result).unwrap());

    let expected_result = serde_yaml::from_str(expected_result).unwrap();

    assert_eq!(got_result, expected_result);
}

#[test]
fn declaration_basic_2() {
    let code = r#"int *const constant_ptr;"#;

    let expected_result = "
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
            identifier: constant_ptr
          derive:
            Pointer:
              qualifiers:
                const_q: true
                restrict_q: false
                volatile_q: false
                atomic_q: false
              to: Base
        - ~
  
    ";

    let mut simple_parser = run_lexer_with_return_that_init_parser(code);
    let got_result = simple_parser.parse_declaration();
    println!("{}", serde_yaml::to_string(&got_result).unwrap());

    let expected_result = serde_yaml::from_str(expected_result).unwrap();

    assert_eq!(got_result, expected_result);
}

#[test]
fn declaration_basic_3() {
    let code = r#"float fa[11], *afp[17];"#;

    let expected_result = "
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
        Basic: Float
      function:
        inline: false
        no_return: false
      alignment: ~
    init:
      - - base:
            identifier: fa
          derive:
            Array:
              qualifiers:
                const_q: false
                restrict_q: false
                volatile_q: false
                atomic_q: false
              is_static: false
              size_expr:
                Constant:
                  Number: 11
              vla: false
              to: Base
        - ~
      - - base:
            identifier: afp
          derive:
            Array:
              qualifiers:
                const_q: false
                restrict_q: false
                volatile_q: false
                atomic_q: false
              is_static: false
              size_expr:
                Constant:
                  Number: 17
              vla: false
              to:
                Pointer:
                  qualifiers:
                    const_q: false
                    restrict_q: false
                    volatile_q: false
                    atomic_q: false
                  to: Base
        - ~
  
    ";

    let mut simple_parser = run_lexer_with_return_that_init_parser(code);
    let got_result = simple_parser.parse_declaration();
    println!("{}", serde_yaml::to_string(&got_result).unwrap());

    let expected_result = serde_yaml::from_str(expected_result).unwrap();

    assert_eq!(got_result, expected_result);
}

#[test]
fn declaration_advanced_0() {
    let code = r#"int (*apfi[3])(int *x, int *y);"#;

    let expected_result = r#"
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
            identifier: apfi
          derive:
            Array:
              qualifiers:
                const_q: false
                restrict_q: false
                volatile_q: false
                atomic_q: false
              is_static: false
              size_expr:
                Constant:
                  Number: "3"
              vla: false
              to:
                Pointer:
                  qualifiers:
                    const_q: false
                    restrict_q: false
                    volatile_q: false
                    atomic_q: false
                  to:
                    FunctionType:
                      parameter_type_list:
                        parameter_list:
                          - Declarator:
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
                              declarator:
                                base:
                                  identifier: x
                                derive:
                                  Pointer:
                                    qualifiers:
                                      const_q: false
                                      restrict_q: false
                                      volatile_q: false
                                      atomic_q: false
                                    to: Base
                          - Declarator:
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
                              declarator:
                                base:
                                  identifier: y
                                derive:
                                  Pointer:
                                    qualifiers:
                                      const_q: false
                                      restrict_q: false
                                      volatile_q: false
                                      atomic_q: false
                                    to: Base
                        ellipsis: false
                      to: Base
        - ~
  
    "#;

    let mut simple_parser = run_lexer_with_return_that_init_parser(code);
    let got_result = simple_parser.parse_declaration();
    println!("{}", serde_yaml::to_string(&got_result).unwrap());

    let expected_result = serde_yaml::from_str(expected_result).unwrap();

    assert_eq!(got_result, expected_result);
}

#[test]
fn declaration_advanced_1() {
    let code = r#"int (*fpfi(int (*)(long), int))(int, ...);"#;

    let expected_result = r#"
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
            identifier: fpfi
          derive:
            FunctionType:
              parameter_type_list:
                parameter_list:
                  - AbstractDeclarator:
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
                      abstract_declarator:
                        Pointer:
                          qualifiers:
                            const_q: false
                            restrict_q: false
                            volatile_q: false
                            atomic_q: false
                          to:
                            FunctionType:
                              parameter_type_list:
                                parameter_list:
                                  - AbstractDeclarator:
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
                                          Basic: Long
                                        function:
                                          inline: false
                                          no_return: false
                                        alignment: ~
                                      abstract_declarator: ~
                                ellipsis: false
                              to: Base
                  - AbstractDeclarator:
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
                      abstract_declarator: ~
                ellipsis: false
              to:
                Pointer:
                  qualifiers:
                    const_q: false
                    restrict_q: false
                    volatile_q: false
                    atomic_q: false
                  to:
                    FunctionType:
                      parameter_type_list:
                        parameter_list:
                          - AbstractDeclarator:
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
                              abstract_declarator: ~
                        ellipsis: true
                      to: Base
        - ~
   
    "#;

    let mut simple_parser = run_lexer_with_return_that_init_parser(code);
    let got_result = simple_parser.parse_declaration();
    println!("{}", serde_yaml::to_string(&got_result).unwrap());

    let expected_result = serde_yaml::from_str(expected_result).unwrap();

    assert_eq!(got_result, expected_result);
}

#[test]
fn declaration_advanced_2() {
    let code = r#"int f(void), *fip(), (*pfi)();"#;

    let expected_result = r#"
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
            identifier: f
          derive:
            FunctionType:
              parameter_type_list:
                parameter_list:
                  - AbstractDeclarator:
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
                          Basic: Void
                        function:
                          inline: false
                          no_return: false
                        alignment: ~
                      abstract_declarator: ~
                ellipsis: false
              to: Base
        - ~
      - - base:
            identifier: fip
          derive:
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
        - ~
      - - base:
            identifier: pfi
          derive:
            Pointer:
              qualifiers:
                const_q: false
                restrict_q: false
                volatile_q: false
                atomic_q: false
              to:
                FunctionType:
                  parameter_type_list:
                    parameter_list: []
                    ellipsis: false
                  to: Base
        - ~
  
    "#;

    let mut simple_parser = run_lexer_with_return_that_init_parser(code);
    let got_result = simple_parser.parse_declaration();
    println!("{}", serde_yaml::to_string(&got_result).unwrap());

    let expected_result = serde_yaml::from_str(expected_result).unwrap();

    assert_eq!(got_result, expected_result);
}
