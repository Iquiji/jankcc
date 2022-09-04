use crate::parser::parse_nodes::expressions::{CExpression, ConstantExpression};

use super::{CompileTimeValue, EnvironmentController};

impl EnvironmentController {
    pub(crate) fn run_constant_expression(&mut self, expr: ConstantExpression) -> CompileTimeValue {
        self.try_run_expression_at_compile_time((*expr.internal).clone())
    }
    pub(crate) fn try_run_expression_at_compile_time(
        &mut self,
        expr: CExpression,
    ) -> CompileTimeValue {
        match expr {
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
            } => todo!(),
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
            CExpression::Constant(constant) => match constant {
                crate::parser::parse_nodes::Constant::Number(number_string) => {
                    let og = number_string.from;
                    if og.contains(".") {
                        CompileTimeValue::Float(og.parse::<f64>().expect(
                            "failed to convert string to float in constant expression runner",
                        ))
                    } else {
                        CompileTimeValue::Int(og.parse::<i128>().expect(
                            "failed to convert string to int in constant expression runner",
                        ))
                    }
                }
            },
            CExpression::StringLiteral(string_literal) => {
                CompileTimeValue::String(string_literal.value)
            }
            CExpression::Paranthesised(_) => todo!(),
            CExpression::GenericSelection(_) => todo!(),
        }
    }
}
