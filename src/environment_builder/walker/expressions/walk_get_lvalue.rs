use super::*;

use crate::{
    environment_builder::{
        ext_type::{ExtType, FunctionParameter, PrettyType},
        EnvironmentController,
    },
    mir::{MIRBlock, MIRConstant, MIRInstruction, MIRSignature, MIRType, MIRValue, IntMathKind, MIRLocatorValue},
    parser::{parse_nodes::expressions::CExpression, span::Spanned},
};

use super::walk_func::FunctionContext;
/*
we need a lvalue for things like lvalue = 4;

*/

impl MIRLocatorValue{
    pub(crate) fn into_rvlaue(&self,ctx: &mut FunctionContext) -> MIRValue{
        match self{
            MIRLocatorValue::LocalVar(local_ref,pretty_type) => {
                let mir_var_type = ctx.mir_function.var_type_map.get(local_ref).unwrap();
                let value_ref = ctx
                    .mir_function
                    .make_intermediate_value_typed(*mir_var_type);
                MIRBlock::ins_instr(
                    &ctx.mir_function.current_block,
                    MIRInstruction::ReadLocal(value_ref, *local_ref),
                );
                value_ref
            },
        }
    }
    pub(crate) fn assign_value(&self,ctx: &mut FunctionContext,assign_value: MIRValue){
        match self{
            MIRLocatorValue::LocalVar(local_ref,pretty_type) => {
                MIRBlock::ins_instr(
                    &ctx.mir_function.current_block,
                    MIRInstruction::AssignLocal(*local_ref,assign_value),
                );
            },
        }
    }
    pub(crate) fn get_pretty_type(&self) -> PrettyType{
        match self{
            MIRLocatorValue::LocalVar(local_ref,pretty_type) => {
                pretty_type.clone()
            },
        }
    }
}

impl EnvironmentController {
    pub(crate) fn walk_expression_get_lvalue(
        &mut self,
        ctx: &mut FunctionContext,
        expression: Spanned<CExpression>,
        wanted_type: &PrettyType,
    ) -> MIRLocatorValue {
        match &*expression.inner {
            CExpression::Expression(_) => todo!(),
            CExpression::Assignment { to_assign, operator, value } => todo!(),
            CExpression::Ternary { condition, if_true, tern_else } => todo!(),
            CExpression::LogicalOr(_) => todo!(),
            CExpression::LogicalAnd(_) => todo!(),
            CExpression::InclusiveOr(_) => todo!(),
            CExpression::ExlusiveOr(_) => todo!(),
            CExpression::And(_) => todo!(),
            CExpression::Equality { left_piece, equality_op, right_piece } => todo!(),
            CExpression::Relational { left_piece, equality_op, right_piece } => todo!(),
            CExpression::Shift { value, shift_type, shift_amount } => todo!(),
            CExpression::Additive { left_value, op, right_value } => todo!(),
            CExpression::Multiplicative { left_value, op, right_value } => todo!(),
            CExpression::Cast { type_name, value } => todo!(),
            CExpression::PrefixIncrement { increment_type, value } => todo!(),
            CExpression::Unary { unary_op, value } => todo!(),
            CExpression::SizeOf { value } => todo!(),
            CExpression::SizeOfType { type_name } => todo!(),
            CExpression::AlignOfType { type_name } => todo!(),
            CExpression::ArraySubscription { array, index } => todo!(),
            CExpression::FunctionCall { function, arguments } => todo!(),
            CExpression::DirectMemberAccess { to_access, member } => todo!(),
            CExpression::IndirectMemberAccess { to_access, member } => todo!(),
            CExpression::PostfixIncrement { increment_type, value } => todo!(),
            CExpression::TypeInitializer { type_name, initializer_list } => todo!(),
            CExpression::Identifier(ident) => {
                let local_ref = *ctx
                    .mir_function
                    .var_name_id_map
                    .get_by_right(&ident.identifier)
                    .unwrap_or_else(|| panic!("using undeclared variable"));

                let var_type = self
                    .symbol_table
                    .get_top_variable(&ident.identifier)
                    .unwrap_or_else(|| panic!("using undeclared variable"))
                    .borrow()
                    .associated_type
                    .clone();
                MIRLocatorValue::LocalVar(local_ref,var_type)
            },
            CExpression::Constant(_) => todo!(),
            CExpression::StringLiteral(_) => todo!(),
            CExpression::Paranthesised(_) => todo!(),
            CExpression::GenericSelection(_) => todo!(),
        }
    }
}