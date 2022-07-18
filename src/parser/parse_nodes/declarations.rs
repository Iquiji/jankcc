use std::ops::Add;

use log::{error, warn};
use serde::{Deserialize, Serialize};

use crate::{
    lexer::{token_types::CKeyword, OriginalLocation},
    parser::{
        span::{Span, Spanned},
        types::{
            CBasicTypes, CEnumType, CStructType, CTypeBasic, CTypeName, CTypeQualifiers,
            CTypeSpecifier,
        },
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
    alignment: Option<Spanned<CAlignmentSpecifier>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct CStorageClass {
    typedef_c: bool,
    extern_c: bool,
    static_c: bool,
    thread_local_c: bool,
    auto_c: bool,
    register_c: bool,
}

impl Add for CStorageClass {
    type Output = CStorageClass;

    fn add(self, rhs: Self) -> Self::Output {
        CStorageClass {
            typedef_c: self.typedef_c || rhs.typedef_c,
            extern_c: self.extern_c || rhs.extern_c,
            static_c: self.static_c || rhs.static_c,
            thread_local_c: self.thread_local_c || rhs.thread_local_c,
            auto_c: self.auto_c || rhs.auto_c,
            register_c: self.register_c || rhs.register_c,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct CFunctionSpecifier {
    inline: bool,
    no_return: bool,
}

impl Add for CFunctionSpecifier {
    type Output = CFunctionSpecifier;

    fn add(self, rhs: Self) -> Self::Output {
        CFunctionSpecifier {
            inline: self.inline || rhs.inline,
            no_return: self.no_return || rhs.no_return,
        }
    }
}

impl CParser {
    pub(crate) fn parse_c_function_specifier(&mut self) -> Spanned<CFunctionSpecifier> {
        let qualifier_possible = [CKeyword::INLINE, CKeyword::NORETURN];
        let matcher = |key: &CKeyword, quals: &mut CFunctionSpecifier| match key {
            CKeyword::INLINE => {
                quals.inline = true;
            }
            CKeyword::NORETURN => {
                quals.no_return = true;
            }
            _ => unreachable!(),
        };

        let start = self.current_token().loc;

        let mut storage_class = CFunctionSpecifier {
            inline: false,
            no_return: false,
        };

        // get beginning storage_class
        use crate::parser::CTokenType::Keyword;
        while let Keyword(keyword) = self.current_token().t_type {
            if qualifier_possible.contains(&keyword) {
                self.advance_idx();
                matcher(&keyword, &mut storage_class);
            } else {
                break;
            }
        }

        let end = self.prev_token().loc;

        Spanned::new(storage_class, start, end)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum CAlignmentSpecifier {
    ToType(Spanned<CTypeName>),
    ToExpression(ConstantExpression),
}
impl CParser {
    pub(crate) fn parse_maybe_alignment_specifier(
        &mut self,
    ) -> Option<Spanned<CAlignmentSpecifier>> {
        let start = self.current_token().loc;

        if self.current_token().t_type == CTokenType::Keyword(CKeyword::ALIGNAS) {
            self.advance_idx();
            self.expect_type_and_string(CTokenType::Punctuator, "(");

            if self.check_is_start_of_type_name(&self.current_token()) {
                Some(Spanned::new(
                    CAlignmentSpecifier::ToType(self.parse_type_name()),
                    start,
                    self.expect_type_and_string(CTokenType::Punctuator, ")").loc,
                ))
            } else {
                Some(Spanned::new(
                    CAlignmentSpecifier::ToExpression(self.parse_constant_expr()),
                    start,
                    self.expect_type_and_string(CTokenType::Punctuator, ")").loc,
                ))
            }
        } else {
            None
        }
    }
}

impl CParser {
    pub(crate) fn parse_storage_class(&mut self) -> Spanned<CStorageClass> {
        let qualifier_possible = [
            CKeyword::TYPEDEF,
            CKeyword::EXTERN,
            CKeyword::STATIC,
            CKeyword::THREAD_LOCAL,
            CKeyword::AUTO,
            CKeyword::REGISTER,
        ];
        let matcher = |key: &CKeyword, quals: &mut CStorageClass| match key {
            CKeyword::TYPEDEF => {
                quals.typedef_c = true;
            }
            CKeyword::EXTERN => {
                quals.extern_c = true;
            }
            CKeyword::STATIC => {
                quals.static_c = true;
            }
            CKeyword::THREAD_LOCAL => {
                quals.thread_local_c = true;
            }
            CKeyword::AUTO => {
                quals.auto_c = true;
            }
            CKeyword::REGISTER => {
                quals.register_c = true;
            }
            _ => unreachable!(),
        };

        let start = self.current_token().loc;

        let mut storage_class = CStorageClass {
            typedef_c: false,
            extern_c: false,
            static_c: false,
            thread_local_c: false,
            auto_c: false,
            register_c: false,
        };

        // get beginning storage_class
        use crate::parser::CTokenType::Keyword;
        while let Keyword(keyword) = self.current_token().t_type {
            if qualifier_possible.contains(&keyword) {
                self.advance_idx();
                matcher(&keyword, &mut storage_class);
            } else {
                break;
            }
        }

        let end = self.prev_token().loc;

        Spanned::new(storage_class, start, end)
    }

    pub(crate) fn parse_declaration_specifiers(&mut self) -> DeclarationSpecifiers {
        /*
        storage-class-specifier => typedef,extern,static,_Thread_local,auto,register
        type-specifier => known
        type-qualifer => known
        function-specifier => inline,_Noreturn
        alignment-specifier => _Alignas ( .. )
        */
        let mut decl_spec = DeclarationSpecifiers {
            storage: CStorageClass {
                typedef_c: false,
                extern_c: false,
                static_c: false,
                thread_local_c: false,
                auto_c: false,
                register_c: false,
            },
            qualifiers: CTypeQualifiers {
                const_q: false,
                restrict_q: false,
                volatile_q: false,
                atomic_q: false,
            },
            specifiers: CTypeSpecifier::Basic(CBasicTypes::Int),
            function: CFunctionSpecifier {
                inline: false,
                no_return: false,
            },
            alignment: None,
        };
        decl_spec.storage = decl_spec.storage + *self.parse_storage_class().inner;
        decl_spec.qualifiers = decl_spec.qualifiers + *self.parse_type_qualifiers().inner;
        decl_spec.function = decl_spec.function + *self.parse_c_function_specifier().inner;
        decl_spec.alignment = self.parse_maybe_alignment_specifier();

        let temp = *self.parse_specifier_qualifier_list().inner;
        decl_spec.qualifiers = decl_spec.qualifiers + temp.qualifiers;
        decl_spec.specifiers = temp.specifier;

        loop{
            let old_decl_spec = decl_spec.clone();

            decl_spec.storage = decl_spec.storage + *self.parse_storage_class().inner;
            decl_spec.qualifiers = decl_spec.qualifiers + *self.parse_type_qualifiers().inner;
            decl_spec.function = decl_spec.function + *self.parse_c_function_specifier().inner;
            decl_spec.alignment = self.parse_maybe_alignment_specifier();
            if old_decl_spec == decl_spec{
                break;
            }
        }


        decl_spec
    }
}

/*
Declarator:

Based or not Based <- Ground Element
Chain of derived types up to None
Based-Type?

*/
fn traverse_derived_replace_base(
    input: DerivedDeclarator,
    replacement: DerivedDeclarator,
) -> DerivedDeclarator {
    match input {
        DerivedDeclarator::Base => replacement,
        DerivedDeclarator::Pointer { qualifiers, to } => DerivedDeclarator::Pointer {
            qualifiers,
            to: Box::new(traverse_derived_replace_base(*to, replacement)),
        },
        // DerivedDeclarator::Binded(bound) => {
        //     DerivedDeclarator::Binded(Box::new(traverse_derived_replace_base(*bound,replacement)))
        // },
        DerivedDeclarator::Array {
            qualifiers,
            is_static,
            size_expr,
            vla,
            to,
        } => DerivedDeclarator::Array {
            qualifiers,
            is_static,
            size_expr,
            vla,
            to: Box::new(traverse_derived_replace_base(*to, replacement)),
        },
        DerivedDeclarator::FunctionType {
            parameter_type_list,
            to,
        } => DerivedDeclarator::FunctionType {
            parameter_type_list,
            to: Box::new(traverse_derived_replace_base(*to, replacement)),
        },
        DerivedDeclarator::FunctionIdentified {
            identifier_list,
            to,
        } => DerivedDeclarator::FunctionIdentified {
            identifier_list,
            to: Box::new(traverse_derived_replace_base(*to, replacement)),
        },
    }
}

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
    // Binded(Box<Self>),
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
        let mut new_head = DerivedDeclarator::Base;

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

            new_head = self.parse_abstract_declarator();

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

                if self.current_token().t_type == CTokenType::Keyword(CKeyword::STATIC) {
                    static_flag = true;
                    self.advance_idx();
                }

                let qualifiers = self.parse_type_qualifiers();

                if self.current_token().t_type == CTokenType::Keyword(CKeyword::STATIC) {
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

        traverse_derived_replace_base(new_head, base)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ParameterTypeList {
    parameter_list: Vec<Spanned<ParameterDeclaration>>,
    ellipsis: bool,
}

impl CParser {
    pub(crate) fn parse_parameter_type_list(&mut self) -> Spanned<ParameterTypeList> {
        let mut result = ParameterTypeList {
            parameter_list: vec![],
            ellipsis: false,
        };

        unimplemented!()
    }
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

impl CParser{
    pub(crate) fn parse_parameter_decl(&mut self) -> Spanned<ParameterDeclaration>{
        let start = self.current_token().loc;
        let decl = self.parse_declaration_specifiers();
        

        unimplemented!()
    }
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

impl CParser {
    // Stubs for later
    pub(crate) fn parse_struct_or_union_specifier(&mut self) -> Spanned<CStructType> {
        unimplemented!()
    }
    pub(crate) fn parse_enum_specifier(&mut self) -> Spanned<CEnumType> {
        unimplemented!()
    }
}
