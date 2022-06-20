mod helper_funcs;
mod token_types;

use std::fs::read_to_string;

use token_types::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CToken {
    pub t_type: CTokenType,
    pub original: String,
    pub loc: OriginalLocation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OriginalLocation {
    file: String,
    line: usize,
    collumn: usize,
}

fn string_to_token_arr(string_to_lexer: String) -> Vec<CToken> {
    let buf = vec![];

    buf
}

fn file_to_token_arr(file_to_lexer: String) -> Result<Vec<CToken>, Box<dyn std::error::Error>> {
    let read_in_file = read_to_string(file_to_lexer)?;

    Ok(string_to_token_arr(read_in_file))
}
