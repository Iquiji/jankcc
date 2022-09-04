use crate::{
    environment_builder::EnvironmentController,
    mir::{self, MIR_Location, MIR_Type},
    parser::{parse_nodes::expressions::CExpression, span::Spanned},
};

use super::walk_func::FunctionContext;

impl EnvironmentController {
    pub(crate) fn walk_expression(
        &mut self,
        ctx: &mut FunctionContext,
        expression: Spanned<CExpression>,
    ) -> MIR_Location {
        match &*expression.inner {
            CExpression::Expression(_) => todo!(),
            CExpression::Assignment {
                to_assign,
                operator,
                value,
            } => todo!(),
            CExpression::Ternary {
                condition,
                if_true,
                tern_else,
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
            } => todo!(),
            CExpression::Relational {
                left_piece,
                equality_op,
                right_piece,
            } => todo!(),
            CExpression::Shift {
                value,
                shift_type,
                shift_amount,
            } => todo!(),
            CExpression::Additive {
                left_value,
                op,
                right_value,
            } => todo!(),
            CExpression::Multiplicative {
                left_value,
                op,
                right_value,
            } => todo!(),
            CExpression::Cast { type_name, value } => todo!(),
            CExpression::PrefixIncrement {
                increment_type,
                value,
            } => todo!(),
            CExpression::Unary { unary_op, value } => todo!(),
            CExpression::SizeOf { value } => todo!(),
            CExpression::SizeOfType { type_name } => todo!(),
            CExpression::AlignOfType { type_name } => todo!(),
            CExpression::ArraySubscription { array, index } => todo!(),
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
                    let mut args = vec![];
                    for arg in arguments {
                        args.push(self.walk_expression(ctx, arg.clone()));
                    }
                    // TODO fix type here as well :)
                    let location =
                        MIR_Location::Local(format!("t{}", ctx.temp_counter), MIR_Type::i64);
                    ctx.temp_counter += 1;
                    ctx.mir_function.blocks[0]
                        .instr
                        .push(mir::MIR_Instruction::Call(location.clone(), ident, args));
                    location
                } else {
                    function.span.error_at_span("function name unknown!");
                    panic!();
                }
            }
            CExpression::DirectMemberAccess { to_access, member } => todo!(),
            CExpression::IndirectMemberAccess { to_access, member } => todo!(),
            CExpression::PostfixIncrement {
                increment_type,
                value,
            } => todo!(),
            CExpression::TypeInitializer {
                type_name,
                initializer_list,
            } => todo!(),
            CExpression::Identifier(_) => todo!(),
            CExpression::Constant(_) => todo!(),
            CExpression::StringLiteral(literal) => {
                let mut value = literal.value.as_bytes().to_vec();
                value.push(0);
                self.mir_programm.constants.push(mir::Constant { value });
                MIR_Location::ConstantLocation(self.mir_programm.constants.len() - 1, MIR_Type::i64)
            }
            CExpression::Paranthesised(_) => todo!(),
            CExpression::GenericSelection(_) => todo!(),
        }
    }
}
