use log::{debug, error, info};
use std::{
    fs::{read_to_string, File},
    io::Write,
    path::Path,
    process::Command,
    time::Instant,
};
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
    #[structopt(short = "f", long = "flush-all")]
    flush_all_intermediate: bool,
}

mod environment_builder;
mod lexer;
mod parser;
mod preprocessor;

use preprocessor::Preprocessor;

use crate::{lexer::Lexer, parser::CParser, environment_builder::EnvironmentController};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let timer_start = Instant::now();
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

    if !opt.quiet {
        println!(
            r#"      _             _       ____ ____ 
    | | __ _ _ __ | | __  / ___/ ___|
 _  | |/ _` | '_ \| |/ / | |  | |    
| |_| | (_| | | | |   <  | |__| |___ 
 \___/ \__,_|_| |_|_|\_\  \____\____|
                                     "#
        );
        println!("by Iquiji --- v0.0.4");
    }

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
        preprocessed_file = String::from_utf8(
            Command::new("gcc")
                .args(&["-E", "-std=c11", "-undef", &in_file_path])
                .output()?
                .stdout,
        )?;
    } else {
        error!("require either Internal or GCC preprocessor! see -h for help!");
        return Ok(());
    }
    if opt.flush_all_intermediate {
        let timer_start_flushing_preprocessed_file = Instant::now();
        info!(
            "Starting Flushing of Preprocessed File: {:?}",
            Path::new(&in_file_path).with_extension("i")
        );
        let mut preprocessed_file_handle =
            File::create(Path::new(&in_file_path).with_extension("i")).unwrap();

        // Write preprocessed_file_handle
        preprocessed_file_handle
            .write_all(preprocessed_file.as_bytes())
            .unwrap();
        let timer_end_flushing_preprocessed_file = timer_start_flushing_preprocessed_file.elapsed();
        info!(
            "Flushing Flushing of Preprocessed File took: {:?}",
            timer_end_flushing_preprocessed_file
        );
    }

    let timer_start_lexing = Instant::now();
    info!("Starting Lexing of file: {:?}", in_file_path);

    // call lexer
    let mut lexer = Lexer::new();
    let token_arr = lexer.string_to_token_arr(preprocessed_file);

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

    let timer_end_parsing = timer_start_parsing.elapsed();
    info!("Parsing of file took: {:?}", timer_end_parsing);

    if opt.flush_all_intermediate {
        let timer_start_flushing_ast = Instant::now();
        info!(
            "Starting Flushing of Ast: {:?}",
            Path::new(&in_file_path).with_extension("ast")
        );
        let mut ast_file_handle =
            File::create(Path::new(&in_file_path).with_extension("ast")).unwrap();

        // Write ast_file_handle
        ast_file_handle
            .write_all(serde_yaml::to_string(&parsed).unwrap().as_bytes())
            .unwrap();
        let timer_end_flushing_ast = timer_start_flushing_ast.elapsed();
        info!("Flushing of Ast took: {:?}", timer_end_flushing_ast);
    }

    let timer_start_environment = Instant::now();
    info!("Starting Environment of file: {:?}", in_file_path);
    
    let mut controller = EnvironmentController::new();
    controller.build(parsed);


    let timer_end_parsing = timer_start_parsing.elapsed();
    info!("Building of Environment took: {:?}", timer_end_parsing);

    let timer_end = timer_start.elapsed();
    info!("Compiling took {:?} in Total", timer_end);

    Ok(())
}
