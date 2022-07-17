use super::CParser;
use crate::lexer::Lexer;

mod decl;
mod expr;
mod type_names;

fn init() {
    let _ = env_logger::builder().is_test(true).try_init();
}

pub(crate) fn run_lexer_with_return_that_init_parser(code: &str) -> CParser {
    init();

    let lexed = Lexer::new().string_to_token_arr(code.to_string());

    CParser::new(lexed)
}
