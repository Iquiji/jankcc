use super::*;

use crate::{
    environment_builder::{
        ext_type::{ExtType, FunctionParameter, PrettyType},
        EnvironmentController,
    },
    mir::{IntMathKind, MIRBlock, MIRConstant, MIRInstruction, MIRSignature, MIRType, MIRValue},
    parser::{
        parse_nodes::{
            declarations::{CFunctionSpecifier, CStorageClass, DeclarationSpecifiers},
            expressions::CExpression,
        },
        span::Spanned,
    },
};

mod walk_get_lvalue;

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
                to_assign,
                operator,
                value,
            } => {
                let lvalue = self.walk_expression_get_lvalue(ctx, to_assign.clone(), wanted_type);

                let rvalue = self.walk_expression(ctx, value.clone(), &lvalue.get_pretty_type());
                lvalue.assign_value(ctx, rvalue);
                lvalue.into_rvlaue(ctx)
            }
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
                left_piece,
                equality_op,
                right_piece,
            } => {
                let left_value = self.walk_expression(ctx, left_piece.clone(), wanted_type);
                let right_value = self.walk_expression(ctx, right_piece.clone(), wanted_type);
                let (left_value, right_value) =
                    self.arithmatic_conversion(ctx, left_value, right_value);

                // Equality always returns an int
                let output_value = ctx.mir_function.make_intermediate_value_typed(
                    ExtType::Int {
                        is_const: false,
                        is_volatile: false,
                        signed: true,
                        size: 4,
                    }
                    .into_pretty(),
                );
                MIRBlock::ins_instr(
                    &ctx.mir_function.current_block,
                    MIRInstruction::Compare(
                        output_value,
                        left_value,
                        right_value,
                        match equality_op {
                            crate::parser::parse_nodes::expressions::EqualityOperator::Equal => {
                                crate::mir::IntCmpKind::Eq
                            }
                            crate::parser::parse_nodes::expressions::EqualityOperator::NotEqual => {
                                crate::mir::IntCmpKind::UnEq
                            }
                        },
                    ),
                );
                output_value
            }
            CExpression::Relational {
                left_piece,
                equality_op,
                right_piece,
            } => {
                let left_value = self.walk_expression(ctx, left_piece.clone(), wanted_type);
                let right_value = self.walk_expression(ctx, right_piece.clone(), wanted_type);
                let (left_value, right_value) =
                    self.arithmatic_conversion(ctx, left_value, right_value);

                // Relational always returns an int
                let output_value = ctx.mir_function.make_intermediate_value_typed(
                    ExtType::Int {
                        is_const: false,
                        is_volatile: false,
                        signed: true,
                        size: 4,
                    }
                    .into_pretty(),
                );
                MIRBlock::ins_instr(
                    &ctx.mir_function.current_block,
                    MIRInstruction::Compare(output_value, left_value, right_value,match equality_op{
                        crate::parser::parse_nodes::expressions::RelationalOperator::Lesser => crate::mir::IntCmpKind::LT,
                        crate::parser::parse_nodes::expressions::RelationalOperator::Greater => crate::mir::IntCmpKind::GT,
                        crate::parser::parse_nodes::expressions::RelationalOperator::LesserEqual => crate::mir::IntCmpKind::LET,
                        crate::parser::parse_nodes::expressions::RelationalOperator::GreaterEqual => crate::mir::IntCmpKind::GET,
                    }),
                );
                output_value
            }
            CExpression::Shift {
                value: _,
                shift_type: _,
                shift_amount: _,
            } => todo!(),
            CExpression::Additive {
                left_value,
                op,
                right_value,
            } => {
                // CExpression Add Op to IntMathKind
                let math_kind = match op {
                    crate::parser::parse_nodes::expressions::AdditiveOperator::Plus => {
                        IntMathKind::Add
                    }
                    crate::parser::parse_nodes::expressions::AdditiveOperator::Minus => {
                        IntMathKind::Sub
                    }
                };
                //
                let left_value = self.walk_expression(ctx, left_value.clone(), wanted_type);
                let right_value = self.walk_expression(ctx, right_value.clone(), wanted_type);
                let (left_value, right_value) =
                    self.arithmatic_conversion(ctx, left_value, right_value);

                let output_value = ctx.mir_function.make_intermediate_value_typed(
                    ctx.mir_function
                        .value_type_map_pretty
                        .get(&left_value)
                        .unwrap()
                        .clone(),
                );
                MIRBlock::ins_instr(
                    &ctx.mir_function.current_block,
                    MIRInstruction::IntMath(output_value, left_value, right_value, math_kind),
                );
                output_value
            }
            CExpression::Multiplicative {
                left_value,
                op,
                right_value,
            } => {
                // CExpression Mul Op to IntMathKind
                let math_kind = match op {
                    crate::parser::parse_nodes::expressions::MultiplicativeOperator::Mult => {
                        IntMathKind::Mul
                    }
                    crate::parser::parse_nodes::expressions::MultiplicativeOperator::Div => {
                        IntMathKind::Div
                    }
                    crate::parser::parse_nodes::expressions::MultiplicativeOperator::Mod => {
                        IntMathKind::Mod
                    }
                };
                //
                let left_value = self.walk_expression(ctx, left_value.clone(), wanted_type);
                let right_value = self.walk_expression(ctx, right_value.clone(), wanted_type);
                let (left_value, right_value) =
                    self.arithmatic_conversion(ctx, left_value, right_value);

                let output_value = ctx.mir_function.make_intermediate_value_typed(
                    ctx.mir_function
                        .value_type_map_pretty
                        .get(&left_value)
                        .unwrap()
                        .clone(),
                );
                MIRBlock::ins_instr(
                    &ctx.mir_function.current_block,
                    MIRInstruction::IntMath(output_value, left_value, right_value, math_kind),
                );
                output_value
            }
            CExpression::Cast { type_name, value } => {
                let target_type = self
                    .extract_pretty_type_from_declaration_specifiers_and_derived_declarator(
                        DeclarationSpecifiers {
                            storage: CStorageClass {
                                typedef_c: false,
                                extern_c: false,
                                static_c: false,
                                thread_local_c: false,
                                auto_c: false,
                                register_c: false,
                            },
                            qualifiers: type_name.inner.base.qualifiers.clone(),
                            specifiers: type_name.inner.base.specifier.clone(),
                            function: CFunctionSpecifier {
                                inline: false,
                                no_return: false,
                            },
                            alignment: None,
                        },
                        *type_name.inner.declarator.inner.clone(),
                    );

                let base = self.walk_expression(ctx, value.clone(), &PrettyType::default_void());

                let output_value = ctx
                    .mir_function
                    .make_intermediate_value_typed(target_type.clone());
                MIRBlock::ins_instr(
                    &ctx.mir_function.current_block,
                    MIRInstruction::IntConvert(
                        output_value,
                        base,
                        MIRType::extract_from_pretty_type(&target_type),
                    ),
                );
                output_value
            }
            CExpression::PrefixIncrement {
                increment_type: _,
                value: _,
            } => todo!(),
            CExpression::Unary { unary_op, value } => {
                use crate::parser::parse_nodes::expressions::*;
                match unary_op {
                    UnaryOperator::REF => {
                        let lvalue = self.walk_expression_get_lvalue(
                            ctx,
                            value.clone(),
                            &ExtType::Void.into_pretty(),
                        );

                        match lvalue {
                            crate::mir::MIRLocatorValue::LocalVar(local_ref, pretty_type) => {
                                let output_value = ctx.mir_function.make_intermediate_value_typed(
                                    ExtType::Pointer {
                                        is_const: false,
                                        is_volatile: false,
                                        to: Box::new(pretty_type.inner_type),
                                    }
                                    .into_pretty(),
                                );
                                MIRBlock::ins_instr(
                                    &ctx.mir_function.current_block,
                                    MIRInstruction::GetAddrOfLocal(output_value, local_ref),
                                );
                                output_value
                            }
                        }
                    }
                    UnaryOperator::DEREF => {
                        let value_to_deref = self.walk_expression(ctx, value.clone(), wanted_type);
                        let type_of_deref = ctx
                            .mir_function
                            .value_type_map_pretty
                            .get(&value_to_deref)
                            .unwrap();
                        if let ExtType::Pointer {
                            is_const,
                            is_volatile,
                            to,
                        } = &type_of_deref.inner_type
                        {
                            let output_value = ctx
                                .mir_function
                                .make_intermediate_value_typed(to.into_pretty());
                            MIRBlock::ins_instr(
                                &ctx.mir_function.current_block,
                                MIRInstruction::Deref(output_value, value_to_deref, MIRType::I32),
                            );
                            output_value
                        } else {
                            expression.span.error_at_span("cannot deref not pointer!");
                            panic!()
                        }
                    }
                    UnaryOperator::VALUE => todo!(),
                    UnaryOperator::NEGATIVE => todo!(),
                    UnaryOperator::BITWISEINVERT => todo!(),
                    UnaryOperator::BOOLEANINVERT => todo!(),
                }
            }
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
                            parameter_type: Box::new(ExtType::Int {
                                is_const: false,
                                is_volatile: false,
                                signed: true,
                                size: 4,
                            }), // todo!(this needs better default handling or something if we dont have a wanted type);
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
                        let output_value = ctx
                            .mir_function
                            .make_intermediate_value_typed(returns.into_pretty());
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
                    if wanted_type == &ExtType::Void.into_pretty() {
                        warn!(
                            "wanted type is void, ignoring in current version, subject to rework!"
                        );
                        expression.span.error_at_span(&format!(
                            "var type different from wanted type!: {:#?} vs {:#?}",
                            var_type, wanted_type
                        ));
                    } else {
                        expression.span.error_at_span(&format!(
                            "var type different from wanted type!: {:#?} vs {:#?}",
                            var_type, wanted_type
                        ));
                        // panic!("var type different from wanted type!");
                    }
                }
                // insert load local instruction
                let value_ref = ctx.mir_function.make_intermediate_value_typed(var_type);
                MIRBlock::ins_instr(
                    &ctx.mir_function.current_block,
                    MIRInstruction::ReadLocal(value_ref, local_ref),
                );
                info!(
                    "ident: '{}' read value_ref: {} ",
                    ident.identifier, value_ref.opaque_ref
                );
                assert_ne!(MIRValue { opaque_ref: 0 }, MIRValue { opaque_ref: 2 });
                value_ref
            }
            CExpression::Constant(constant) => match constant {
                crate::parser::parse_nodes::Constant::Number(numberlike) => {
                    let mir_type = MIRType::extract_from_pretty_type(wanted_type);
                    // make intermediate value insert instr to fetch constant number and return the opaque pointer to the value
                    let value_ref = ctx
                        .mir_function
                        .make_intermediate_value_typed(wanted_type.clone());
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
                let value_ref = ctx
                    .mir_function
                    .make_intermediate_value_typed(wanted_type.clone());
                MIRBlock::ins_instr(
                    &ctx.mir_function.current_block,
                    MIRInstruction::GetConstDataPtr(value_ref, constant_ref),
                );
                value_ref
            }
            CExpression::Paranthesised(expr) => {
                self.walk_expression(ctx, expr.clone(), wanted_type)
            }
            CExpression::GenericSelection(_) => todo!(),
        }
    }
}
