pub(crate) use super::{walk_func::FunctionContext, *};
pub(crate) use crate::{
    environment_builder::{
        ext_type::PrettyType, symbol_table::VariableInstance, EnvironmentController,
    },
    parser::{
        parse_nodes::statements::{self, Statement},
        span::Spanned,
    },
};

mod compound;
mod r#return;

impl EnvironmentController {
    pub(crate) fn walk_statement(
        &mut self,
        ctx: &mut FunctionContext,
        statement: Spanned<Statement>,
    ) {
        debug!("{}", serde_yaml::to_string(&statement).unwrap());
        match &*statement.inner {
            Statement::Labeled { label: _, body: _ } => todo!(),
            Statement::SwitchCase {
                const_expr: _,
                statement: _,
            } => todo!(),
            Statement::SwitchDefault { statement: _ } => todo!(),
            Statement::Compound(compound_statement_list) => {
                self.handle_compound_statement(ctx, compound_statement_list);
            }
            Statement::CExpression(expression) => {
                let _ = self.walk_expression(ctx, expression.clone(), &PrettyType::default_void());
            }
            Statement::NoneExpr => {}
            Statement::If {
                controlling_expr: _,
                true_body: _,
                else_body: _,
            } => todo!(),
            Statement::Switch {
                controlling_expr: _,
                body: _,
            } => todo!(),
            Statement::While {
                while_type: _,
                controlling_expr: _,
                body: _,
            } => todo!(),
            Statement::For {
                decl_clause: _,
                expr_clause: _,
                controlling_expr: _,
                after_expr: _,
                body: _,
            } => todo!(),
            Statement::Goto(_) => todo!(),
            Statement::Continue => todo!(),
            Statement::Break => todo!(),
            Statement::Return(return_expr) => {
                self.handle_return_statement(ctx,return_expr);
            }
        }
    }
}
