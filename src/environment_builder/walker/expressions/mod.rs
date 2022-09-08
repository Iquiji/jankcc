use super::*;

use crate::{
    environment_builder::{
        ext_type::{ExtType, FunctionParameter, PrettyType},
        EnvironmentController,
    },
    mir::{MIRBlock, MIRConstant, MIRInstruction, MIRSignature, MIRType, MIRValue},
    parser::{parse_nodes::expressions::CExpression, span::Spanned},
};

use super::walk_func::FunctionContext;
/*
for type-checking i will need a wanted type parameter that is optional tho :/ <= Is it?
transformations are generated as needed.

*/
impl EnvironmentController {
    pub(crate) fn walk_expression(
        &mut self,
        ctx: &mut FunctionContext,
        expression: Spanned<CExpression>,
        wanted_type: &PrettyType,
    ) -> MIRValue {
        match &*expression.inner {
            CExpression::Expression(_) => todo!(),
            CExpression::Assignment {
                to_assign: _,
                operator: _,
                value: _,
            } => todo!(),
            CExpression::Ternary {
                condition: _,
                if_true: _,
                tern_else: _,
            } => todo!(),
            CExpression::LogicalOr(_) => todo!(),
            CExpression::LogicalAnd(_) => todo!(),
            CExpression::InclusiveOr(_) => todo!(),
            CExpression::ExlusiveOr(_) => todo!(),
            CExpression::And(_) => todo!(),
            CExpression::Equality {
                left_piece: _,
                equality_op: _,
                right_piece: _,
            } => todo!(),
            CExpression::Relational {
                left_piece: _,
                equality_op: _,
                right_piece: _,
            } => todo!(),
            CExpression::Shift {
                value: _,
                shift_type: _,
                shift_amount: _,
            } => todo!(),
            CExpression::Additive {
                left_value,
                op: _,
                right_value,
            } => {
                let left_value = self.walk_expression(ctx, left_value.clone(), wanted_type);
                let right_value = self.walk_expression(ctx, right_value.clone(), wanted_type);
                let output_value = ctx
                    .mir_function
                    .make_intermediate_value_typed(MIRType::extract_from_pretty_type(wanted_type));
                MIRBlock::ins_instr(
                    &ctx.mir_function.current_block,
                    MIRInstruction::Add(output_value, left_value, right_value),
                );
                output_value
            }
            CExpression::Multiplicative {
                left_value: _,
                op: _,
                right_value: _,
            } => todo!(),
            CExpression::Cast {
                type_name: _,
                value: _,
            } => todo!(),
            CExpression::PrefixIncrement {
                increment_type: _,
                value: _,
            } => todo!(),
            CExpression::Unary {
                unary_op: _,
                value: _,
            } => todo!(),
            CExpression::SizeOf { value: _ } => todo!(),
            CExpression::SizeOfType { type_name: _ } => todo!(),
            CExpression::AlignOfType { type_name: _ } => todo!(),
            CExpression::ArraySubscription { array: _, index: _ } => todo!(),
            CExpression::FunctionCall {
                function,
                arguments,
            } => {
                let (function_type, ident) = match &*function.inner {
                    CExpression::Identifier(ident) => (
                        self.symbol_table.get_top_variable(&ident.identifier),
                        ident.identifier.clone(),
                    ),
                    _ => panic!("TODO: call function not identifier"),
                };

                if let Some(function_type) = function_type {
                    let function_type = function_type.borrow().associated_type.clone();
                    if let ExtType::Function {
                        overextendable,
                        returns,
                        parameters,
                    } = &function_type.inner_type
                    {
                        // this is for variadic functions so if there is a variadic function we can extend the type iter and dont care about overloading the function
                        let temp = vec![FunctionParameter {
                            ident: String::new(),
                            parameter_type: Box::new(ExtType::Void),
                        }];
                        let temp_non_extending = vec![];
                        let iter_extension = if *overextendable {
                            temp.iter().cycle()
                        } else {
                            temp_non_extending.iter().cycle()
                        };

                        // collect all arg values by walking the ast and requiring the given type
                        let mut args = vec![];
                        for (arg, param_type) in arguments
                            .iter()
                            .zip(parameters.iter().chain(iter_extension))
                        {
                            args.push(self.walk_expression(
                                ctx,
                                arg.clone(),
                                &param_type.parameter_type.clone().into_pretty(),
                            ));
                        }

                        // push the actual call and return the MIRValue that results from that :)
                        let output_value = ctx.mir_function.make_intermediate_value_typed(
                            MIRType::extract_from_pretty_type(&returns.into_pretty()),
                        );
                        MIRBlock::ins_instr(
                            &ctx.mir_function.current_block,
                            MIRInstruction::Call(
                                output_value,
                                ident,
                                args,
                                MIRSignature::from_function_pretty_type(&function_type),
                            ),
                        );
                        output_value
                    } else {
                        panic!("cannot make MIR function signature out of not function PrettyType")
                    }
                } else {
                    function.span.error_at_span("function name unknown!");
                    panic!();
                }
            }
            CExpression::DirectMemberAccess {
                to_access: _,
                member: _,
            } => todo!(),
            CExpression::IndirectMemberAccess {
                to_access: _,
                member: _,
            } => todo!(),
            CExpression::PostfixIncrement {
                increment_type: _,
                value: _,
            } => todo!(),
            CExpression::TypeInitializer {
                type_name: _,
                initializer_list: _,
            } => todo!(),
            CExpression::Identifier(ident) => {
                // get local_ref
                let local_ref = *ctx
                    .mir_function
                    .var_name_id_map
                    .get_by_right(&ident.identifier)
                    .unwrap_or_else(|| panic!("using undeclared variable"));
                let _mir_var_type = ctx.mir_function.var_type_map.get(&local_ref).unwrap();
                let var_type = self
                    .symbol_table
                    .get_top_variable(&ident.identifier)
                    .unwrap_or_else(|| panic!("using undeclared variable"))
                    .borrow()
                    .associated_type
                    .clone();

                if &var_type != wanted_type {
                    // todo!(: fix this)
                    if wanted_type == &ExtType::Void.into_pretty(){
                        warn!("wanted type is void, ignoring in current version, subject to rework!");
                        expression.span.error_at_span(&format!("var type different from wanted type!: {:#?} vs {:#?}",var_type,wanted_type));
                    }else{
                        expression.span.error_at_span(&format!("var type different from wanted type!: {:#?} vs {:#?}",var_type,wanted_type));
                        panic!("var type different from wanted type!");
                    }
                }
                // insert load local instruction
                let value_ref = ctx
                    .mir_function
                    .make_intermediate_value_typed(MIRType::extract_from_pretty_type(&var_type));
                MIRBlock::ins_instr(
                    &ctx.mir_function.current_block,
                    MIRInstruction::ReadLocal(value_ref, local_ref),
                );
                println!("ident: '{}' read value_ref: {} ",ident.identifier,value_ref.opaque_ref);
                assert_ne!(MIRValue{ opaque_ref: 0 },MIRValue{ opaque_ref: 2 });
                value_ref
            }
            CExpression::Constant(constant) => match constant {
                crate::parser::parse_nodes::Constant::Number(numberlike) => {
                    let mir_type = MIRType::extract_from_pretty_type(wanted_type);
                    // make intermediate value insert instr to fetch constant number and return the opaque pointer to the value
                    let value_ref = ctx.mir_function.make_intermediate_value_typed(mir_type);
                    MIRBlock::ins_instr(
                        &ctx.mir_function.current_block,
                        MIRInstruction::ConstNum(
                            value_ref,
                            numberlike.from.parse::<i64>().unwrap(),
                            mir_type,
                        ),
                    );
                    value_ref
                }
            },
            CExpression::StringLiteral(literal) => {
                warn!(
                    "wanted type for string literal is: {:?}, unhandled :./",
                    wanted_type
                );
                let mir_type = MIRType::extract_from_pretty_type(wanted_type);

                // get constant and make a ref to the data constant
                let constant = MIRConstant::from_string(literal.value.clone());
                let constant_ref = ctx.mir_function.insert_constant(constant);

                // insert the GetConstDataPtr instr and return the value ref
                let value_ref = ctx.mir_function.make_intermediate_value_typed(mir_type);
                MIRBlock::ins_instr(
                    &ctx.mir_function.current_block,
                    MIRInstruction::GetConstDataPtr(value_ref, constant_ref),
                );
                value_ref
            }
            CExpression::Paranthesised(_) => todo!(),
            CExpression::GenericSelection(_) => todo!(),
        }
    }
}
