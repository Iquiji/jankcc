mod helper_funcs;
pub mod token_types;

#[cfg(test)]
mod tests;

use std::{fmt, fs::read_to_string};

use log::{error, trace, Log};
use serde::{Deserialize, Serialize};
use token_types::*;
use logos::{Logos,Lexer as LogosLexer};

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
                let lex = LogosToken::lexer(line);

                for toki in lex{
                    let mut token = match toki{
                        LogosToken::Error => {
                            error!("Error in logos!: {:?}",toki);
                            continue;
                        },
                        LogosToken::String(toki) => {
                            toki
                        },
                        LogosToken::Identifier(toki) => {
                            toki
                        },
                        LogosToken::Number(toki) => {
                            toki
                        },
                        LogosToken::Punctuator(toki) => {
                            toki
                        },
                        LogosToken::Keyword(toki) => {
                            toki
                        },
                    };
                    token.loc.line = self.current_loc.line;
                    token.loc.file = self.current_loc.file.clone();

                    buf.push(token);
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


#[derive(Logos, Debug, PartialEq)]
enum LogosToken {
    #[regex(r"[ \t\n\f]+", logos::skip)]
    #[error]
    Error,

    // Callbacks can use closure syntax, or refer
    // to a function defined elsewhere.
    //
    // Each pattern can have it's own callback.
   
    #[regex("(0[xX])[a-fA-F0-9]+(((u|U)(l|L|ll|LL)?)|((l|L|ll|LL)(u|U)?))", number)]
    #[regex("[1-9][0-9]*(((u|U)(l|L|ll|LL)?)|((l|L|ll|LL)(u|U)?))", number)]
    #[regex(r#""0"[0-7]*(((u|U)(l|L|ll|LL)?)|((l|L|ll|LL)(u|U)?))"#, number)]
    #[regex(r#"[0-9]*"."[0-9]+(f|F|l|L)?"#, number)]
    Number(CToken),

    #[regex(r#"/"(?:[^"\\]|\\.)*"/"#, string)]
    String(CToken),

    

    #[regex(r#"/\[ | \] | \( | \) | \. | -> | ++ | -- | & | * | + | \- | ~ | ! | / | % | << | >> | < | > | <= | >= | == | !=/"#, punct,priority=8,)]
    #[regex(r#"/\^| \| | \&\& | \\|\\| | \\? | : | ; | ... | = | *=| \\/= | %=| +=| \-=| <<=| >>=| \&= | \^= | \\|= | , | \#| \#\#/"#, punct,priority=10,)]
    Punctuator(CToken),

    #[regex(r#"( auto
     | break
     | case
     | char
     | const
     | continue
     | default
     | do
     | double
     | else
     | enum
     | extern
     | float
     | for
     | goto
     | if
     | inline
     | int
     | long
     | register
     | restrict
     | return
     | short
     | signed
     | sizeof
     | static
     | struct
     | switch
     | typedef
     | union
     | unsigned
     | void
     | volatile
     | while
     | _Alignas
     | _Alignof
     | _Atomic
     | _Bool
     | _Complex
     | _Generic
     | _Imaginary
     | _Noreturn
     | _Static_assert
     | _Thread_local )"#, keyword)]
    Keyword(CToken),

    #[regex(r#"[_a-zA-Z][_a-zA-Z0-9]*"#, ident)]
    Identifier(CToken),
}


fn number(lex: &mut LogosLexer<LogosToken>) -> CToken {
    CToken{
        t_type: CTokenType::Constant,
        original: lex.slice().to_string(),
        loc: OriginalLocation { file: "".to_string(), line: 0, collumn: lex.span().start },
    }
}
fn string(lex: &mut LogosLexer<LogosToken>) -> CToken {
    CToken{
        t_type: CTokenType::StringLiteral,
        original: lex.slice().to_string(),
        loc: OriginalLocation { file: "".to_string(), line: 0, collumn: lex.span().start },
    }
}
fn keyword(lex: &mut LogosLexer<LogosToken>) -> CToken {
    CToken{
        t_type: CTokenType::Keyword(CKeyword::to_keyword(lex.slice()).unwrap()),
        original: lex.slice().to_string(),
        loc: OriginalLocation { file: "".to_string(), line: 0, collumn: lex.span().start },
    }
}
fn ident(lex: &mut LogosLexer<LogosToken>) -> CToken {
    CToken{
        t_type: CTokenType::Identifier,
        original: lex.slice().to_string(),
        loc: OriginalLocation { file: "".to_string(), line: 0, collumn: lex.span().start },
    }
}
fn punct(lex: &mut LogosLexer<LogosToken>) -> CToken {
    CToken{
        t_type: CTokenType::Punctuator,
        original: lex.slice().to_string(),
        loc: OriginalLocation { file: "".to_string(), line: 0, collumn: lex.span().start },
    }
}