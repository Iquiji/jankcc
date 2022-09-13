use std::{borrow::Borrow, cell::RefCell};

use log::{debug, info};

use crate::{
    environment_builder::{
        ext_type::{ExtType, PrettyType},
        symbol_table::VariableInstance,
        EnvironmentController,
    },
    mir::{GlobalEntity, MIRFunction, MIRSignature, MIRType},
    parser::{parse_nodes::FunctionDefinition, span::Spanned},
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

        let mut func_ctx = FunctionContext::new();
        func_ctx.mir_function.name = func.declarator.base.identifier.clone();

        func_ctx.mir_function.signature = MIRSignature::from_function_pretty_type(&extracted_type);
        if let ExtType::Function {
            overextendable: _,
            returns,
            parameters,
        } = &extracted_type.inner_type
        {
            for parameter_name in parameters {
                // param name for later
                func_ctx
                    .mir_function
                    .parameter_names
                    .push(parameter_name.ident.clone());
                // param is also a local variable
                func_ctx.mir_function.insert_variable(
                    parameter_name.ident.clone(),
                    parameter_name.parameter_type.into_pretty(),
                );
                // it is also in the symbol table
                self.symbol_table.get_current_scope().variables.insert(
                    parameter_name.ident.clone(),
                    RefCell::new(VariableInstance {
                        is_extern: false,
                        usage_counter: 0,
                        associated_type: parameter_name.parameter_type.into_pretty(),
                    }),
                );
            }
            func_ctx.pretty_return_type = returns.into_pretty();
        } else {
            panic!("cannot make MIR function signature out of not function PrettyType")
        }

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

        debug!(
            "{:#?}",
            func_ctx.mir_function.ctx_gen.intermediate_value_counter
        );
        debug!("{:#?}", func_ctx.mir_function.value_type_map);

        self.mir_programm.functions.push(func_ctx.mir_function);

        // TODO: move this
        self.mir_programm.globals.extend(
            self.symbol_table
                .scope
                .variables
                .iter()
                .filter(|var| var.1.borrow().usage_counter > 0)
                .map(|extern_var| GlobalEntity {
                    name: extern_var.0.clone(),
                    extern_linkage: extern_var.1.borrow().is_extern,
                }),
        );
    }
}

pub(crate) struct FunctionContext {
    pub(crate) mir_function: MIRFunction,
    pub(crate) pretty_return_type: PrettyType,
}
impl FunctionContext {
    pub(crate) fn new() -> FunctionContext {
        FunctionContext {
            mir_function: MIRFunction::new(),
            pretty_return_type: PrettyType::default_void(),
        }
    }
}
