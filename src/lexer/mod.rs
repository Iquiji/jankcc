mod helper_funcs;
pub mod token_types;

#[cfg(test)]
mod tests;

use std::{
    fmt::{self, Display},
    fs::read_to_string,
};

use log::{debug, error, trace};
use serde::{Deserialize, Serialize};
use token_types::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CToken {
    pub t_type: CTokenType,
    pub original: String,
    pub loc: OriginalLocation,
}

impl fmt::Display for CToken {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(
            f,
            "{}-{}: '{}' -- {:?}",
            self.loc.line, self.loc.collumn, self.original, self.t_type
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OriginalLocation {
    pub file: String,
    pub line: usize,
    pub collumn: usize,
}

pub struct Lexer {
    current_loc: OriginalLocation,
}
impl Lexer {
    pub fn new() -> Self {
        Self {
            current_loc: OriginalLocation {
                file: String::new(),
                line: 0,
                collumn: 0,
            },
        }
    }

    pub fn string_to_token_arr(&mut self, string_to_lexer: String) -> Vec<CToken> {
        let mut buf = vec![];

        for line in string_to_lexer.lines() {
            // check if theres a line resync directive:
            if line.trim_start().starts_with('#') {
                // line directive
                if let Err(err) = self.handle_line_resync_from_preprocessor(line.trim_start()) {
                    error!("error while trying to execute line resync directive from preprocessor!: {:?}",&err);
                    panic!();
                }
            } else {
                let mut current_token_string = String::new();

                // main loop where we check for Tokens
                let mut char_line_iter = line.chars().into_iter().peekable();

                while let Some(character) = char_line_iter.next() {
                    if current_token_string == " " {
                        current_token_string = String::new();
                    }
                    // current_token_string = current_token_string.trim_start().to_string();
                    if helper_funcs::is_punctuator(&current_token_string) {
                        // punctuator
                        let mut punctuator_temp = current_token_string.clone();
                        punctuator_temp.push(character);
                        if punctuator_temp == ".." {
                            if char_line_iter.next() == Some('.') {
                                buf.push(CToken {
                                    t_type: CTokenType::Punctuator,
                                    original: "...".to_string(),
                                    loc: self.current_loc.clone(),
                                });
                                current_token_string = String::new();
                                continue;
                            } else {
                                error!("ellipsis hack exception! seek help!");
                            }
                        } else if helper_funcs::is_punctuator(&punctuator_temp) {
                            if char_line_iter.peek().is_none() {
                                buf.push(CToken {
                                    t_type: CTokenType::Punctuator,
                                    original: punctuator_temp.clone(),
                                    loc: self.current_loc.clone(),
                                });
                                current_token_string = String::new();
                            }
                            // we continue till longest punctuator
                            while let Some(character) = char_line_iter.peek() {
                                punctuator_temp.push(*character);
                                if !helper_funcs::is_punctuator(&punctuator_temp) {
                                    // we do this until it isnt a punctioator anymore in which case we push the last char outside
                                    trace!("current punctuator temp: {:?}", punctuator_temp);
                                    punctuator_temp.remove(punctuator_temp.len() - 1);
                                    buf.push(CToken {
                                        t_type: CTokenType::Punctuator,
                                        original: punctuator_temp.clone(),
                                        loc: self.current_loc.clone(),
                                    });
                                    current_token_string = String::new();
                                    // current_token_string.push(*character);
                                    break;
                                } else {
                                    char_line_iter.next();
                                }
                            }

                            continue;
                        } else {
                            // we just push this one and continue on
                            buf.push(CToken {
                                t_type: CTokenType::Punctuator,
                                original: current_token_string.clone(),
                                loc: self.current_loc.clone(),
                            });
                            current_token_string = String::new();
                        }
                    }
                    if character.is_whitespace() {
                        // it is a whitespace so we do nothing but maybe error if current token string is nonempty
                        if !current_token_string.is_empty() {
                            error!(
                                "!current_token_string.is_empty(): {:?}",
                                current_token_string
                            );
                        }
                    } else if helper_funcs::is_nondigit(character) {
                        // identifier
                        let mut end_char = '`';
                        current_token_string.push(character);
                        for character in char_line_iter.by_ref() {
                            // as long as we have digit or nondigit
                            if helper_funcs::is_nondigit(character)
                                || helper_funcs::is_digit(character)
                            {
                                current_token_string.push(character);
                            } else {
                                end_char = character;
                                break;
                            }
                        }
                        if let Some(keyword) = CKeyword::to_keyword(&current_token_string) {
                            buf.push(CToken {
                                t_type: CTokenType::Keyword(keyword),
                                original: current_token_string.clone(),
                                loc: self.current_loc.clone(),
                            });
                        } else {
                            buf.push(CToken {
                                t_type: CTokenType::Identifier,
                                original: current_token_string.clone(),
                                loc: self.current_loc.clone(),
                            });
                        }
                        current_token_string = String::new();
                        if end_char != '`' {
                            current_token_string.push(end_char);
                        }
                    } else if helper_funcs::is_digit(character) {
                        // number
                        current_token_string.push(character);
                        let mut end_char = '`';
                        let mut point_seperator_reached = false;
                        let mut hex_x_reached = false;
                        for character in char_line_iter.by_ref() {
                            // as long as we have digit or nondigit
                            if (hex_x_reached
                                && ['a', 'b', 'c', 'd', 'e', 'f']
                                    .contains(&character.to_ascii_lowercase()))
                                || helper_funcs::is_digit(character)
                            {
                                current_token_string.push(character);
                            } else if character == '.' {
                                if point_seperator_reached {
                                    panic!("Second Point seperator in number")
                                }
                                point_seperator_reached = true;
                                current_token_string.push(character);
                            } else if character.to_ascii_lowercase() == 'x' {
                                if hex_x_reached {
                                    panic!("Second Hex 'x' seperator in number")
                                }
                                current_token_string.push(character);
                                hex_x_reached = true;
                            } else if hex_x_reached
                                && (character.to_ascii_uppercase() == 'L'
                                    || character.to_ascii_uppercase() == 'U')
                            {
                                current_token_string.push(character);
                                if let Some(second_l_maybe) = char_line_iter.next() {
                                    if second_l_maybe.to_ascii_uppercase() == 'L' {
                                        current_token_string.push(character);
                                        if let Some(second_l_maybe) = char_line_iter.next() {
                                            if second_l_maybe.to_ascii_uppercase() == 'L' {
                                                current_token_string.push(character);
                                            } else {
                                                end_char = second_l_maybe;
                                            }
                                        }
                                    } else {
                                        end_char = second_l_maybe;
                                    }
                                }
                                break;
                            } else {
                                end_char = character;
                                break;
                            }
                        }
                        buf.push(CToken {
                            t_type: CTokenType::Constant,
                            original: current_token_string.clone(),
                            loc: self.current_loc.clone(),
                        });
                        if hex_x_reached {
                            debug!("hex_constant!: {:?}", current_token_string);
                        }
                        current_token_string = String::new();
                        if end_char != '`' {
                            current_token_string.push(end_char);
                        }
                    } else {
                        self.current_loc.collumn += 1;
                        current_token_string.push(character);
                    }
                    // if !current_token_string.is_empty() {
                    //     buf.push(CToken {
                    //         t_type: CTokenType::Identifier,
                    //         original: current_token_string.clone(),
                    //         loc: self.current_loc.clone(),
                    //     });
                    // }
                    if current_token_string.clone().ends_with('\"') {
                        current_token_string.remove(current_token_string.len() - 1);
                        // warn!(
                        //     "string-start: {:?} - {:?}",
                        //     char_line_iter, self.current_loc
                        // );
                        // char_line_iter.next();
                        for character_next in char_line_iter.by_ref() {
                            // error!("string: !{}!'{}'", character_next, current_token_string);

                            // as long as we have are still in a string:
                            if character_next == '"' {
                                if current_token_string.ends_with('\\') {
                                    current_token_string.remove(current_token_string.len() - 1);
                                    current_token_string.push('"');
                                    continue;
                                } else {
                                    break;
                                }
                            } else if character_next == 'n' {
                                if current_token_string.ends_with('\\') {
                                    current_token_string.remove(current_token_string.len() - 1);
                                    current_token_string.push('\n');
                                    continue;
                                }
                                current_token_string.push(character_next);
                            } else {
                                current_token_string.push(character_next);
                            }
                        }
                        buf.push(CToken {
                            t_type: CTokenType::StringLiteral,
                            original: current_token_string
                                .clone()
                                .trim_start_matches('"')
                                .trim_end_matches('"')
                                .to_string(),
                            loc: self.current_loc.clone(),
                        });
                        trace!(
                            "string-end!: '{}' - {:?}-{:?}",
                            current_token_string,
                            self.current_loc.line,
                            self.current_loc.collumn,
                        );
                        current_token_string = String::new();
                    }
                }
                if helper_funcs::is_punctuator(&current_token_string) {
                    // punctuator
                    buf.push(CToken {
                        t_type: CTokenType::Punctuator,
                        original: current_token_string.clone(),
                        loc: self.current_loc.clone(),
                    });
                    // current_token_string = String::new();
                }
            }
            self.current_loc.line += 1;
            self.current_loc.collumn = 0;
        }

        buf
    }

    pub fn _file_to_token_arr(
        &mut self,
        file_to_lexer: String,
    ) -> Result<Vec<CToken>, Box<dyn std::error::Error>> {
        let read_in_file = read_to_string(file_to_lexer)?;

        Ok(self.string_to_token_arr(read_in_file))
    }
    pub fn handle_line_resync_from_preprocessor(
        &mut self,
        sync_string: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let split_sync_string: Vec<&str> = sync_string.split_ascii_whitespace().collect();
        let get_resync_line_num: usize = split_sync_string[1].parse()?;
        let file_resync = split_sync_string[2];

        self.current_loc.line = get_resync_line_num;
        self.current_loc.collumn = 0;
        self.current_loc.file = file_resync.to_string();

        Ok(())
    }
}
impl Display for OriginalLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}-{}", self.file, self.line, self.collumn)
    }
}
