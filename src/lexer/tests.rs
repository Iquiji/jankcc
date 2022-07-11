use crate::lexer::CKeyword::*;
use crate::lexer::CToken;
use crate::lexer::CTokenType::*;
use crate::lexer::Lexer;
use crate::lexer::OriginalLocation;

#[cfg(test)]
use pretty_assertions::assert_eq;

#[test]
fn test_lexer_simple() {
    let input = r#"
int main() {
    printf("hello world\n");
    if 8 == 9{
        return 1;
    } else {
        return 0;
    }
}"#;

    // TODO fix collumn number in lexer
    let expected_output = vec![
        CToken {
            t_type: Keyword(INT),
            original: "int".to_string(),
            loc: OriginalLocation {
                file: "".to_string(),
                line: 1,
                collumn: 0,
            },
        },
        CToken {
            t_type: Identifier,
            original: "main".to_string(),
            loc: OriginalLocation {
                file: "".to_string(),
                line: 1,
                collumn: 0,
            },
        },
        CToken {
            t_type: Punctuator,
            original: "(".to_string(),
            loc: OriginalLocation {
                file: "".to_string(),
                line: 1,
                collumn: 0,
            },
        },
        CToken {
            t_type: Punctuator,
            original: ")".to_string(),
            loc: OriginalLocation {
                file: "".to_string(),
                line: 1,
                collumn: 1,
            },
        },
        CToken {
            t_type: Punctuator,
            original: "{".to_string(),
            loc: OriginalLocation {
                file: "".to_string(),
                line: 1,
                collumn: 2,
            },
        },
        CToken {
            t_type: Identifier,
            original: "printf".to_string(),
            loc: OriginalLocation {
                file: "".to_string(),
                line: 2,
                collumn: 0,
            },
        },
        CToken {
            t_type: Punctuator,
            original: "(".to_string(),
            loc: OriginalLocation {
                file: "".to_string(),
                line: 2,
                collumn: 0,
            },
        },
        CToken {
            t_type: StringLiteral,
            original: "hello world\\n".to_string(),
            loc: OriginalLocation {
                file: "".to_string(),
                line: 2,
                collumn: 1,
            },
        },
        CToken {
            t_type: Punctuator,
            original: ")".to_string(),
            loc: OriginalLocation {
                file: "".to_string(),
                line: 2,
                collumn: 2,
            },
        },
        CToken {
            t_type: Punctuator,
            original: ";".to_string(),
            loc: OriginalLocation {
                file: "".to_string(),
                line: 2,
                collumn: 3,
            },
        },
        CToken {
            t_type: Keyword(IF),
            original: "if".to_string(),
            loc: OriginalLocation {
                file: "".to_string(),
                line: 3,
                collumn: 0,
            },
        },
        CToken {
            t_type: Constant,
            original: "8".to_string(),
            loc: OriginalLocation {
                file: "".to_string(),
                line: 3,
                collumn: 0,
            },
        },
        CToken {
            t_type: Punctuator,
            original: "==".to_string(),
            loc: OriginalLocation {
                file: "".to_string(),
                line: 3,
                collumn: 1,
            },
        },
        CToken {
            t_type: Constant,
            original: "9".to_string(),
            loc: OriginalLocation {
                file: "".to_string(),
                line: 3,
                collumn: 1,
            },
        },
        CToken {
            t_type: Punctuator,
            original: "{".to_string(),
            loc: OriginalLocation {
                file: "".to_string(),
                line: 3,
                collumn: 1,
            },
        },
        CToken {
            t_type: Keyword(RETURN),
            original: "return".to_string(),
            loc: OriginalLocation {
                file: "".to_string(),
                line: 4,
                collumn: 0,
            },
        },
        CToken {
            t_type: Constant,
            original: "1".to_string(),
            loc: OriginalLocation {
                file: "".to_string(),
                line: 4,
                collumn: 0,
            },
        },
        CToken {
            t_type: Punctuator,
            original: ";".to_string(),
            loc: OriginalLocation {
                file: "".to_string(),
                line: 4,
                collumn: 0,
            },
        },
        CToken {
            t_type: Punctuator,
            original: "}".to_string(),
            loc: OriginalLocation {
                file: "".to_string(),
                line: 5,
                collumn: 1,
            },
        },
        CToken {
            t_type: Keyword(ELSE),
            original: "else".to_string(),
            loc: OriginalLocation {
                file: "".to_string(),
                line: 5,
                collumn: 1,
            },
        },
        CToken {
            t_type: Punctuator,
            original: "{".to_string(),
            loc: OriginalLocation {
                file: "".to_string(),
                line: 5,
                collumn: 2,
            },
        },
        CToken {
            t_type: Keyword(RETURN),
            original: "return".to_string(),
            loc: OriginalLocation {
                file: "".to_string(),
                line: 6,
                collumn: 0,
            },
        },
        CToken {
            t_type: Constant,
            original: "0".to_string(),
            loc: OriginalLocation {
                file: "".to_string(),
                line: 6,
                collumn: 0,
            },
        },
        CToken {
            t_type: Punctuator,
            original: ";".to_string(),
            loc: OriginalLocation {
                file: "".to_string(),
                line: 6,
                collumn: 0,
            },
        },
        CToken {
            t_type: Punctuator,
            original: "}".to_string(),
            loc: OriginalLocation {
                file: "".to_string(),
                line: 7,
                collumn: 1,
            },
        },
        CToken {
            t_type: Punctuator,
            original: "}".to_string(),
            loc: OriginalLocation {
                file: "".to_string(),
                line: 8,
                collumn: 1,
            },
        },
    ];

    assert_eq!(
        Lexer::new().string_to_token_arr(input.to_string()),
        expected_output
    );
}
#[test]
fn test_lexer_extended_punctuators() {
    let input = r#"--++->"#;

    // TODO fix collumn number in lexer
    let expected_output = vec![
        CToken {
            t_type: Punctuator,
            original: "--".to_string(),
            loc: OriginalLocation {
                file: "".to_string(),
                line: 0,
                collumn: 1,
            },
        },
        CToken {
            t_type: Punctuator,
            original: "++".to_string(),
            loc: OriginalLocation {
                file: "".to_string(),
                line: 0,
                collumn: 2,
            },
        },
        CToken {
            t_type: Punctuator,
            original: "->".to_string(),
            loc: OriginalLocation {
                file: "".to_string(),
                line: 0,
                collumn: 3,
            },
        },
    ];

    assert_eq!(
        Lexer::new().string_to_token_arr(input.to_string()),
        expected_output
    );
}

