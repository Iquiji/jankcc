mod types;
mod parse_nodes;
mod tokens;

use chumsky::prelude::*;

use crate::lexer::CToken;

use self::{tokens::Token, parse_nodes::{TranslationUnit, ExternalDeclaration}};
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

pub type Span = std::ops::Range<usize>;

pub fn lexer() -> impl Parser<char, Vec<(Token, Span)>, Error = Simple<char>> {    
    // To be filled in later...
    
    // A parser for numbers
    let num = text::int(10)
    .chain::<char, _, _>(just('.').chain(text::digits(10)).or_not().flatten())
    .collect::<String>()
    .map(Token::Num);

    // A parser for strings
    let str_ = just('"')
        .ignore_then(filter(|c| *c != '"').repeated())
        .then_ignore(just('"'))
        .collect::<String>()
        .map(Token::StringLiteral);


    // A parser for control characters (delimiters, semicolons, etc.)
    let ctrl = one_of("()[]{};,").map(|c: char| Token::Punctuator(c.to_string()));

    // A parser for identifiers and keywords
    let ident = text::ident().map(|ident: String| match ident.as_str() {
        _ => Token::Ident(ident),
    });

    // A single token can be one of the above
    let token = num
        .or(str_)
        .or(ctrl)
        .or(ident)
        .recover_with(skip_then_retry_until([]));

    let comment = just("//").then(take_until(just('\n'))).padded();

    token
        .map_with_span(|tok, span| (tok, span))
        .padded_by(comment.repeated())
        .padded()
        .repeated()
}


pub fn parser() -> impl Parser<CToken, Vec<ExternalDeclaration>, Error = Simple<char>>{
    unimplemented!()
}