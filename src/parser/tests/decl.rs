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

#[test]
fn declaration_initialized_0() {
    let code = r#"int x[] = { 1, 3, 5 };"#;

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
            identifier: x
          derive:
            Array:
              qualifiers:
                const_q: false
                restrict_q: false
                volatile_q: false
                atomic_q: false
              is_static: false
              size_expr: ~
              vla: false
              to: Base
        - Compound:
            - - []
              - Single:
                  Constant:
                    Number: "1"
            - - []
              - Single:
                  Constant:
                    Number: "3"
            - - []
              - Single:
                  Constant:
                    Number: "5"
  
    "#;

    let mut simple_parser = run_lexer_with_return_that_init_parser(code);
    let got_result = simple_parser.parse_declaration();
    println!("{}", serde_yaml::to_string(&got_result).unwrap());

    let expected_result = serde_yaml::from_str(expected_result).unwrap();

    assert_eq!(got_result, expected_result);
}

#[test]
fn declaration_initialized_1() {
    let code = r#"
int y[4][3] = {
    { 1, 3, 5 },
    { 2, 4, 6 },
    { 3, 5, 7 },
};"#;

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
            identifier: y
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
                Array:
                  qualifiers:
                    const_q: false
                    restrict_q: false
                    volatile_q: false
                    atomic_q: false
                  is_static: false
                  size_expr:
                    Constant:
                      Number: "4"
                  vla: false
                  to: Base
        - Compound:
            - - []
              - Compound:
                  - - []
                    - Single:
                        Constant:
                          Number: "1"
                  - - []
                    - Single:
                        Constant:
                          Number: "3"
                  - - []
                    - Single:
                        Constant:
                          Number: "5"
            - - []
              - Compound:
                  - - []
                    - Single:
                        Constant:
                          Number: "2"
                  - - []
                    - Single:
                        Constant:
                          Number: "4"
                  - - []
                    - Single:
                        Constant:
                          Number: "6"
            - - []
              - Compound:
                  - - []
                    - Single:
                        Constant:
                          Number: "3"
                  - - []
                    - Single:
                        Constant:
                          Number: "5"
                  - - []
                    - Single:
                        Constant:
                          Number: "7"
  
    "#;

    let mut simple_parser = run_lexer_with_return_that_init_parser(code);
    let got_result = simple_parser.parse_declaration();
    println!("{}", serde_yaml::to_string(&got_result).unwrap());

    let expected_result = serde_yaml::from_str(expected_result).unwrap();

    assert_eq!(got_result, expected_result);
}

#[test]
fn declaration_initialized_2() {
    let code = r#"
    short q[4][3][2] = {
        { 1 },
        { 2, 3 },
        { 4, 5, 6 }
    };"#;

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
        Basic: Short
      function:
        inline: false
        no_return: false
      alignment: ~
    init:
      - - base:
            identifier: q
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
                  Number: "2"
              vla: false
              to:
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
                    Array:
                      qualifiers:
                        const_q: false
                        restrict_q: false
                        volatile_q: false
                        atomic_q: false
                      is_static: false
                      size_expr:
                        Constant:
                          Number: "4"
                      vla: false
                      to: Base
        - Compound:
            - - []
              - Compound:
                  - - []
                    - Single:
                        Constant:
                          Number: "1"
            - - []
              - Compound:
                  - - []
                    - Single:
                        Constant:
                          Number: "2"
                  - - []
                    - Single:
                        Constant:
                          Number: "3"
            - - []
              - Compound:
                  - - []
                    - Single:
                        Constant:
                          Number: "4"
                  - - []
                    - Single:
                        Constant:
                          Number: "5"
                  - - []
                    - Single:
                        Constant:
                          Number: "6"
    "#;

    let mut simple_parser = run_lexer_with_return_that_init_parser(code);
    let got_result = simple_parser.parse_declaration();
    println!("{}", serde_yaml::to_string(&got_result).unwrap());

    let expected_result = serde_yaml::from_str(expected_result).unwrap();

    assert_eq!(got_result, expected_result);
}

