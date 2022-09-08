use crate::{
    environment_builder::{
        ext_type::{ExtType, FunctionParameter, PrettyType},
        EnvironmentController,
    },
    mir::{MIRValue},
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
        _wanted_type: &PrettyType,
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
                left_value: _,
                op: _,
                right_value: _,
            } => {
                todo!()
                // let left_value = self.walk_expression(ctx, left_value.clone(), wanted_type);
                // let right_value = self.walk_expression(ctx, right_value.clone(), wanted_type);
                // let return_loc = ctx
                //     .mir_function
                //     .make_temp_location(left_value.get_mir_type());
                // ctx.mir_function.blocks[0]
                //     .instr
                //     .push(mir::MIR_Instruction::Add(
                //         return_loc.clone(),
                //         left_value,
                //         right_value,
                //     ));
                // return_loc
            }
            CExpression::Multiplicative {
                left_value: _,
                op: _,
                right_value: _,
            } => todo!(),
            CExpression::Cast { type_name: _, value: _ } => todo!(),
            CExpression::PrefixIncrement {
                increment_type: _,
                value: _,
            } => todo!(),
            CExpression::Unary { unary_op: _, value: _ } => todo!(),
            CExpression::SizeOf { value: _ } => todo!(),
            CExpression::SizeOfType { type_name: _ } => todo!(),
            CExpression::AlignOfType { type_name: _ } => todo!(),
            CExpression::ArraySubscription { array: _, index: _ } => todo!(),
            CExpression::FunctionCall {
                function,
                arguments,
            } => {
                let (function_type, _ident) = match &*function.inner {
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
                        returns: _,
                        parameters,
                    } = &function_type.inner_type
                    {
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

                        // let location = ctx.mir_function.make_temp_location(
                        //     MIR_Type::extract_from_pretty_type(&PrettyType {
                        //         inner_type: *returns.clone(),
                        //     }),
                        // );
                        // ctx.mir_function.blocks[0]
                        //     .instr
                        //     .push(mir::MIR_Instruction::Call(
                        //         location.clone(),
                        //         ident,
                        //         args,
                        //         MIR_Signature::from_function_pretty_type(&function_type),
                        //     ));
                        todo!()
                    } else {
                        panic!("cannot make MIR function signature out of not function PrettyType")
                    }
                } else {
                    function.span.error_at_span("function name unknown!");
                    panic!();
                }
            }
            CExpression::DirectMemberAccess { to_access: _, member: _ } => todo!(),
            CExpression::IndirectMemberAccess { to_access: _, member: _ } => todo!(),
            CExpression::PostfixIncrement {
                increment_type: _,
                value: _,
            } => todo!(),
            CExpression::TypeInitializer {
                type_name: _,
                initializer_list: _,
            } => todo!(),
            CExpression::Identifier(_ident) => todo!(),
            CExpression::Constant(constant) => match constant {
                crate::parser::parse_nodes::Constant::Number(_numberlike) => {
                    todo!()
                }
            },
            CExpression::StringLiteral(_literal) => {
                todo!()
                // let mut value = vec![];
                // let char_iter = literal.value.chars();

                // for character in char_iter {
                //     // println!("{}",character);
                //     if character == '\n' {
                //         value.push(0xA);
                //     } else {
                //         value.extend(character.to_string().as_bytes());
                //     }
                // }

                // value.push(0);
                // self.mir_programm.constants.push(mir::Constant { value });
                // MIR_Location::ConstantLocation(self.mir_programm.constants.len() - 1, MIR_Type::i64)
            }
            CExpression::Paranthesised(_) => todo!(),
            CExpression::GenericSelection(_) => todo!(),
        }
    }
}
