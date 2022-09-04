use std::{borrow::Borrow, cell::RefCell};

use log::{error, info};

use crate::{
    environment_builder::{symbol_table::VariableInstance, EnvironmentController},
    mir::{GlobalEntity, MIR_Block, MIR_Function, MIR_Instruction, MIR_Location, MIR_Type},
    parser::{
        parse_nodes::{
            statements::{self, Statement},
            FunctionDefinition,
        },
        span::Spanned,
    },
};

impl EnvironmentController {
    pub(crate) fn walk_func(&mut self, func: Spanned<FunctionDefinition>) {
        let extracted_type = self
            .extract_pretty_type_from_declaration_specifiers_and_derived_declarator(
                func.function_specifiers.clone(),
                func.declarator.derive.clone(),
            );
        self.symbol_table.scope.variables.insert(
            func.declarator.base.identifier.clone(),
            RefCell::new(VariableInstance {
                is_extern: false,
                usage_counter: 0,
                associated_type: extracted_type.clone(),
            }),
        );
        info!(
            "Function: {:?} -> {:?}",
            func.declarator.base.identifier, extracted_type
        );

        let mut func_ctx = FunctionContext::new(func.declarator.base.identifier.clone());

        func_ctx.mir_function.blocks.push(MIR_Block {
            instr: vec![],
            branches: None,
        });

        self.walk_statement(&mut func_ctx, func.body.clone());

        let used_vars = self
            .symbol_table
            .scope
            .variables
            .iter()
            .filter(|var| var.1.borrow().usage_counter > 0)
            .map(|var| format!("{}-usage: {}", var.0, var.1.borrow().usage_counter))
            .collect::<Vec<String>>()
            .join("\n");

        info!("all_used_vars: {}", used_vars);

        self.mir_programm.functions.push(func_ctx.mir_function);

        self.mir_programm.globals.extend(
            self.symbol_table
                .scope
                .variables
                .iter()
                .filter(|var| var.1.borrow().usage_counter > 0)
                .map(|extern_var| GlobalEntity {
                    name: extern_var.0.clone(),
                    extern_linkage: true,
                }),
        ) // todo fix this true
    }
}

pub(crate) struct FunctionContext {
    pub(crate) mir_function: MIR_Function,
    pub(crate) temp_counter: usize,
}
impl FunctionContext {
    pub(crate) fn new(name: String) -> FunctionContext {
        FunctionContext {
            mir_function: MIR_Function {
                name,
                blocks: vec![],
            },
            temp_counter: 0,
        }
    }
}

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
                        statements::CompoundItem::Declaration(_) => todo!(),
                    }
                }
            }
            Statement::CExpression(expression) => {
                let _ = self.walk_expression(ctx, expression.clone());
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
                let val = self.try_run_expression_at_compile_time(
                    *expr.clone().expect("NO return value").inner,
                );
                let val = match val {
                    crate::environment_builder::CompileTimeValue::Int(int) => int,
                    crate::environment_builder::CompileTimeValue::Float(_) => todo!(),
                    crate::environment_builder::CompileTimeValue::String(_) => todo!(),
                };
                ctx.mir_function.blocks[0]
                    .instr
                    .push(MIR_Instruction::Return(MIR_Location::Constant(
                        val as i64,
                        MIR_Type::i64,
                    )));
            }
        }
    }
}