#[test]
fn declaration_struct_initialized_0() {
    let code = r#"struct { int a[3], b; } w[] = { { 1 }, 2 };"#;

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
        StructOrUnion:
          struct_type: Struct
          ident: ~
          declarations:
            - StructDeclaration:
                specifier_qualifier:
                  qualifiers:
                    const_q: false
                    restrict_q: false
                    volatile_q: false
                    atomic_q: false
                  specifier:
                    Basic: Int
                delcarator_list:
                  - Declarator:
                      base:
                        identifier: a
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
                          to: Base
                  - Declarator:
                      base:
                        identifier: b
                      derive: Base
      function:
        inline: false
        no_return: false
      alignment: ~
    init:
      - - base:
            identifier: w
          derive:
            Array:
              qualifiers:
                const_q: false
                restrict_q: false
                volatile_q: false
                atomic_q: false
              is_static: false
              size_expr: ~
              vla: false
              to: Base
        - Compound:
            - - []
              - Compound:
                  - - []
                    - Single:
                        Constant:
                          Number: "1"
            - - []
              - Single:
                  Constant:
                    Number: "2"
  
    "#;

    let mut simple_parser = run_lexer_with_return_that_init_parser(code);
    let got_result = simple_parser.parse_declaration();
    println!("{}", serde_yaml::to_string(&got_result).unwrap());

    let expected_result = serde_yaml::from_str(expected_result).unwrap();

    assert_eq!(got_result, expected_result);
}

#[test]
fn struct_declaration_nested() {
    let code = r#"
struct s {
    struct { int i; };
    int a[];
};"#;

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
        StructOrUnion:
          struct_type: Struct
          ident:
            identifier: s
          declarations:
            - StructDeclaration:
                specifier_qualifier:
                  qualifiers:
                    const_q: false
                    restrict_q: false
                    volatile_q: false
                    atomic_q: false
                  specifier:
                    StructOrUnion:
                      struct_type: Struct
                      ident: ~
                      declarations:
                        - StructDeclaration:
                            specifier_qualifier:
                              qualifiers:
                                const_q: false
                                restrict_q: false
                                volatile_q: false
                                atomic_q: false
                              specifier:
                                Basic: Int
                            delcarator_list:
                              - Declarator:
                                  base:
                                    identifier: i
                                  derive: Base
                delcarator_list: []
            - StructDeclaration:
                specifier_qualifier:
                  qualifiers:
                    const_q: false
                    restrict_q: false
                    volatile_q: false
                    atomic_q: false
                  specifier:
                    Basic: Int
                delcarator_list:
                  - Declarator:
                      base:
                        identifier: a
                      derive:
                        Array:
                          qualifiers:
                            const_q: false
                            restrict_q: false
                            volatile_q: false
                            atomic_q: false
                          is_static: false
                          size_expr: ~
                          vla: false
                          to: Base
      function:
        inline: false
        no_return: false
      alignment: ~
    init: []
  
    "#;

    let mut simple_parser = run_lexer_with_return_that_init_parser(code);
    let got_result = simple_parser.parse_declaration();
    println!("{}", serde_yaml::to_string(&got_result).unwrap());

    let expected_result = serde_yaml::from_str(expected_result).unwrap();

    assert_eq!(got_result, expected_result);
}

#[test]
fn type_initializer_simple_declaration() {
    let code = r#"int *p = (int []){2, 4};"#;

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
          identifier: p
        derive:
          Pointer:
            qualifiers:
              const_q: false
              restrict_q: false
              volatile_q: false
              atomic_q: false
            to: Base
      - Single:
          TypeInitializer:
            type_name:
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
                  size_expr: ~
                  vla: false
                  to: Base
            initializer_list:
              Compound:
                - - []
                  - Single:
                      Constant:
                        Number: "2"
                - - []
                  - Single:
                      Constant:
                        Number: "4"

"#;

    let mut simple_parser = run_lexer_with_return_that_init_parser(code);
    let got_result = simple_parser.parse_declaration();
    println!("{}", serde_yaml::to_string(&got_result).unwrap());

    let expected_result = serde_yaml::from_str(expected_result).unwrap();

    assert_eq!(got_result, expected_result);
}

#[test]
fn enum_declaration_simple() {
    let code = r#"enum hue { chartreuse, burgundy, claret=20, winedark };"#;

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
      Enum:
        ident:
          identifier: hue
        enumerators:
          - enumeration_constant:
              identifier: chartreuse
            const_assignment: ~
          - enumeration_constant:
              identifier: burgundy
            const_assignment: ~
          - enumeration_constant:
              identifier: claret
            const_assignment:
              internal:
                Constant:
                  Number: "20"
          - enumeration_constant:
              identifier: winedark
            const_assignment: ~
    function:
      inline: false
      no_return: false
    alignment: ~
  init: []
  
"#;

    let mut simple_parser = run_lexer_with_return_that_init_parser(code);
    let got_result = simple_parser.parse_declaration();
    println!("{}", serde_yaml::to_string(&got_result).unwrap());

    let expected_result = serde_yaml::from_str(expected_result).unwrap();

    assert_eq!(got_result, expected_result);
}
