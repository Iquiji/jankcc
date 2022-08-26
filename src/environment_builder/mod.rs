use self::symbol_table::{BlockContainer, ScopeContainer};

mod symbol_table;
mod walker;
mod ext_type;

/*
Need to know:
- Current Function
- Current Switch Statement

*/

/*
Need to Add new Type Representation that has the expr and stuff removed
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
