use crate::mir::{MIRBlock, MIRInstruction};
use crate::parser::parse_nodes::expressions::CExpression;

use super::*;

impl EnvironmentController {
    pub(crate) fn handle_return_statement(
        &mut self,
        ctx: &mut FunctionContext,
        return_expr: &Option<Spanned<CExpression>>,
    ) {
        debug!("return statement!");
        if let Some(expr) = return_expr {
            let return_value =
                self.walk_expression(ctx, expr.clone(), &ctx.pretty_return_type.clone());
            MIRBlock::ins_instr(
                &ctx.mir_function.current_block,
                MIRInstruction::Return(return_value),
            );
            ctx.mir_function.current_block.borrow_mut().is_exit_block = true; // no branching from here on out we already returned!
        } else {
            warn!("return without parameter is probably not functioning correctly!");
            // ctx.mir_function.blocks[0]
            //     .instr
            //     .push(MIR_Instruction::Return(MIR_Location::Constant(
            //         0,
            //         MIRType::i64,
            //     )));
        }
    }
}
