#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Token{
    Num(String),
    Ident(String),
    Punctuator(String),
    StringLiteral(String),
}