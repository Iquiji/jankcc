use crate::{
    environment_builder::ext_type::ExtType,
    mir::{MIRBlock, MIRBranch, MIRInstruction, MIRType, MIRValue},
    parser::parse_nodes::{
        declarations::{DeclarationSpecifiers, Declarator, Initializer},
        expressions::CExpression,
    },
};

use super::{statements::CompoundItem, *};

impl EnvironmentController {
    pub(crate) fn handle_for_statement(
        &mut self,
        ctx: &mut FunctionContext,
        decl_clause: &Option<Spanned<Declaration>>,
        expr_clause: &Option<Spanned<CExpression>>,
        controlling_expr: &Option<Spanned<CExpression>>,
        after_expr: &Option<Spanned<CExpression>>,
        body: &Spanned<Statement>,
    ) {
        // if decl_clause it is a initializiation before the loop startss
        if let Some(decl_clause) = decl_clause {
            match &*decl_clause.inner {
                Declaration::Declaration { specifiers, init } => {
                    // actual declaration
                    self.handle_declaration(ctx, specifiers, init);
                }
                Declaration::StaticAssertDeclaration(static_assert) => {
                    self.handle_static_assert(static_assert)
                }
            }
        }
        if let Some(expr_clause) = expr_clause {
            self.walk_expression(ctx, expr_clause.clone(), &ExtType::Void.into_pretty());
        }

        // make header_block
        let header_block_id = ctx.mir_function.blocks.len();
        let header_block = MIRBlock::new_wrapped();
        ctx.mir_function.blocks.push(header_block.clone());

        // ending_block
        let ending_block_id = ctx.mir_function.blocks.len();
        let ending_block = MIRBlock::new_wrapped();
        ctx.mir_function.blocks.push(ending_block.clone());

        //  body_block
        let body_block_id = ctx.mir_function.blocks.len();
        let body_block = MIRBlock::new_wrapped();
        ctx.mir_function.blocks.push(body_block.clone());

        ctx.mir_function.current_block.borrow_mut().branches = Some((
            MIRValue { opaque_ref: 0 },
            vec![MIRBranch {
                is_default: true,
                value_needed: 0,
                to_block: header_block_id,
            }],
        ));

        // set current block
        ctx.mir_function.current_block = header_block.clone();

        if let Some(controlling_expr) = controlling_expr {
            let control_value = self.walk_expression(
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

            ctx.mir_function.current_block.borrow_mut().branches = Some((
                control_value,
                vec![
                    MIRBranch {
                        is_default: false,
                        value_needed: 1,
                        to_block: body_block_id,
                    },
                    MIRBranch {
                        is_default: true,
                        value_needed: 0,
                        to_block: ending_block_id,
                    },
                ],
            ));
        }
        ctx.mir_function.current_block = body_block;

        self.walk_statement(ctx, body.clone());

        ctx.mir_function.current_block.borrow_mut().branches = Some((
            MIRValue { opaque_ref: 0 },
            vec![MIRBranch {
                is_default: true,
                value_needed: 0,
                to_block: header_block_id,
            }],
        ));

        // after_expr
        if let Some(after_expr) = after_expr {
            self.walk_expression(ctx, after_expr.clone(), &ExtType::Void.into_pretty());
        }
        ctx.mir_function.current_block = ending_block;
    }
}
