#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CTokenType {
    /// one of CKeyword
    Keyword(CKeyword),
    /// nondigit (nondigit | digit)*
    Identifier,
    /// integer-constant floating-constant enumeration-constant character-constant
    Constant,
    /// encoding-prefix opt " s-char-sequence opt "
    StringLiteral,
    /// one of see helper_funcs.rs/is_punctuator
    Punctuator,
    /// end
    Eof,
}

#[allow(clippy::upper_case_acronyms)]
#[allow(non_camel_case_types)]
#[allow(unused)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CKeyword {
    AUTO,
    BREAK,
    CASE,
    CHAR,
    CONST,
    CONTINUE,
    DEFAULT,
    DO,
    DOUBLE,
    ELSE,
    ENUM,
    EXTERN,
    FLOAT,
    FOR,
    GOTO,
    IF,
    INLINE,
    INT,
    LONG,
    REGISTER,
    RESTRICT,
    RETURN,
    SHORT,
    SIGNED,
    SIZEOF,
    STATIC,
    STRUCT,
    SWITCH,
    TYPEDEF,
    UNION,
    UNSIGNED,
    VOID,
    VOLATILE,
    WHILE,
    ALIGNAS,
    ALIGNOF,
    ATOMIC,
    BOOL,
    COMPLEX,
    GENERIC,
    IMAGINARY,
    NORETURN,
    STATIC_ASSERT,
    THREAD_LOCAL,
}
impl CKeyword {
    pub fn to_keyword(string: &str) -> Option<CKeyword> {
        use CKeyword::*;

        match string.to_ascii_uppercase().as_str() {
            "AUTO" => Some(AUTO),
            "BREAK" => Some(BREAK),
            "CASE" => Some(CASE),
            "CHAR" => Some(CHAR),
            "CONST" => Some(CONST),
            "CONTINUE" => Some(CONTINUE),
            "DEFAULT" => Some(DEFAULT),
            "DO" => Some(DO),
            "DOUBLE" => Some(DOUBLE),
            "ELSE" => Some(ELSE),
            "ENUM" => Some(ENUM),
            "EXTERN" => Some(EXTERN),
            "FLOAT" => Some(FLOAT),
            "FOR" => Some(FOR),
            "GOTO" => Some(GOTO),
            "IF" => Some(IF),
            "INLINE" => Some(INLINE),
            "INT" => Some(INT),
            "LONG" => Some(LONG),
            "REGISTER" => Some(REGISTER),
            "RESTRICT" => Some(RESTRICT),
            "RETURN" => Some(RETURN),
            "SHORT" => Some(SHORT),
            "SIGNED" => Some(SIGNED),
            "SIZEOF" => Some(SIZEOF),
            "STATIC" => Some(STATIC),
            "STRUCT" => Some(STRUCT),
            "SWITCH" => Some(SWITCH),
            "TYPEDEF" => Some(TYPEDEF),
            "UNION" => Some(UNION),
            "UNSIGNED" => Some(UNSIGNED),
            "VOID" => Some(VOID),
            "VOLATILE" => Some(VOLATILE),
            "WHILE" => Some(WHILE),
            "_ALIGNAS" => Some(ALIGNAS),
            "_ALIGNOF" => Some(ALIGNOF),
            "_ATOMIC" => Some(ATOMIC),
            "_BOOL" => Some(BOOL),
            "_COMPLEX" => Some(COMPLEX),
            "_GENERIC" => Some(GENERIC),
            "_IMAGINARY" => Some(IMAGINARY),
            "_NORETURN" => Some(NORETURN),
            "_STATIC_ASSERT" => Some(STATIC_ASSERT),
            "_THREAD_LOCAL" => Some(THREAD_LOCAL),
            _ => None,
        }
    }
}
