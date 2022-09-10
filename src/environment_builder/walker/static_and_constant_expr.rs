use crate::{
    environment_builder::EnvironmentController,
    parser::{parse_nodes::declarations::StaticAssertDeclaration, span::Spanned},
};

impl EnvironmentController {
    pub(crate) fn handle_static_assert(
        &mut self,
        _static_assert: &Spanned<StaticAssertDeclaration>,
    ) {
    }
}
