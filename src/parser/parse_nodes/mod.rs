use self::{
    declarations::{Declaration, DeclarationSpecifiers, Declarator},
    statements::CompoundStatement,
};

pub mod declarations;
pub mod expressions;
pub mod statements;

pub(crate) struct Identifier {
    string: String,
}
pub(crate) struct NumberLike {
    from: String,
}
pub(crate) struct StringLiteral {
    value: String,
}

pub(crate) enum Constant {
    Integer(NumberLike),
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
pub(crate) type TranslationUnit = Vec<ExternalDeclaration>;

pub(crate) enum ExternalDeclaration {
    FunctionDefinition(Box<FunctionDefinition>),
    Declaration(Box<Declaration>),
}

pub(crate) struct FunctionDefinition {
    specifiers: DeclarationSpecifiers,
    declarator: Declarator,
    declaration_list: Option<DeclarationList>,
    body: CompoundStatement,
}
pub(crate) type DeclarationList = Vec<Declaration>;
