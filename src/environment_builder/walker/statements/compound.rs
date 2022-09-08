use super::{statements::CompoundItem, *};

impl EnvironmentController {
    pub(crate) fn handle_compound_statement(
        &mut self,
        ctx: &mut FunctionContext,
        compound_statement_list: &[CompoundItem],
    ) {
        self.symbol_table.enter_new_level();
        for statement in compound_statement_list {
            match statement {
                statements::CompoundItem::Statement(statement) => {
                    self.walk_statement(ctx, statement.clone())
                }
                statements::CompoundItem::Declaration(declaration) => {
                    use crate::parser::parse_nodes::declarations::*;
                    match &*declaration.inner {
                        Declaration::Declaration { specifiers, init } => {
                            // actual declaration
                            for var_that_is_declared in init {
                                let extracted_type = self
                                    .extract_pretty_type_from_declaration_specifiers_and_derived_declarator(
                                        specifiers.clone(),
                                        var_that_is_declared.0.derive.clone(),
                                    );
                                let var_name = var_that_is_declared.0.base.identifier.clone();
                                self.symbol_table.get_current_scope().variables.insert(
                                    var_name.clone(),
                                    RefCell::new(VariableInstance {
                                        is_extern: specifiers.storage.extern_c,
                                        usage_counter: 0,
                                        associated_type: extracted_type.clone(),
                                    }),
                                );
                                // ctx.mir_function.vars.push((
                                //     var_name.clone(),
                                //     MIRType::extract_from_pretty_type(&extracted_type),
                                // ));
                                debug!(
                                    "Variable decl as Compound Item: {:?} -> {:?}",
                                    var_that_is_declared.0.base.identifier, extracted_type
                                );
                                if let Some(initializer) = &var_that_is_declared.1 {
                                    match &*initializer.inner {
                                        Initializer::Single(single) => {
                                            let _expr_result = self.walk_expression(
                                                ctx,
                                                single.clone(),
                                                &extracted_type,
                                            );
                                            // ctx.mir_function.blocks[0].instr.push(
                                            //     MIRInstruction::Assign(
                                            //         MIRLocation::Local(
                                            //             var_name.clone(),
                                            //             MIRType::extract_from_pretty_type(
                                            //                 &extracted_type,
                                            //             ),
                                            //         ),
                                            //         expr_result,
                                            //     ),
                                            // );
                                        }
                                        Initializer::Compound(_) => todo!(),
                                    }
                                }
                            }
                        }
                        Declaration::StaticAssertDeclaration(static_assert) => {
                            self.handle_static_assert(static_assert)
                        }
                    }
                }
            }
        }
        self.symbol_table.exit_new_level();
    }
}
