use super::{*, walk_func::FunctionContext};
use crate::{
    environment_builder::{
        ext_type::{ExtType, PrettyType},
        symbol_table::VariableInstance,
        EnvironmentController,
    },
    mir::{GlobalEntity, MIRBlock, MIRFunction, MIRInstruction, MIRSignature, MIRType},
    parser::{
        parse_nodes::{
            statements::{self, Statement},
            FunctionDefinition,
        },
        span::Spanned,
    },
};

impl EnvironmentController {
    pub(crate) fn walk_statement(
        &mut self,
        ctx: &mut FunctionContext,
        statement: Spanned<Statement>,
    ) {
        println!("{}", serde_yaml::to_string(&statement).unwrap());
        match &*statement.inner {
            Statement::Labeled { label, body } => todo!(),
            Statement::SwitchCase {
                const_expr,
                statement,
            } => todo!(),
            Statement::SwitchDefault { statement } => todo!(),
            Statement::Compound(compound_statement_list) => {
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
                                        let var_name =
                                            var_that_is_declared.0.base.identifier.clone();
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
                                                    let expr_result = self.walk_expression(
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
            }
            Statement::CExpression(expression) => {
                let _ = self.walk_expression(ctx, expression.clone(), &PrettyType::default_void());
            }
            Statement::NoneExpr => {}
            Statement::If {
                controlling_expr,
                true_body,
                else_body,
            } => todo!(),
            Statement::Switch {
                controlling_expr,
                body,
            } => todo!(),
            Statement::While {
                while_type,
                controlling_expr,
                body,
            } => todo!(),
            Statement::For {
                decl_clause,
                expr_clause,
                controlling_expr,
                after_expr,
                body,
            } => todo!(),
            Statement::Goto(_) => todo!(),
            Statement::Continue => todo!(),
            Statement::Break => todo!(),
            Statement::Return(expr) => {
                // if let Some(expr) = &expr {
                //     let return_location =
                //         self.walk_expression(ctx, expr.clone(), &ctx.pretty_return_type.clone());
                //     ctx.mir_function.blocks[0]
                //         .instr
                //         .push(MIR_Instruction::Return(return_location));
                // } else {
                //     warn!("return without parameter is probably not functioning correctly!");
                //     ctx.mir_function.blocks[0]
                //         .instr
                //         .push(MIR_Instruction::Return(MIR_Location::Constant(
                //             0,
                //             MIRType::i64,
                //         )));
                // }
            }
        }
    }
}
