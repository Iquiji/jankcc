use log::{debug, error, info, warn};
use std::{fs::read_to_string, process::Command, time::Instant};
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
    /// Input file
    input_file_path: String,
}

mod lexer;
mod parser;
mod preprocessor;

use preprocessor::Preprocessor;

use crate::{lexer::Lexer, parser::CParser};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();

    let log_level = if opt.quiet {
        log::LevelFilter::Off
    } else {
        match opt.verbose {
            0 => log::LevelFilter::Info,
            1 => log::LevelFilter::Error,
            2 => log::LevelFilter::Warn,
            3 => log::LevelFilter::Info,
            4 => log::LevelFilter::Debug,
            5 => log::LevelFilter::Trace,
            _ => log::LevelFilter::Trace,
        }
    };

    env_logger::builder()
        .format_timestamp(None)
        .filter_level(log_level)
        .init();

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
    println!("by Iquiji --- v0.0.3");

    let in_file_path = opt.input_file_path;

    let read_in_file = read_to_string(in_file_path.clone())?;

    let preprocessed_file: String;

    if opt.internal_preprocessor {
        let mut preprocessor = Preprocessor::new();
        let preprocessed_file_temp =
            preprocessor.preprocess_code_string(read_in_file, in_file_path.clone());
        preprocessed_file = preprocessor.replace_final(preprocessed_file_temp);

        // println!("-------\n{}\n-------", preprocessed_file);

        // let mut file = File::create(Path::new(&in_file_path).with_extension("j.i"))?;
        // file.write_all(preprocessed_file.as_bytes())?;
    } else if opt.gcc_preprocessor {
        // TODO!
        preprocessed_file = String::from_utf8(
            Command::new("gcc")
                .args(&["-E", &in_file_path])
                .output()?
                .stdout,
        )?;
    } else {
        error!("require either Internal or GCC preprocessor! see -h for help!");
        return Ok(());
    }

    let timer_start_lexing = Instant::now();
    info!("Starting Lexing of file: {:?}", in_file_path);

    // call lexer
    let mut lexer = Lexer::new();
    let token_arr = lexer.string_to_token_arr(preprocessed_file.clone());

    for token in &token_arr {
        debug!("{}", token);
    }

    let timer_end_lexing = timer_start_lexing.elapsed();
    info!("Lexing of file took: {:?}", timer_end_lexing);

    let timer_start_parsing = Instant::now();
    info!("Starting Parsing of file: {:?}", in_file_path);

    // do da stuff
    let mut parser = CParser::new(token_arr);
    let parsed = parser.parse();
    info!("Parsed Program: {:?}", parsed);

    let timer_end_parsing = timer_start_parsing.elapsed();
    info!("Parsing of file took: {:?}", timer_end_parsing);

    Ok(())
}
