use crate::{
    environment_builder::ext_type::ExtType,
    mir::{MIRBlock, MIRBranch, MIRInstruction, MIRType},
    parser::parse_nodes::{
        declarations::{DeclarationSpecifiers, Declarator, Initializer},
        expressions::CExpression,
    },
};

use super::{statements::CompoundItem, *};

impl EnvironmentController {
    pub(crate) fn handle_if_statement(
        &mut self,
        ctx: &mut FunctionContext,
        controlling_expr: &Spanned<CExpression>,
        true_body: &Spanned<Statement>,
        else_body: &Option<Spanned<Statement>>,
    ) {
        // controlling value
        let cond_value = self.walk_expression(
            ctx,
            controlling_expr.clone(),
            &ExtType::Int {
                is_const: false,
                is_volatile: false,
                signed: true,
                size: 4,
            }
            .into_pretty(),
        );

        // current block: need to jump conditionally
        let current_block = ctx.mir_function.current_block.clone();

        // make if_true block
        let if_true_block_id = ctx.mir_function.blocks.len();
        let if_true_block = MIRBlock::new_wrapped();
        ctx.mir_function.blocks.push(if_true_block.clone());

        // set branch to if_true block
        current_block.borrow_mut().branches = Some((
            cond_value,
            vec![MIRBranch {
                is_default: false,
                value_needed: 1,
                to_block: if_true_block_id,
            }],
        ));

        // if else_body make it:
        let mut else_block_id_link = -1;
        if else_body.is_some() {
            let else_block_id = ctx.mir_function.blocks.len();
            else_block_id_link = else_block_id as i32;
            let else_block = MIRBlock::new_wrapped();
            ctx.mir_function.blocks.push(else_block);
        }

        // merge block:
        let merge_block_id = ctx.mir_function.blocks.len();
        let merge_block = MIRBlock::new_wrapped();
        ctx.mir_function.blocks.push(merge_block.clone());

        if else_block_id_link != -1 {
            if let Some(current_block_branches) = &mut current_block.borrow_mut().branches {
                current_block_branches.1.push(MIRBranch {
                    is_default: true,
                    value_needed: 0,
                    to_block: else_block_id_link as usize,
                });
            } else {
                panic!()
            }
        } else if let Some(current_block_branches) = &mut current_block.borrow_mut().branches {
            current_block_branches.1.push(MIRBranch {
                is_default: true,
                value_needed: 0,
                to_block: merge_block_id as usize,
            });
        } else {
            panic!()
        }

        // instr in true block
        ctx.mir_function.current_block = if_true_block;
        self.walk_statement(ctx, true_body.clone());
        // set branch at end of then current block
        if !ctx.mir_function.current_block.borrow().is_exit_block {
            // only if we dont return from that block
            ctx.mir_function.current_block.borrow_mut().branches = Some((
                cond_value,
                vec![MIRBranch {
                    is_default: true,
                    value_needed: 0,
                    to_block: merge_block_id,
                }],
            ));
        }

        // if else_block then do instr in there
        if let Some(else_body) = else_body {
            let else_block = ctx.mir_function.blocks[else_block_id_link as usize].clone();
            ctx.mir_function.current_block = else_block;
            self.walk_statement(ctx, else_body.clone());
            // set branch at end of then current block
            if !ctx.mir_function.current_block.borrow().is_exit_block {
                // only if we dont return from that block
                ctx.mir_function.current_block.borrow_mut().branches = Some((
                    cond_value,
                    vec![MIRBranch {
                        is_default: true,
                        value_needed: 0,
                        to_block: merge_block_id,
                    }],
                ));
            }
        }

        // set merge_block to current
        ctx.mir_function.current_block = merge_block;
    }
}
