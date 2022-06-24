use std::{fs::{read_to_string, File}, io::Write, path::Path};
use structopt::StructOpt;

/// A StructOpt example
#[derive(StructOpt, Debug)]
#[structopt()]
struct Opt {
    /// Silence all output
    #[structopt(short = "q", long = "quiet")]
    quiet: bool,
    /// Verbose mode (-v, -vv, -vvv, etc)
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: usize,
    /// Use Internal preprocessor
    #[structopt(short = "ipp", long = "internal-preprocessor")]
    internal_preprocessor: bool,
    /// Use gcc preprocessor
    #[structopt(short = "gccpp", long = "gcc-preprocessor")]
    gcc_preprocessor: bool,
}

mod lexer;
mod preprocessor;

use preprocessor::Preprocessor;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let opt = Opt::from_args();

    let log_level = if opt.quiet{
        log::LevelFilter::Off
    } else {
        match opt.verbose{
            0 => log::LevelFilter::Info,
            1 => log::LevelFilter::Error,
            2 => log::LevelFilter::Warn,
            3 => log::LevelFilter::Info,
            4 => log::LevelFilter::Debug,
            5 => log::LevelFilter::Trace,
            _ => log::LevelFilter::Trace,
        }
    };

    env_logger::builder().format_timestamp(None).filter_level(log_level).init();
    
    // new()
    //     .module(module_path!())
    //     .quiet(opt.quiet)
    //     .verbosity(opt.verbose)
    //     .init()
    //     .unwrap();


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

    if opt.internal_preprocessor{
        let mut preprocessor = Preprocessor::new();
        let preprocessed_file = preprocessor.preprocess_code_string(read_in_file,in_file_path.to_string());
        let preprocessed_file = preprocessor.replace_final(preprocessed_file);
    
        // println!("-------\n{}\n-------", preprocessed_file);
    
        let mut file = File::create(Path::new(&in_file_path).with_extension("j.i"))?;
        file.write_all(preprocessed_file.as_bytes())?;
    }
    if opt.gcc_preprocessor{
        // TODO!
    }

    Ok(())
}
