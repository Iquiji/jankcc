use log::debug;

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
            } => todo!(),
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
                function: _,
                arguments: _,
            } => todo!(),
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
            CExpression::Identifier(_) => todo!(),
            CExpression::Constant(constant) => match constant {
                crate::parser::parse_nodes::Constant::Number(number_string) => {
                    let og = number_string.from;
                    if og.contains(".") {
                        CompileTimeValue::Float(og.parse::<f64>().expect(
                            "failed to convert string to float in constant expression runner",
                        ))
                    } else {
                        let og_trimmed = og
                            .trim_end_matches('l')
                            .trim_end_matches('L')
                            .trim_end_matches('l')
                            .trim_end_matches('L')
                            .trim_end_matches('u')
                            .trim_end_matches('U');
                        debug!("og: {},og_trimmed: {}", og, og_trimmed);
                        CompileTimeValue::Int(og_trimmed.parse::<i128>().expect(
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
