use super::CParser;
use crate::lexer::Lexer;

mod expr;
mod type_names;

pub(crate) fn run_lexer_with_return_that_init_parser(code: &str) -> CParser {
    let lexed = Lexer::new().string_to_token_arr(code.to_string());

    CParser::new(lexed)
}
