/*
Qualifiers?

*/

use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};

use crate::parser::parse_nodes::declarations::{
    CFunctionSpecifier, CStorageClass, DeclarationSpecifiers, DerivedDeclarator,
};

use super::{CompileTimeValue, EnvironmentController};

/// A transformed Type from the Parser
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum ExtType {
    Void,
    Int {
        is_const: bool,
        is_volatile: bool,
        signed: bool,
        size: u64,
    },
    Float {
        is_const: bool,
        is_volatile: bool,
        size: u64,
    },
    Array {
        is_const: bool,
        is_volatile: bool,
        arr_size: Option<u64>,
        to: Box<ExtType>,
    },
    Pointer {
        is_const: bool,
        is_volatile: bool,
        to: Box<ExtType>,
    },
    Function {
        /// ,... syntax for function overloading
        overextendable: bool,
        returns: Box<ExtType>,
        parameters: Vec<FunctionParameter>,
    },
    Struct {
        is_const: bool,
        is_volatile: bool,
        tag: Option<String>,
        alignment: Option<u64>,
        members: Vec<StructOrUnionMember>,
    },
    Union {
        is_const: bool,
        is_volatile: bool,
        tag: Option<String>,
        members: Vec<StructOrUnionMember>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct StructOrUnionMember {
    ident: String,
    member_type: Box<ExtType>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct FunctionParameter {
    pub(crate) ident: String,
    pub(crate) parameter_type: Box<ExtType>,
}

impl EnvironmentController {
    pub(crate) fn extract_base_ext_type_from_declaration_specifiers(
        &mut self,
        decl_spec: DeclarationSpecifiers,
    ) -> ExtType {
        use crate::parser::types::CTypeSpecifier::*;
        match decl_spec.specifiers {
            Basic(basic) => {
                // Convert into Size and signed instead of naming
                let (size, signed, is_void, is_float): (u64, bool, bool, bool) = match basic {
                    crate::parser::types::CBasicTypes::Void => (0, false, true, false),
                    crate::parser::types::CBasicTypes::Char => (1, false, false, false),
                    crate::parser::types::CBasicTypes::SignedChar => (1, true, false, false),
                    crate::parser::types::CBasicTypes::UnsignedChar => (1, false, false, false),
                    crate::parser::types::CBasicTypes::Short => (2, true, false, false),
                    crate::parser::types::CBasicTypes::UnShort => (2, false, false, false),
                    crate::parser::types::CBasicTypes::Int => (4, true, false, false),
                    crate::parser::types::CBasicTypes::UnInt => (4, false, false, false),
                    crate::parser::types::CBasicTypes::Long => (8, true, false, false),
                    crate::parser::types::CBasicTypes::UnLong => (8, false, false, false),
                    crate::parser::types::CBasicTypes::LongLong => (8, true, false, false),
                    crate::parser::types::CBasicTypes::UnLongLong => (8, false, false, false),
                    crate::parser::types::CBasicTypes::Float => (4, false, false, true),
                    crate::parser::types::CBasicTypes::Double => (8, false, false, true),
                    crate::parser::types::CBasicTypes::LongDouble => (8, false, false, true),
                    crate::parser::types::CBasicTypes::Bool => (1, false, false, false),
                    crate::parser::types::CBasicTypes::FloatComplex => {
                        error!("PrettyType does not support Complex Numbers");
                        panic!("PrettyType does not support Complex Numbers!")
                    }
                    crate::parser::types::CBasicTypes::DoubleComplex => {
                        error!("PrettyType does not support Complex Numbers");
                        panic!("PrettyType does not support Complex Numbers!")
                    }
                    crate::parser::types::CBasicTypes::LongDoubleComplex => {
                        error!("PrettyType does not support Complex Numbers");
                        panic!("PrettyType does not support Complex Numbers!")
                    }
                };
                if is_void {
                    return ExtType::Void;
                }
                if !is_float {
                    ExtType::Int {
                        is_const: decl_spec.qualifiers.const_q,
                        is_volatile: decl_spec.qualifiers.volatile_q,
                        signed,
                        size,
                    }
                } else {
                    ExtType::Float {
                        is_const: decl_spec.qualifiers.const_q,
                        is_volatile: decl_spec.qualifiers.volatile_q,
                        size,
                    }
                }
            }
            // TODO: Tags must be compatible in scope => _IO_FILE gets completed :)
            StructOrUnion(struct_or_union) => match struct_or_union.struct_type {
                crate::parser::types::CStructOrUnionTypeType::Struct => ExtType::Struct {
                    is_const: decl_spec.qualifiers.const_q,
                    is_volatile: decl_spec.qualifiers.volatile_q,
                    tag: struct_or_union
                        .ident
                        .as_ref()
                        .map(|tag| tag.identifier.clone()),
                    members: {
                        let mut collector = vec![];

                        for member in &struct_or_union.declarations {
                            match member{
                                crate::parser::types::CSructDeclaration::StaticAssertDeclaration(static_assert_decl) => self.handle_static_assert(static_assert_decl),
                                crate::parser::types::CSructDeclaration::StructDeclaration { specifier_qualifier, delcarator_list } => {
                                    for decl in delcarator_list {
                                        let true_decl = match &*decl.inner {
                                            crate::parser::types::CStructDeclarator::Declarator(decl) => decl,
                                            crate::parser::types::CStructDeclarator::BitField { declarator: _, expr: _ } => panic!("bit fields unsupported"),
                                        };
                                        let extracted_type = self
                                            .extract_pretty_type_from_declaration_specifiers_and_derived_declarator(
                                                DeclarationSpecifiers { storage: CStorageClass{ typedef_c: false, extern_c: false, static_c: false, thread_local_c: false, auto_c: false, register_c: false }, qualifiers: specifier_qualifier.qualifiers.clone(), specifiers: specifier_qualifier.specifier.clone(), function: CFunctionSpecifier{ inline: false, no_return: false }, alignment: None },
                                                true_decl.derive.clone(),
                                            );
                                        collector.push(StructOrUnionMember { ident: true_decl.base.identifier.clone(), member_type: Box::new(extracted_type.inner_type) });
                                    }
                                },
                            }
                        }

                        collector
                    },
                    alignment: None, //TODO!
                },
                crate::parser::types::CStructOrUnionTypeType::Union => ExtType::Union {
                    is_const: decl_spec.qualifiers.const_q,
                    is_volatile: decl_spec.qualifiers.volatile_q,
                    tag: struct_or_union
                        .ident
                        .as_ref()
                        .map(|tag| tag.identifier.clone()),
                    members: {
                        let mut collector = vec![];

                        for member in &struct_or_union.declarations {
                            match member{
                                crate::parser::types::CSructDeclaration::StaticAssertDeclaration(static_assert_decl) => self.handle_static_assert(static_assert_decl),
                                crate::parser::types::CSructDeclaration::StructDeclaration { specifier_qualifier, delcarator_list } => {
                                    for decl in delcarator_list {
                                        let true_decl = match &*decl.inner {
                                            crate::parser::types::CStructDeclarator::Declarator(decl) => decl,
                                            crate::parser::types::CStructDeclarator::BitField { declarator: _, expr: _ } => panic!("bit fields unsupported"),
                                        };
                                        let extracted_type = self
                                            .extract_pretty_type_from_declaration_specifiers_and_derived_declarator(
                                                DeclarationSpecifiers { storage: CStorageClass{ typedef_c: false, extern_c: false, static_c: false, thread_local_c: false, auto_c: false, register_c: false }, qualifiers: specifier_qualifier.qualifiers.clone(), specifiers: specifier_qualifier.specifier.clone(), function: CFunctionSpecifier{ inline: false, no_return: false }, alignment: None },
                                                true_decl.derive.clone(),
                                            );
                                        collector.push(StructOrUnionMember { ident: true_decl.base.identifier.clone(), member_type: Box::new(extracted_type.inner_type) });
                                    }
                                },
                            }
                        }

                        collector
                    },
                },
            },
            Enum(_) => todo!("enum Currently not implemented"),
            Typedefed(typedef_name) => {
                if let Some(typedef_instance) = self
                    .symbol_table
                    .get_top_typedefed(&typedef_name.identifier)
                {
                    typedef_instance.borrow().def_type.clone().inner_type
                } else {
                    panic!("typedef not found: {}", typedef_name.identifier);
                }
            }
            Atomic(_) => todo!("Atomic not supported"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct PrettyType {
    pub(crate) inner_type: ExtType,
}

impl EnvironmentController {
    pub(crate) fn extract_pretty_type_from_declaration_specifiers_and_derived_declarator(
        &mut self,
        decl_spec: DeclarationSpecifiers,
        derived_decl: DerivedDeclarator,
    ) -> PrettyType {
        let base = self.extract_base_ext_type_from_declaration_specifiers(decl_spec);

        PrettyType {
            inner_type: self.handle_derived_declarator_for_pretty_type(base, derived_decl),
        }
    }

    pub(crate) fn handle_derived_declarator_for_pretty_type(
        &mut self,
        wrap_around: ExtType,
        derived: DerivedDeclarator,
    ) -> ExtType {
        match derived {
            DerivedDeclarator::Base => wrap_around,
            DerivedDeclarator::Pointer { qualifiers, to } => ExtType::Pointer {
                is_const: qualifiers.const_q,
                is_volatile: qualifiers.volatile_q,
                to: Box::new(self.handle_derived_declarator_for_pretty_type(wrap_around, *to)),
            },
            DerivedDeclarator::Array {
                qualifiers,
                is_static,
                size_expr,
                vla,
                to,
            } => {
                if is_static {
                    debug!("static in array ignored");
                }
                if vla {
                    warn!("VLA unsupported, ignored in prettying type");
                }

                let arr_size: Option<u64> = if let Some(size_expr) = size_expr {
                    let const_value = self.try_run_expression_at_compile_time((*size_expr).clone());
                    if let CompileTimeValue::Int(val) = const_value {
                        Some(val as u64)
                    } else {
                        size_expr.span.error_at_span("arr_size is no integer!");
                        panic!();
                    }
                } else {
                    None
                };

                ExtType::Array {
                    is_const: qualifiers.const_q,
                    is_volatile: qualifiers.volatile_q,
                    arr_size,
                    to: Box::new(self.handle_derived_declarator_for_pretty_type(wrap_around, *to)),
                }
            }
            DerivedDeclarator::FunctionType {
                parameter_type_list,
                to,
            } => ExtType::Function {
                overextendable: (*parameter_type_list.inner).ellipsis,
                returns: Box::new(self.handle_derived_declarator_for_pretty_type(wrap_around, *to)),
                parameters: (*parameter_type_list.inner)
                    .parameter_list
                    .iter()
                    .map(|parameter| {
                        use crate::parser::parse_nodes::declarations::ParameterDeclaration::*;
                        match &(*parameter.inner){
                            Declarator { specifiers, declarator } => {
                                let para_type = self.extract_pretty_type_from_declaration_specifiers_and_derived_declarator(specifiers.clone(),declarator.derive.clone());
                                let name = declarator.base.identifier.clone();
                                FunctionParameter{
                                    ident: name,
                                    parameter_type: Box::new(para_type.inner_type),
                                }
                            },
                            AbstractDeclarator { specifiers, abstract_declarator } => {
                                // TODO: this is a hack!
                                info!("AbstractDeclarator, unnamed arg in Function is not functional!");
                                let para_type = if let Some(declarator) = abstract_declarator{
                                    self.extract_pretty_type_from_declaration_specifiers_and_derived_declarator(specifiers.clone(),declarator.clone()).inner_type
                                } else{
                                    self.extract_base_ext_type_from_declaration_specifiers(specifiers.clone())
                                };
                                // let name = declarator.base.identifier.clone();
                                FunctionParameter{
                                    ident: String::from("__anon__"),
                                    parameter_type: Box::new(para_type),
                                }
                            },
                        }
                    }).collect(),
            },
            DerivedDeclarator::FunctionIdentified {
                identifier_list: _,
                to: _,
            } => unimplemented!("DerivedDeclarator::FunctionIdentified unsupported currently :/"),
        }
    }
}
