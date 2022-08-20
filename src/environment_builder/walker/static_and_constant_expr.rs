use crate::{parser::{span::Spanned, parse_nodes::declarations::StaticAssertDeclaration}, environment_builder::EnvironmentController};

impl EnvironmentController{
    pub(crate) fn handle_static_assert(&mut self,static_assert: &Spanned<StaticAssertDeclaration>){


    }
}