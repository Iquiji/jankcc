use log::debug;
use serde::{Deserialize, Serialize};

use crate::lexer::token_types::CTokenType;

use self::{
    declarations::{Declaration, DeclarationSpecifiers, Declarator, DerivedDeclarator},
    statements::Statement,
};

use super::{span::Spanned, CParser};

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
pub(crate) type TranslationUnit = Vec<Spanned<ExternalDeclaration>>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum ExternalDeclaration {
    FunctionDefinition(Spanned<FunctionDefinition>),
    Declaration(Spanned<Declaration>),
}

impl CParser {
    pub(crate) fn parse_external_declaration(&mut self) -> Spanned<ExternalDeclaration> {
        let start = self.current_token().loc;

        // we need to differiantiate between declaratian and function
        debug!(
            "deciding on function or declaration: {}",
            self.current_token().loc
        );
        let before_differ_idx = self.idx;
        // common point decl_specifier
        self.parse_declaration_specifiers();
        // warn!("{:?}",);

        // ; -> no function
        // warn!("{:?}",self.current_token());

        let is_function = if self.current_token().t_type == CTokenType::Punctuator
            && self.current_token().original == ";"
        {
            false
        } else {
            // another common point if not early end on declaration
            self.parse_declarator();
            // warn!("{:?}",self.current_token());
            if self.current_token().t_type == CTokenType::Punctuator
                && self.current_token().original == "="
            {
                false
            } else {
                !(self.current_token().t_type == CTokenType::Punctuator
                    && (self.current_token().original == ","
                        || self.current_token().original == ";"))
            }
        };
        // reset after "search"
        self.idx = before_differ_idx;

        let res = if is_function {
            debug!("function on loc: {}", self.current_token().loc);
            ExternalDeclaration::FunctionDefinition(self.parse_function_definition())
        } else {
            debug!("declaration on loc: {}", self.current_token().loc);
            ExternalDeclaration::Declaration(self.parse_declaration())
        };
        debug!("finished!");

        Spanned::new(res, start, self.prev_token().loc)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct FunctionDefinition {
    pub(crate) function_specifiers: DeclarationSpecifiers,
    /// base of declarator is the name of the function
    pub(crate) declarator: Spanned<Declarator>,
    /// if declarator is a identifier list, this specifies the types
    pub(crate) declarations: Vec<Spanned<Declaration>>,
    pub(crate) body: Spanned<Statement>,
}

impl CParser {
    pub(crate) fn parse_function_definition(&mut self) -> Spanned<FunctionDefinition> {
        let start = self.current_token().loc;

        let function_specifiers = self.parse_declaration_specifiers();

        let declarator = self.parse_declarator();

        // if declarator is a identifier list, this specifies the types
        let declarations = if let DerivedDeclarator::FunctionIdentified {
            identifier_list: _,
            to: _,
        } = declarator.inner.derive.clone()
        {
            let mut buf = vec![];
            while !(self.current_token().t_type == CTokenType::Punctuator
                && self.current_token().original == "{")
            {
                buf.push(self.parse_declaration());
            }
            buf
        } else {
            vec![]
        };

        self.expect_type_and_string(CTokenType::Punctuator, "{");
        self.idx -= 1;

        let body = self.parse_statement();

        Spanned::new(
            FunctionDefinition {
                function_specifiers,
                declarator,
                declarations,
                body,
            },
            start,
            self.prev_token().loc,
        )
    }
}
