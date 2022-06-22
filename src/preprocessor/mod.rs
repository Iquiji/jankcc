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

use std::{collections::HashMap, path::Path, fs::read_to_string, iter::{Enumerate, Fuse}, slice::Iter};

pub struct Preprocessor {
    define_map: HashMap<String, String>,
}
impl Preprocessor {
    pub fn new() -> Self {
        Preprocessor {
            define_map: HashMap::new(),
        }
    }
    pub fn preprocess_code_string(&mut self, code_string: String,file_path: String) -> String {
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

        // set by if and elif and endif statements
        let mut execute_macro = true;
        let mut line_iter = split_code.iter().enumerate().fuse();


        let output = self.execute_preprocessor(&mut line_iter, file_path);

        output
    }
    fn execute_preprocessor(&mut self,line_iter: &mut Fuse<Enumerate<Iter<Line>>>,file_path: String) -> String{
        let mut output_buf: Vec<String> = vec![];

        while let Some(enumerated_line) = line_iter.next() {
            let line = enumerated_line.1;
            let line_num = enumerated_line.0;

            if line.word_list.is_empty(){
                output_buf.push(String::new());
                continue;
            }
            if line.word_list[0] == "#" {
                // we need to do stuff
                // and generate debug like this:
                // # linenum filename flags
                // ‘1’ This indicates the start of a new file.
                // ‘2’ This indicates returning to a file (after having included another file).
                // ‘3’ This indicates that the following text comes from a system header file, so certain warnings should be suppressed.
                // ‘4’ This indicates that the following text should be treated as being wrapped in an implicit extern "C" block.

                // ifs can be nested:
                // so we go until a endif with equal nesting is found?


                match line.word_list[1].as_str() {
                    "include" => {
                        let file_to_include: String;
                        if line.word_list[3].starts_with('<') && line.word_list[3].ends_with('>'){
                            let temp = line.word_list[3].strip_prefix('<').unwrap().strip_suffix('>').unwrap().to_string();
                            file_to_include = format!("/usr/include/{}",temp);
                        }else if line.word_list[3].starts_with('"') && line.word_list[3].ends_with('"'){
                            let temp = line.word_list[3].strip_prefix('"').unwrap().strip_suffix('"').unwrap().to_string();
                            file_to_include = format!("{}{}",Path::new(&file_path.clone()).parent().unwrap().to_str().unwrap(),temp);
                        }else{
                            panic!("unknown include string {:?}",line);
                        }

                        // emit new file is started
                        if Path::new(&file_to_include).exists(){
                            output_buf.push(format!("# 1 {} 1 3 4",file_to_include));
                            let preprocessed_include = self.preprocess_code_string(read_to_string(file_to_include.clone()).unwrap(),file_to_include.clone());
                            output_buf.push(preprocessed_include);
                            output_buf.push(format!("# {} {} 2 3 4",line_num,file_to_include));
                        }else {
                            eprintln!("file we want to include doesnot exist! {:?}",file_to_include);
                        }

                    },
                    "if" => {
                        let mut nesting_level = 0;
                        let mut if_else_toggle = false; // if true we are in if_else
                        let mut if_true: Vec<Line> = vec![];
                        let mut if_else: Vec<Line> = vec![];
                        for enumerated_line in line_iter.by_ref(){
                            let line = enumerated_line.1;
                            let line_num = enumerated_line.0;
                            if line.word_list.is_empty(){
                                output_buf.push(String::new());
                                continue;
                            }
                            if line.word_list[0] == "#" {
                                let nesting_level_increaser = vec!["if","ifdef","ifndef"];
                                if nesting_level_increaser.contains(&line.word_list[1].as_str()){
                                    nesting_level += 1;
                                }
                                if line.word_list[1] == "else" && nesting_level == 0{
                                    if_else_toggle = true;
                                    continue;
                                }
                                if line.word_list[1] == "endif"{
                                    if nesting_level == 0{
                                        break;
                                    }else{
                                        nesting_level -= 1;
                                    }
                                }
                            }
                            if !if_else_toggle{
                                if_true.push(line.clone());
                            }else{
                                if_else.push(line.clone());
                            }
                        }
                        // now handle if statement:
                        fn process_if(define_map: &mut  HashMap<String,String>,line: Vec<String>,idx: usize) -> bool{
                            let mut is_true = false;
                            match line[idx].as_ref(){
                                "!" => {
                                    is_true = !process_if(define_map,line.clone(), idx + 1);
                                },
                                "(" => {
                                    
                                },
                                "||" => {

                                },
                                "&&" => {

                                },
                                "defined" => {
                                    let is_true = define_map.contains_key(&line[idx + 2]);
                                },
                                defined_to_check => {
                                    
                                },
                            }
                            is_true
                        }
                        let is_true = process_if(&mut self.define_map,line.word_list.clone(),3);
                    },
                    "ifdef" => {
                        let define_subject = line.word_list[3].clone();

                        let mut nesting_level = 0;
                        let mut if_else_toggle = false; // if true we are in if_else
                        let mut if_true: Vec<Line> = vec![];
                        let mut if_else: Vec<Line> = vec![];
                        for enumerated_line in line_iter.by_ref(){
                            let line = enumerated_line.1;
                            let line_num = enumerated_line.0;
                            if line.word_list.is_empty(){
                                output_buf.push(String::new());
                                continue;
                            }
                            if line.word_list[0] == "#" {
                                let nesting_level_increaser = vec!["if","ifdef","ifndef"];
                                if nesting_level_increaser.contains(&line.word_list[1].as_str()){
                                    nesting_level += 1;
                                }
                                if line.word_list[1] == "else" && nesting_level == 0{
                                    if_else_toggle = true;
                                    continue;
                                }
                                if line.word_list[1] == "endif"{
                                    if nesting_level == 0{
                                        break;
                                    }else{
                                        nesting_level -= 1;
                                    }
                                }
                            }
                            if !if_else_toggle{
                                if_true.push(line.clone());
                            }else{
                                if_else.push(line.clone());
                            }
                        }
                        if self.define_map.contains_key(&define_subject){
                            output_buf.push(self.execute_preprocessor(&mut if_true.iter().enumerate().fuse(), file_path.clone()));
                        }else{
                            output_buf.push(self.execute_preprocessor(&mut if_else.iter().enumerate().fuse(), file_path.clone()));
                        }
                    },
                    "ifndef" => {
                        let define_subject = line.word_list[3].clone();

                        let mut nesting_level = 0;
                        let mut if_else_toggle = false; // if true we are in if_else
                        let mut if_true: Vec<Line> = vec![];
                        let mut if_else: Vec<Line> = vec![];
                        for enumerated_line in line_iter.by_ref(){
                            let line = enumerated_line.1;
                            let line_num = enumerated_line.0;
                            if line.word_list.is_empty(){
                                output_buf.push(String::new());
                                continue;
                            }
                            if line.word_list[0] == "#" {
                                let nesting_level_increaser = vec!["if","ifdef","ifndef"];
                                if nesting_level_increaser.contains(&line.word_list[1].as_str()){
                                    nesting_level += 1;
                                }
                                if line.word_list[1] == "else" && nesting_level == 0{
                                    if_else_toggle = true;
                                    continue;
                                }
                                if line.word_list[1] == "endif"{
                                    if nesting_level == 0{
                                        break;
                                    }else{
                                        nesting_level -= 1;
                                    }
                                }
                            }
                            if !if_else_toggle{
                                if_true.push(line.clone());
                            }else{
                                if_else.push(line.clone());
                            }
                        }
                        if self.define_map.contains_key(&define_subject){
                            output_buf.push(self.execute_preprocessor(&mut if_else.iter().enumerate().fuse(), file_path.clone()));
                        }else{
                            output_buf.push(self.execute_preprocessor(&mut if_true.iter().enumerate().fuse(), file_path.clone()));
                        }
                    },
                    "else" => {unimplemented!("on line: {:?}",line)},
                    "elif" => {unimplemented!("on line: {:?}",line)},
                    "define" => {
                        let define_subject = line.word_list[3].clone();
                        let define_val = if line.word_list.len() > 5{
                            line.word_list[5].clone()
                        }else{
                            String::new()
                        };
                        if let std::collections::hash_map::Entry::Vacant(e) = self.define_map.entry(define_subject.clone()) {
                            e.insert(define_val);
                        } else {
                            eprintln!("'{}' is already defined! {:?}",define_subject,line);
                            self.define_map.insert(define_subject,define_val);
                        }
                    },
                    "undef" => {
                        let undefine_subject = line.word_list[3].clone();
                        self.define_map.remove(&undefine_subject);
                    },
                    "error" => {unimplemented!("on line: {:?}",line)},
                    "line" => {unimplemented!("on line: {:?}",line)},
                    "pragma" => {unimplemented!("on line: {:?}",line)},
                    unknown_command => panic!("Unknown Preprocessor command!: {}", unknown_command),
                }
            } else {
                // flush to our output
                output_buf.push(format!(
                    "{}{}",
                    (0..line.ident_level)
                        .map(|_| String::new())
                        .collect::<Vec<String>>()
                        .join(" "),
                    line.word_list.join("")
                ));
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
static PUNCTUATOR_LIST: &[&str] = &["#", "(", ")", "!", "||", "&&","=="];

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
        if c == '/'{
            if chars.peek().is_some() && chars.peek().unwrap() == &'/'{
                chars.next();
                while let Some(c) = chars.next(){
                    if c == '\n'{
                        if !current_word.is_empty() {
                            current_line.word_list.push(current_word.clone());
                            current_word = String::new();
                        }
                        line_buf.push(current_line);
                        current_line = Line {
                            ident_level: 0,
                            word_list: vec![],
                        };
                        break;
                    }
                    //nothing till end of line
                }
                continue;
            }else if chars.peek().is_some() && chars.peek().unwrap() == &'*'{
                while let Some(c) = chars.next(){
                    if c == '\n'{
                        if !current_word.is_empty() {
                            current_line.word_list.push(current_word.clone());
                            current_word = String::new();
                        }
                        line_buf.push(current_line);
                        current_line = Line {
                            ident_level: 0,
                            word_list: vec![],
                        };
                    }
                    if c == '*' && chars.peek().is_some() && chars.peek().unwrap() == &'/'{
                        chars.next();
                        if !current_word.is_empty() {
                            current_line.word_list.push(current_word.clone());
                            current_word = String::new();
                        }
                        break;
                    }
                    //nothing till end of multiline
                }
                continue;
            }
        }
        if current_line.word_list.is_empty() && current_word.is_empty() {
            if c == '\t' {
                current_line.ident_level += 4;
            }
            if c == ' ' {
                current_line.ident_level += 1;
            }
        }
        if PUNCTUATOR_LIST.contains(&current_word.as_str()) && !in_string {
            current_line.word_list.push(current_word);
            current_word = String::new();
        }
        if PUNCTUATOR_LIST.contains(&c.to_string().as_str()) {
            if !current_word.is_empty() {
                current_line.word_list.push(current_word.clone());
                current_word = String::new();
            }
            current_line.word_list.push(c.to_string());
        } else if c == '\\'
            && !in_string
            && chars.peek().is_some()
            && *chars.peek().unwrap() == '\n'
        {
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
        } else if c == ' ' || c == '\t'{
            if in_string {
                current_word.push(c);
            } else if !current_word.is_empty() {
                current_line.word_list.push(current_word);
                current_word = String::new();
                current_word.push(' ');
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

    for line in &line_buf {
        println!("{:?}", line);
    }

    line_buf
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Line {
    ident_level: usize,
    word_list: Vec<String>,
}


enum PreproccesorDirective{

}

// if statements have a condition
enum PreprocessorIfCondition{
    Equals,
    Not,
    And,
    Or,
}