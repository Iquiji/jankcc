use serde::{Deserialize, Serialize};

use self::{declarations::DeclarationSpecifiers, statements::CompoundStatement};

pub mod declarations;
pub mod expressions;
pub mod statements;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Identifier {
    pub(crate) identifier: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub(crate) struct NumberLike {
    pub(crate) from: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct StringLiteral {
    pub(crate) value: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum Constant {
    Number(NumberLike),
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum ExternalDeclaration {
    FunctionDefinition(Box<FunctionDefinition>),
    Declaration(),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct FunctionDefinition {
    specifiers: Box<DeclarationSpecifiers>,
    body: Box<CompoundStatement>,
}
