use log::{error, warn};
use serde::{Deserialize, Serialize};

use crate::{
    lexer::{token_types::CKeyword, OriginalLocation},
    parser::{
        span::{Span, Spanned},
        types::{CTypeName, CTypeQualifiers, CTypeSpecifier},
        CParser,
    },
};

use super::{expressions::*, Identifier, StringLiteral};

use crate::lexer::token_types::CTokenType;

/*
(6.7.10) static_assert-declaration:
    _Static_assert ( constant-expression , string-literal ) ;
*/
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct StaticAssertDeclaration {
    expression: ConstantExpression,
    string_literal: StringLiteral,
}

impl CParser {
    pub(crate) fn parse_static_assert(&mut self) -> Spanned<StaticAssertDeclaration> {
        let start = self.current_token().loc;

        self.expect_type(CTokenType::Keyword(CKeyword::STATIC_ASSERT));
        self.expect_type_and_string(CTokenType::Punctuator, "(");

        let assert_decl = StaticAssertDeclaration {
            expression: self.parse_constant_expr(),
            string_literal: StringLiteral {
                value: {
                    self.expect_type_and_string(CTokenType::Punctuator, ",");
                    self.expect_type(CTokenType::StringLiteral).original
                },
            },
        };

        self.expect_type_and_string(CTokenType::Punctuator, ")");

        Spanned::new(assert_decl, start, self.prev_token().loc)
    }
}

/*

Struct:
list of declarations,
each declaration:
- specifier-qualifier-list struct-declarator-list?
struct-declarator-list: list of:
- declarator
or
- declarator opt : constant-expression
declarator:
- pointer opt direct-declarator


Struct:
Vec<Decl>
Decl:
- specifier-qualifier-list
 - List of:
  -Normal
   or
  -Initialized

*/
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct DeclarationSpecifiers {
    storage: CStorageClass,
    qualifiers: CTypeQualifiers,
    specifiers: CTypeSpecifier,
    function: CFunctionSpecifier,
    alignment: Option<CAlignmentSpecifier>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct CStorageClass {
    typedef_c: bool,
    extern_c: bool,
    static_c: bool,
    thread_local_c: bool,
    auto_c: bool,
    register: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct CFunctionSpecifier {
    inline: bool,
    no_return: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum CAlignmentSpecifier {
    ToType(CTypeName),
    ToExpression(ConstantExpression),
}

/*
Declarator:

Based or not Based <- Ground Element
Chain of derived types up to None
Based-Type?

*/

/*
Declarator is universal just different versioning for
type-name
and
declarator
*/
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum DerivedDeclarator {
    Base,
    Pointer {
        qualifiers: Spanned<CTypeQualifiers>,
        to: Box<Self>,
    },
    Binded(Box<Self>),
    Array {
        qualifiers: Spanned<CTypeQualifiers>,
        is_static: bool,
        size_expr: Option<Spanned<CExpression>>,
        // Variable length Array
        vla: bool,
        to: Box<Self>,
    },
    FunctionType {
        parameter_type_list: Spanned<ParameterTypeList>,
        to: Box<Self>,
    },
    FunctionIdentified {
        identifier_list: Vec<Spanned<Identifier>>,
        to: Box<Self>,
    },
}

impl CParser {
    /// returns base or pointer depending on if pointing or not
    pub(crate) fn parse_pointer(&mut self, base: DerivedDeclarator) -> DerivedDeclarator {
        if self.current_token().t_type == CTokenType::Punctuator
            && self.current_token().original == "*"
        {
            self.advance_idx();

            let qualifiers = self.parse_type_qualifiers();
            self.parse_pointer(DerivedDeclarator::Pointer {
                qualifiers,
                to: Box::new(base),
            })
        } else {
            base
        }
    }

    pub(crate) fn parse_abstract_declarator(&mut self) -> DerivedDeclarator {
        let mut base = self.parse_pointer(DerivedDeclarator::Base);

        // discern between ( abstract_declarator ) and ( parameter_type_list )
        // following '(' is '(' or '[' or '*'
        if self.current_token().t_type == CTokenType::Punctuator
            && self.current_token().original == "("
            && self.next_token().t_type == CTokenType::Punctuator
            && ["(", "[", "*"].contains(&self.next_token().original.as_str())
        {
            if base != DerivedDeclarator::Base {
                error!("direct-abstract-declarator is based and there is a pointer => currently confuzius!");
            }
            self.advance_idx();

            // TODO: this gets to outermost because reasons

            base = DerivedDeclarator::Binded(Box::new(self.parse_abstract_declarator()));

            self.expect_type_and_string(CTokenType::Punctuator, ")");
        }

        while self.current_token().t_type == CTokenType::Punctuator {
            if self.current_token().original == "(" {
                // parameter type list
                self.advance_idx();
                if !(self.current_token().t_type == CTokenType::Punctuator
                    && self.current_token().original == ")")
                {
                    //there is a parameter type list
                    todo!("Parameter Type List")
                } else {
                    base = DerivedDeclarator::FunctionType {
                        parameter_type_list: Spanned::new(
                            ParameterTypeList {
                                parameter_list: vec![],
                                ellipsis: false,
                            },
                            self.current_token().loc,
                            self.current_token().loc,
                        ),
                        to: Box::new(base),
                    };
                }
                self.expect_type_and_string(CTokenType::Punctuator, ")");
            } else if self.current_token().original == "[" {
                // array
                self.advance_idx();

                //VLA 
                if self.current_token().t_type == CTokenType::Punctuator
                    && self.current_token().original == "*"
                {                        
                    base = DerivedDeclarator::Array {
                        qualifiers: self.parse_type_qualifiers(),
                        is_static: false,
                        size_expr: None,
                        vla: true,
                        to: Box::new(base),
                    };

                    self.advance_idx();
                    self.expect_type_and_string(CTokenType::Punctuator, "]");
                 
                    continue;
                }

                let mut static_flag = false;

                if self.current_token().t_type == CTokenType::Keyword(CKeyword::STATIC){
                    static_flag = true;
                    self.advance_idx();
                }

                let qualifiers = self.parse_type_qualifiers();

                if self.current_token().t_type == CTokenType::Keyword(CKeyword::STATIC){
                    static_flag = true;
                    self.advance_idx();
                }

                let mut size_expr = None;

                if !(self.current_token().t_type == CTokenType::Punctuator
                    && self.current_token().original == "]")
                {
                    size_expr = Some(self.parse_expr_assignment());
                }

                base = DerivedDeclarator::Array {
                    qualifiers,
                    is_static: static_flag,
                    size_expr,
                    vla: false,
                    to: Box::new(base),
                };

                self.expect_type_and_string(CTokenType::Punctuator, "]");
            } else {
                break;
            }
        }

        base
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ParameterTypeList {
    parameter_list: Vec<Spanned<ParameterDeclaration>>,
    ellipsis: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum ParameterDeclaration {
    // TODO:\
    // declaration-specifiers declarator
    // declaration-specifiers abstract-declarator?
    Declarator {
        specifiers: DeclarationSpecifiers,
        declarator: Spanned<DerivedDeclarator>,
    },
    AbstractDeclarator {
        specifiers: DeclarationSpecifiers,
        abstract_declarator: Option<Spanned<DerivedDeclarator>>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum Initializer {
    Single(Spanned<CExpression>),
    Compound(Vec<(Vec<Designator>, Spanned<Self>)>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum Designator {
    Array(Spanned<ConstantExpression>),
    Member(Identifier),
}

pub(crate) type InitializerList = Vec<(Vec<Designator>, Spanned<Initializer>)>;
