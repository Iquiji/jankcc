mod types;
mod parse_nodes;
mod tokens;

use chumsky::{prelude::*, Span};

use self::{parse_nodes::TranslationUnit, tokens::Token};
// TODO:

pub struct CParser{
    program: String,
}

impl CParser{
    pub fn new(program: String) -> Self{
        CParser { program }
    }
    pub fn parse(&self){

    }
}

pub fn lexer() -> impl Parser<char, Vec<(Token, Span)>, Error = Simple<char>> {    // To be filled in later...
    unimplemented!()
}