use std::cell::RefCell;

use log::debug;

use crate::environment_builder::symbol_table::TypedefInstance;
use crate::environment_builder::symbol_table::VariableInstance;
use crate::parser::parse_nodes::declarations::Declaration;
use crate::parser::parse_nodes::ExternalDeclaration::*;
use crate::parser::parse_nodes::TranslationUnit;
use crate::parser::span::Spanned;

use super::EnvironmentController;

mod static_and_constant_expr;
mod walk_expressions;
mod walk_func;

impl EnvironmentController {
    pub(crate) fn walk_translation_unit(&mut self, translation_unit: TranslationUnit) {
        for external_declaration in translation_unit {
            match &*external_declaration {
                FunctionDefinition(function_def) => self.walk_func(function_def.clone()),
                Declaration(declaration) => self.handle_external_declaration(declaration),
            }
        }
    }
    pub(crate) fn handle_external_declaration(&mut self, declaration: &Spanned<Declaration>) {
        match &*declaration.inner {
            Declaration::Declaration { specifiers, init } => {
                if specifiers.storage.typedef_c {
                    for typedefed_name in init {
                        let extracted_type = self
                            .extract_pretty_type_from_declaration_specifiers_and_derived_declarator(
                                specifiers.clone(),
                                typedefed_name.0.derive.clone(),
                            );
                        self.symbol_table.scope.typedefs.insert(
                            typedefed_name.0.base.identifier.clone(),
                            RefCell::new(TypedefInstance {
                                def_type: extracted_type.clone(),
                            }),
                        );
                        debug!(
                            "Typedef: {:?} -> {:?}",
                            typedefed_name.0.base.identifier, extracted_type
                        );
                    }
                } else {
                    for variable_name in init {
                        let extracted_type = self
                            .extract_pretty_type_from_declaration_specifiers_and_derived_declarator(
                                specifiers.clone(),
                                variable_name.0.derive.clone(),
                            );
                        self.symbol_table.scope.variables.insert(
                            variable_name.0.base.identifier.clone(),
                            RefCell::new(VariableInstance {
                                is_extern: false,
                                usage_counter: 0,
                                associated_type: extracted_type.clone(),
                            }),
                        );
                        debug!(
                            "Variable def: {:?} -> {:?}",
                            variable_name.0.base.identifier, extracted_type
                        );
                    }
                }
            }
            Declaration::StaticAssertDeclaration(static_assert) => {
                self.handle_static_assert(static_assert)
            }
        }
    }
}
