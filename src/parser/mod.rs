pub(crate) mod parse_nodes;
pub(crate) mod types;

use crate::lexer::{token_types::CTokenType, CToken};

use self::parse_nodes::declarations::{Declaration, InitDeclaratorList, StorageClassSpecifier};
use self::parse_nodes::{ExternalDeclaration, Identifier, TranslationUnit};
// TODO:

pub struct CParser {
    tokens: Vec<CToken>,
}

impl CParser {
    pub fn new(program_tokens: Vec<CToken>) -> Self {
        CParser {
            tokens: program_tokens,
        }
    }
    pub(crate) fn parse(&mut self) -> TranslationUnit {
        vec![]
    }
}
