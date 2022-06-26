mod helper_funcs;
mod token_types;

use std::{fs::read_to_string, fmt};

use log::error;
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
        write!(f, "{}-{}: '{}' -- {:?}", self.loc.line,self.loc.collumn,self.original,self.t_type)
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OriginalLocation {
    file: String,
    line: usize,
    collumn: usize,
}

pub struct Lexer{
    current_loc: OriginalLocation,
}
impl Lexer{
    pub fn new() -> Self{
        Self { current_loc: OriginalLocation { file: String::new(), line: 0, collumn: 0 } }
    }

    pub fn string_to_token_arr(&mut self,string_to_lexer: String) -> Vec<CToken> {
        let mut buf = vec![];

        
        let mut in_string = false;

        for line in string_to_lexer.lines(){
            // check if theres a line resync directive:
            if line.trim_start().starts_with('#'){
                // line directive
                if let Err(err) = self.handle_line_resync_from_preprocessor(line.trim_start()){
                    error!("error while trying to execute line resync directive from preprocessor!: {:?}",&err);
                    panic!();
                }
            } else {
                let mut current_token_string = String::new();
                
                // main loop where we check for Tokens
                let mut char_line_iter = line.chars().into_iter().peekable().fuse();

                while let Some(character) = char_line_iter.next(){
                    current_token_string = current_token_string.trim_start().to_string();

                    if helper_funcs::is_punctuator(&current_token_string){
                        // punctuator
                        buf.push(CToken { t_type: CTokenType::Punctuator, original: current_token_string.clone(), loc: self.current_loc.clone() });
                        current_token_string = String::new();
                    } else if helper_funcs::is_nondigit(character) {
                        // identifier
                        current_token_string.push(character);
                        for character in char_line_iter.by_ref(){
                            // as long as we have digit or nondigit
                            if helper_funcs::is_nondigit(character) || helper_funcs::is_digit(character){
                                current_token_string.push(character);
                            } else{
                                if let Some(keyword) = CKeyword::to_keyword(&current_token_string){
                                    buf.push(CToken { t_type: CTokenType::Keyword(keyword), original: current_token_string.clone(), loc: self.current_loc.clone() });
                                } else {
                                    buf.push(CToken { t_type: CTokenType::Identifier, original: current_token_string.clone(), loc: self.current_loc.clone() });
                                }
                                current_token_string = String::new();
                                current_token_string.push(character);
                                break;
                            }
                        }
                        
                    } else if character == '"' || !current_token_string.is_empty() && current_token_string.clone().ends_with('\"'){
                        if !current_token_string.is_empty() && current_token_string.clone().ends_with('\"'){
                            current_token_string.remove(current_token_string.len() - 1);
                        }
                        for character in char_line_iter.by_ref(){
                            error!("string: {}",current_token_string);

                            // as long as we have are still in a string:
                            if character == '"'{
                                if current_token_string.ends_with('\\'){
                                    current_token_string.remove(current_token_string.len() - 1);
                                    current_token_string.push('"');
                                    continue;
                                } else {
                                    buf.push(CToken { t_type: CTokenType::StringLiteral, original: current_token_string.clone().trim_start_matches('"').trim_end_matches('"').to_string(), loc: self.current_loc.clone() });
                                    current_token_string = String::new();
                                    break;
                                }
                            } else{
                                current_token_string.push(character);
                            }
                        }
                    } else if current_token_string.len() == 1 && helper_funcs::is_digit(current_token_string.clone().chars().next().unwrap()) {
                        // number
                        current_token_string.push(character);
                        let mut point_seperator_reached = false;
                        for character in char_line_iter.by_ref(){
                            // as long as we have digit or nondigit
                            if helper_funcs::is_nondigit(character) || helper_funcs::is_digit(character){
                                current_token_string.push(character);
                            } else if character == '.'{
                                if point_seperator_reached{
                                    panic!("Second Point seperator in number")
                                }
                                point_seperator_reached = true;
                            }else{
                                buf.push(CToken { t_type: CTokenType::Constant, original: current_token_string.clone(), loc: self.current_loc.clone() });
                                current_token_string = String::new();
                                current_token_string.push(character);
                                break;
                            }
                        }
                    } else if character.is_whitespace(){
                        // it is a whitespace so we do nothing but maybe error if current token string is nonempty
                        if !current_token_string.is_empty(){
                            error!("!current_token_string.is_empty(): {:?}",current_token_string);
                        }
                    } else {
                        self.current_loc.collumn += 1;
                        current_token_string.push(character);
                    }
                }
                
                if !current_token_string.is_empty(){
                    buf.push(CToken { t_type: CTokenType::Identifier, original: current_token_string, loc: self.current_loc.clone() });
                }
            }
            self.current_loc.line += 1;
            self.current_loc.collumn = 0;
        }
    
        buf
    }
    
    pub fn file_to_token_arr(&mut self,file_to_lexer: String) -> Result<Vec<CToken>, Box<dyn std::error::Error>> {
        let read_in_file = read_to_string(file_to_lexer)?;
    
        Ok(self.string_to_token_arr(read_in_file))
    }
    pub fn handle_line_resync_from_preprocessor(&mut self,sync_string: &str) -> Result<(), Box<dyn std::error::Error>>{
        let split_sync_string: Vec<&str> = sync_string.split_ascii_whitespace().collect();
        let get_resync_line_num: usize = split_sync_string[1].parse()?;
        let file_resync = split_sync_string[2];

        self.current_loc.line = get_resync_line_num;
        self.current_loc.collumn = 0;
        self.current_loc.file = file_resync.to_string();

        Ok(())
    }
}