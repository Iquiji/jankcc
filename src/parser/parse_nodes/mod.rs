mod expressions;
mod declarations;
mod statements;

pub(crate) struct Identifier{
    string: String,
}
pub(crate) struct NumberLike{
    from: String,
}
pub(crate) struct StringLiteral{
    value: String,
}

/*
A.2.4 External definitions
    (6.9) translation-unit:
        external-declaration
        translation-unit external-declaration
    (6.9) external-declaration:
        function-definition
        declaration
    (6.9.1) function-definition:
        declaration-specifiers declarator declaration-listopt compound-statement
    (6.9.1) declaration-list:
        declaration
        declaration-list declaration
*/