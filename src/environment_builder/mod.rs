use crate::mir::MIRProgramm;

use self::symbol_table::{BlockContainer, ScopeContainer};

#[cfg(test)]
mod tests;

mod constant_expr;
pub mod ext_type;
mod symbol_table;
mod walker;

/*
Need to know:
- Current Function
- Current Switch Statement

*/

pub(crate) enum CompileTimeValue {
    Int(i128),
    Float(f64),
    String(String),
}

/*
Need to Add new Type Representation that has the expr and stuff removed
*/
pub struct EnvironmentController {
    symbol_table: BlockContainer,
    mir_programm: MIRProgramm,
}

impl EnvironmentController {
    pub fn new() -> Self {
        EnvironmentController {
            symbol_table: BlockContainer::new(),
            mir_programm: MIRProgramm::new(),
        }
    }
    pub(crate) fn build(&mut self, ast: crate::parser::parse_nodes::TranslationUnit) {
        self.walk_translation_unit(ast);
    }
    pub(crate) fn get_mir(&mut self) -> MIRProgramm {
        self.mir_programm.clone()
    }
}
