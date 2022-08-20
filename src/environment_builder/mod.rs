use self::symbol_table::{BlockContainer, ScopeContainer};

mod symbol_table;
mod walker;

/*
Need to know:
- Current Function
- Current Switch Statement

*/

pub struct EnvironmentController {
    symbol_table: BlockContainer,
}

impl EnvironmentController {
    pub fn new() -> Self {
        EnvironmentController {
            symbol_table: BlockContainer {
                scope: ScopeContainer::new(),
                active_inner: None,
                past_inner: vec![],
            },
        }
    }
    pub(crate) fn build(&mut self, ast: crate::parser::parse_nodes::TranslationUnit) {
        self.walk_translation_unit(ast);
        todo!()
    }
}