#[test]
fn b_plus_a() {
    let input = r#"b+a b + a"#;

    // TODO fix collumn number in lexer
    let expected_output = vec![
        CToken {
            t_type: Identifier,
            original: "b".to_string(),
            loc: OriginalLocation {
                file: "".to_string(),
                line: 0,
                collumn: 0,
            },
        },
        CToken {
            t_type: Punctuator,
            original: "+".to_string(),
            loc: OriginalLocation {
                file: "".to_string(),
                line: 0,
                collumn: 0,
            },
        },
        CToken {
            t_type: Identifier,
            original: "a".to_string(),
            loc: OriginalLocation {
                file: "".to_string(),
                line: 0,
                collumn: 0,
            },
        },
        CToken {
            t_type: Identifier,
            original: "b".to_string(),
            loc: OriginalLocation {
                file: "".to_string(),
                line: 0,
                collumn: 0,
            },
        },
        CToken {
            t_type: Punctuator,
            original: "+".to_string(),
            loc: OriginalLocation {
                file: "".to_string(),
                line: 0,
                collumn: 1,
            },
        },
        CToken {
            t_type: Identifier,
            original: "a".to_string(),
            loc: OriginalLocation {
                file: "".to_string(),
                line: 0,
                collumn: 1,
            },
        },
    ];

    assert_eq!(
        Lexer::new().string_to_token_arr(input.to_string()),
        expected_output
    );
}

#[test]
fn b_plus_a_ext() {
    let input = r#"--b + a"#;

    // TODO fix collumn number in lexer
    let expected_output = vec![
        CToken {
            t_type: Punctuator,
            original: "--".to_string(),
            loc: OriginalLocation {
                file: "".to_string(),
                line: 0,
                collumn: 1,
            },
        },
        CToken {
            t_type: Identifier,
            original: "b".to_string(),
            loc: OriginalLocation {
                file: "".to_string(),
                line: 0,
                collumn: 1,
            },
        },
        CToken {
            t_type: Punctuator,
            original: "+".to_string(),
            loc: OriginalLocation {
                file: "".to_string(),
                line: 0,
                collumn: 2,
            },
        },
        CToken {
            t_type: Identifier,
            original: "a".to_string(),
            loc: OriginalLocation {
                file: "".to_string(),
                line: 0,
                collumn: 2,
            },
        },
    ];

    assert_eq!(
        Lexer::new().string_to_token_arr(input.to_string()),
        expected_output
    );
}