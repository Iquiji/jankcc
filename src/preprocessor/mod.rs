/*
Notes:
#include <file.h> | "file.h": preprocess that file and copy pastes every line that isnt a macro call
#define [X] [Y]: replaces every occurance of [X] with [Y]
#ifdef [X]
#if ...statements...
#ifndef [X]
#else
#endif

Statements:
- defined
- can be connected by: "!", "||", "&&"
*/

use std::collections::HashMap;

pub struct Preprocessor {
    define_map: HashMap<String, String>,
}
impl Preprocessor {
    pub fn new() -> Self {
        Preprocessor {
            define_map: HashMap::new(),
        }
    }
    pub fn preprocess_code_string(&mut self, code_string: String) -> String {
        let mut output_buf: Vec<String> = vec![];
        // 1. Physical source file multibyte characters are mapped, in an implementation-
        // defined manner, to the source character set (introducing new-line characters for
        // end-of-line indicators) if necessary. Trigraph sequences are replaced by
        // corresponding single-character internal representations

        // 2. Each instance of a backslash character (\) immediately followed by a new-line
        // character is deleted, splicing physical source lines to form logical source lines.
        // Only the last backslash on any physical source line shall be eligible for being part
        // of such a splice. A source file that is not empty shall end in a new-line character,
        // which shall not be immediately preceded by a backslash character before any such
        // splicing takes place.

        // 3. The source file is decomposed into preprocessing tokens 7) and sequences of
        // white-space characters (including comments). A source file shall not end in a
        // partial preprocessing token or in a partial comment. Each comment is replaced by
        // one space character. New-line characters are retained. Whether each nonempty
        // sequence of white-space characters other than new-line is retained or replaced by
        // one space character is implementation-defined.
        let split_code = split_code_string_into_words(code_string);

        // 4. Preprocessing directives are executed, macro invocations are expanded, and
        // _Pragma unary operator expressions are executed. If a character sequence that
        // matches the syntax of a universal character name is produced by token
        // concatenation (6.10.3.3), the behavior is undefined. A #include preprocessing
        // directive causes the named header or source file to be processed from phase 1
        // through phase 4, recursively. All preprocessing directives are then deleted.
        for line in &split_code{
            if line.word_list[0] == "#"{
                // we need to do stuff
                // and generate debug like this:
                // # linenum filename flags
                // ‘1’ This indicates the start of a new file. 
                // ‘2’ This indicates returning to a file (after having included another file). 
                // ‘3’ This indicates that the following text comes from a system header file, so certain warnings should be suppressed. 
                // ‘4’ This indicates that the following text should be treated as being wrapped in an implicit extern "C" block. 

                match line.word_list[1].as_str(){
                    "include" => {
                        
                    },
                    unknown_command => panic!("Unknown Preprocessor command!: {}",unknown_command),
                }
            }else{
                // flush to our output
                output_buf.push(format!("{}{}",(0..line.ident_level).map(|_| String::new()).collect::<Vec<String>>().join(" "),line.word_list.join(" ")));
            }
        }

        output_buf.join("\n")
    }
}

// preprocessing-token:
//      header-name
//      identifier: nondigit (digit | nondigit)*
//      pp-number
//      character-constant
//      string-literal
//      punctuator: one of
// [ ] ( ) { } . ->
// ++ -- & * + - ~ !
// / % << >> < > <= >= == != ^ | && ||
// ? : ; ...
// = *= /= %= += -= <<= >>= &= ^= |=
// , # ##
// <: :> <% %> %: %:%:
//      each non-white-space character that cannot be one of the above

fn split_code_string_into_words(code_string: String) -> Vec<Line> {
    let mut line_buf: Vec<Line> = vec![];

    let mut current_line = Line {
        ident_level: 0,
        word_list: vec![],
    };

    let mut current_word = "".to_string();
    let mut in_string = false;

    let mut chars = code_string.chars().fuse().peekable();

    while let Some(c) = chars.next() {
        // Each instance of a backslash character (\) immediately followed by a new-line
        // character is deleted, splicing physical source lines to form logical source lines.
        if current_word == "#"{
            current_line.word_list.push(current_word);
            current_word = String::new();
        }
        if current_line.word_list.is_empty() && current_word.is_empty(){
            if c == '\t'{
                current_line.ident_level += 4;
            }
            if c == ' '{
                current_line.ident_level += 1;
            }
        }
        if c == '\\' && !in_string && chars.peek().is_some() && *chars.peek().unwrap() == '\n' {
            chars.next();
        } else if c == '\n' && !in_string {
            if !current_word.is_empty() {
                current_line.word_list.push(current_word.clone());
                current_word = String::new();
            }
            line_buf.push(current_line);
            current_line = Line {
                ident_level: 0,
                word_list: vec![],
            };
        } else if c == '"' {
            in_string = !in_string;
            current_word.push(c);
        } else if c == ' ' {
            if in_string {
                current_word.push(c);
            } else if !current_word.is_empty(){
                current_line.word_list.push(current_word);
                current_word = String::new();
            }
        } else {
            current_word.push(c);
        }
    }
    if !current_word.is_empty() {
        current_line.word_list.push(current_word.clone());
    }
    if current_line
        != (Line {
            ident_level: 0,
            word_list: vec![],
        })
    {
        line_buf.push(current_line);
    }

    for line in &line_buf{
        println!("{:?}", line);
    }
    

    line_buf
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Line {
    ident_level: usize,
    word_list: Vec<String>,
}
