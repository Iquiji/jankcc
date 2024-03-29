pub(crate) mod parse_nodes;
pub(crate) mod span;
pub(crate) mod types;

#[cfg(test)]
mod tests;

use std::collections::HashSet;

use crate::lexer::token_types::CKeyword;
use crate::lexer::OriginalLocation;
use crate::lexer::{token_types::CTokenType, CToken};

// use self::parse_nodes::declarations::{Declaration, InitDeclaratorList, StorageClassSpecifier};
use self::parse_nodes::{Identifier, TranslationUnit};
// use self::span::Spanned;

pub struct CParser {
    tokens: Vec<CToken>,
    idx: usize,
    typedef_table: Vec<HashSet<String>>,
}

impl CParser {
    pub fn new(program_tokens: Vec<CToken>) -> Self {
        CParser {
            tokens: program_tokens,
            idx: 0,
            typedef_table: vec![HashSet::new()],
        }
    }
    pub(crate) fn parse(&mut self) -> TranslationUnit {
        let mut translation_unit = vec![];

        while self.current_token().t_type != CTokenType::Eof {
            translation_unit.push(self.parse_external_declaration());
        }

        translation_unit
    }
}
/*
    I may need a symbol table during parse time for resolving ambigous grammar?!
    simple typedef-table to seperate ambiguity?
    can be cleaned after scope and deleted after use

    list of hashsets
*/
impl CParser {
    pub(crate) fn is_typedef(&self, ident: &str) -> bool {
        for table in &self.typedef_table {
            if table.contains(ident) {
                return true;
            }
        }
        false
    }
    pub(crate) fn push_typedef_scope(&mut self) {
        self.typedef_table.push(HashSet::new());
    }
    pub(crate) fn pop_typedef_scope(&mut self) {
        self.typedef_table.pop();
    }
    pub(crate) fn insert_typedef(&mut self, ident: &str) {
        let last = self.typedef_table.len() - 1;
        self.typedef_table[last].insert(ident.to_string());
    }
}

/*
    Helper functions for expecting, accepting and selecting tokens
*/
impl CParser {
    pub(crate) fn expect_type(&mut self, type_to_accept: CTokenType) -> CToken {
        if self.current_token().t_type == type_to_accept {
            self.advance_idx()
        } else {
            self.error_unexpected(
                self.current_token(),
                &format!("TokenType: {:?}", type_to_accept),
            );
            unreachable!();
        }
    }
    pub(crate) fn expect_type_and_string(
        &mut self,
        type_to_accept: CTokenType,
        string: &str,
    ) -> CToken {
        if self.current_token().t_type == type_to_accept && self.current_token().original == string
        {
            self.advance_idx()
        } else {
            self.error_unexpected(
                self.current_token(),
                &format!(
                    "TokenType: {:?} with Original String: '{}'",
                    type_to_accept, string
                ),
            );
            unreachable!();
        }
    }
    pub(crate) fn expect_one_of_keywords(&mut self, keywords_to_accept: &[CKeyword]) -> CKeyword {
        // error!("expect_one_of_keywords only stub right now");
        let current_token = self.current_token();
        if let CTokenType::Keyword(keyword) = current_token.t_type {
            if keywords_to_accept.contains(&keyword) {
                self.advance_idx();
                keyword
            } else {
                self.error_unexpected(
                    self.current_token(),
                    &format!("expected one of keywords: {:?}", keywords_to_accept),
                );
                unreachable!()
            }
        } else {
            self.error_unexpected(
                self.current_token(),
                &format!("expected one of keywords: {:?}", keywords_to_accept),
            );
            unreachable!()
        }
    }
    pub(crate) fn current_token(&self) -> CToken {
        self.tokens
            .get(self.idx)
            .unwrap_or(&CToken {
                t_type: CTokenType::Eof,
                original: String::new(),
                loc: OriginalLocation {
                    file: String::new(),
                    line: 0,
                    collumn: 0,
                },
            })
            .clone()
    }
    pub(crate) fn next_token(&self) -> CToken {
        self.tokens
            .get(self.idx + 1)
            .unwrap_or(&CToken {
                t_type: CTokenType::Eof,
                original: String::new(),
                loc: OriginalLocation {
                    file: String::new(),
                    line: 0,
                    collumn: 0,
                },
            })
            .clone()
    }
    pub(crate) fn prev_token(&self) -> CToken {
        self.tokens
            .get(self.idx.saturating_sub(1))
            .unwrap_or(&CToken {
                t_type: CTokenType::Eof,
                original: String::new(),
                loc: OriginalLocation {
                    file: String::new(),
                    line: 0,
                    collumn: 0,
                },
            })
            .clone()
    }
    pub(crate) fn advance_idx(&mut self) -> CToken {
        let temp = self.tokens[self.idx].clone();
        self.idx += 1;
        temp
    }
}
/*
    Error handling when something is expected
*/
impl CParser {
    pub(crate) fn error_unexpected(&mut self, found: CToken, expected: &str) {
        panic!(
            "Line {}-{}: Expected: {}, Instead found Token: {:?}",
            found.loc.line, found.loc.collumn, expected, found
        );
    }
}
