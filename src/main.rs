use std::{fs::{read_to_string, File}, io::Write, path::Path};

mod lexer;
mod preprocessor;

use preprocessor::Preprocessor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        r#"      _             _       ____ ____ 
    | | __ _ _ __ | | __  / ___/ ___|
 _  | |/ _` | '_ \| |/ / | |  | |    
| |_| | (_| | | | |   <  | |__| |___ 
 \___/ \__,_|_| |_|_|\_\  \____\____|
                                     "#
    );
    println!("by Iquiji --- v0.0.1");

    let in_file_path = "./C_Testfiles/hello_world/hello_world.c";

    let read_in_file = read_to_string(in_file_path)?;

    let preprocessed_file = Preprocessor::new().preprocess_code_string(read_in_file,in_file_path.to_string());

    println!("-------\n{}\n-------", preprocessed_file);

    let mut file = File::create(Path::new(&in_file_path).with_extension("j.i"))?;
    file.write_all(preprocessed_file.as_bytes())?;

    Ok(())
}
