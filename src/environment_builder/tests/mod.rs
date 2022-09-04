use crate::{
    lexer::Lexer,
    parser::{parse_nodes::declarations::Declaration, CParser},
};

use super::EnvironmentController;

mod pretty_type;

pub(crate) fn make_environment_controller() -> EnvironmentController {
    EnvironmentController::new()
}

fn init() {
    let _ = env_logger::builder().is_test(true).try_init();
}

pub(crate) fn run_lexer_with_return_that_init_parser(code: &str) -> CParser {
    init();

    let lexed = Lexer::new().string_to_token_arr(code.to_string());

    CParser::new(lexed)
}

pub(crate) fn parser_parse_specific<T, F>(code: &str, func: F) -> T
where
    F: Fn(&mut CParser) -> T,
{
    let mut simple_parser = run_lexer_with_return_that_init_parser(code);
    func(&mut simple_parser)
}
