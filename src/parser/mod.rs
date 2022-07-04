mod parse_nodes;
mod types;

use chumsky::error::SimpleReason;
use chumsky::{prelude::*, Error};

use crate::lexer::{token_types::CTokenType, CToken};

use self::parse_nodes::declarations::{Declaration, InitDeclaratorList, StorageClassSpecifier};
use self::parse_nodes::{ExternalDeclaration, Identifier, TranslationUnit};
// TODO:

pub struct CParser {
    program: String,
}

impl CParser {
    pub fn new(program: String) -> Self {
        CParser { program }
    }
    pub fn parse(&self) {}
}

pub type Span = std::ops::Range<usize>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Token {
    Identifier(String),
    StringLiteral(String),
    Constant(String),
    Keyword(String),
    Punctuator(String),
}
impl Token{
    pub fn to_inner_string(&self) -> String{
        match self{
            Token::Identifier(inner_str) => inner_str.clone(),
            Token::StringLiteral(inner_str) => inner_str.clone(),
            Token::Constant(inner_str) => inner_str.clone(),
            Token::Keyword(inner_str) => inner_str.clone(),
            Token::Punctuator(inner_str) => inner_str.clone(),
        }
    }
}

pub(crate) fn lexer() -> impl Parser<char, Vec<(Token,Span)>, Error = Simple<char>> {
    // A parser for numbers
    let num = text::int(10)
        .chain::<char, _, _>(just('.').chain(text::digits(10)).or_not().flatten())
        .collect::<String>()
        .map(Token::Constant);

    // A parser for strings
    let str_ = just('"')
        .ignore_then(filter(|c| *c != '"').repeated())
        .then_ignore(just('"'))
        .collect::<String>()
        .map(Token::StringLiteral);

    // A parser for identifiers and keywords
    let ident = text::ident().map(|ident: String| match ident.as_str() {
        "auto" => Token::Keyword("auto".to_string()),
        "break" => Token::Keyword("break".to_string()),
        "case" => Token::Keyword("case".to_string()),
        "char" => Token::Keyword("char".to_string()),
        "const" => Token::Keyword("const".to_string()),
        "continue" => Token::Keyword("continue".to_string()),
        "default" => Token::Keyword("default".to_string()),
        "do" => Token::Keyword("do".to_string()),
        "double" => Token::Keyword("double".to_string()),
        "else" => Token::Keyword("else".to_string()),
        "enum" => Token::Keyword("enum".to_string()),
        "extern" => Token::Keyword("extern".to_string()),
        "float" => Token::Keyword("float".to_string()),
        "for" => Token::Keyword("for".to_string()),
        "goto" => Token::Keyword("goto".to_string()),
        "if" => Token::Keyword("if".to_string()),
        "inline" => Token::Keyword("inline".to_string()),
        "int" => Token::Keyword("int".to_string()),
        "long" => Token::Keyword("long".to_string()),
        "register" => Token::Keyword("register".to_string()),
        "restrict" => Token::Keyword("restrict".to_string()),
        "return" => Token::Keyword("return".to_string()),
        "short" => Token::Keyword("short".to_string()),
        "signed" => Token::Keyword("signed".to_string()),
        "sizeof" => Token::Keyword("sizeof".to_string()),
        "static" => Token::Keyword("static".to_string()),
        "struct" => Token::Keyword("struct".to_string()),
        "switch" => Token::Keyword("switch".to_string()),
        "typedef" => Token::Keyword("typedef".to_string()),
        "union" => Token::Keyword("union".to_string()),
        "unsigned" => Token::Keyword("unsigned".to_string()),
        "void" => Token::Keyword("void".to_string()),
        "volatile" => Token::Keyword("volatile".to_string()),
        "while" => Token::Keyword("while".to_string()),
        _ => Token::Identifier(ident),
    });

    // let extended_punctuator = just("[")
    //     .or(just("<<"))
    //     .or(just(">>"))
    //     .or(just("<"))
    //     .or(just(">"))
    //     .or(just("<="))
    //     .or(just(">="))
    //     .or(just("=="))
    //     .or(just("!="))
    //     .or(just("^"))
    //     .or(just("|"))
    //     .or(just("&&"))
    //     .or(just("||"))
    //     .or(just("?"))
    //     .or(just(":"))
    //     .or(just(";"))
    //     .or(just("..."))
    //     .or(just("="))
    //     .or(just("*="))
    //     .or(just("/="))
    //     .or(just("%="))
    //     .or(just("+="))
    //     .or(just("-="))
    //     .or(just("<<="))
    //     .or(just(">>="))
    //     .or(just("&="))
    //     .or(just("^="))
    //     .or(just("|="))
    //     .or(just(""))
    //     .or(just("#"))
    //     .or(just("##"))
    //     .or(just("<:"))
    //     .or(just(":>"))
    //     .or(just("<%"))
    //     .or(just("%>"))
    //     .or(just("%:"))
    //     .or(just("%:%:"))
    //     .or(just("]"))
    //     .or(just("("))
    //     .or(just(")"))
    //     .or(just("{"))
    //     .or(just("}"))
    //     .or(just("."))
    //     .or(just("->"))
    //     .or(just("++"))
    //     .or(just("--"))
    //     .or(just("&"))
    //     .or(just("*"))
    //     .or(just("+"))
    //     .or(just("-"))
    //     .or(just("~"))
    //     .or(just("!"))
    //     .or(just("/"))
    //     .or(just("%"))
    //     .map(Token::Punctuator).boxed();

    let punctuator_first = choice((
        just("...").to("..."),
        just("<<").to("<<"),
        just(">>").to(">>"),
        just("<").to("<"),
        just(">").to(">"),
        just("<=").to("<="),
        just(">=").to(">="),
        just("==").to("=="),
        just("!=").to("!="),
        just("^").to("^"),
        just("|").to("|"),
        just("&&").to("&&"),
        just("||").to("||"),
    )).map(|punc_str: &str| Token::Punctuator(punc_str.to_string()));
    let punctuator_second = choice((
        just("*=").to("*="),
        just("/=").to("/="),
        just("%=").to("%="),
        just("+=").to("+="),
        just("-=").to("-="),
        just("<<=").to("<<="),
        just(">>=").to(">>="),
        just("&=").to("&="),
        just("^=").to("^="),
        just("|=").to("|="),
        just(",").to(","),
        just("##").to("##"),
        just("->").to("->"),
        just("++").to("++"),
        just("--").to("--"),
    )).map(|punc_str: &str| Token::Punctuator(punc_str.to_string()));
    let punctuator_third = choice((
        just('[').to("["),
        just("?").to("?"),
        just(":").to(":"),
        just(";").to(";"),
        just("=").to("="),
        just("#").to("#"),
        just("]").to("]"),
        just("(").to("("),
        just(")").to(")"),
        just("{").to("{"),
        just("}").to("}"),
        just(".").to("."),
        just("&").to("&"),
        just("*").to("*"),
        just("+").to("+"),
        just("-").to("-"),
        just("~").to("~"),
        just("!").to("!"),
        just("/").to("/"),
        just("%").to("%"),
    )).map(|punc_str: &str| Token::Punctuator(punc_str.to_string()));
   
    let punctuator = punctuator_first .or(punctuator_second).or(punctuator_third);

    let line_resync_from_preprocessing_ignored =just::<_, _, Simple<char>>("\n#")
        .then(take_until(text::newline()))
        .ignored().ignored();

    // A single token can be one of the above
    let token = num
        .or(str_)
        .or(punctuator)       
        .or(ident)
        .recover_with(skip_then_retry_until([]));

    let single_line = just::<_, _, Simple<char>>("//")
        .then(take_until(text::newline()))
        .ignored();

    let multi_line = just::<_, _, Simple<char>>("/*")
        .then(take_until(just("*/")))
        .ignored();

    let comment = line_resync_from_preprocessing_ignored.or(single_line).or(multi_line);

    token.map_with_span(|tok, span| (tok, span))
        .padded_by(comment.repeated())
        .padded()
        .repeated()
}
