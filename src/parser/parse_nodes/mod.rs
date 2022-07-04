use self::{
    declarations::{Declaration, DeclarationSpecifiers, Declarator},
    statements::CompoundStatement,
};

pub mod declarations;
pub mod expressions;
pub mod statements;

#[derive(Debug,Clone,PartialEq,Eq)]
pub(crate) struct Identifier {
    string: String,
}

#[derive(Debug,Clone,PartialEq,Eq)]
pub(crate) struct NumberLike {
    from: String,
}

#[derive(Debug,Clone,PartialEq,Eq)]
pub(crate) struct StringLiteral {
    value: String,
}

#[derive(Debug,Clone,PartialEq,Eq)]
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

#[derive(Debug,Clone,PartialEq,Eq)]
pub(crate) enum ExternalDeclaration {
    FunctionDefinition(Box<FunctionDefinition>),
    Declaration(Box<Declaration>),
}

#[derive(Debug,Clone,PartialEq,Eq)]
pub(crate) struct FunctionDefinition {
    specifiers: Box<DeclarationSpecifiers>,
    declarator: Box<Declarator>,
    declaration_list: Option<Box<DeclarationList>>,
    body: Box<CompoundStatement>,
}
pub(crate) type DeclarationList = Vec<Declaration>;
