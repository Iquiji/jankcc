use chumsky::Parser;
use log::{debug, error, info};
use std::{fs::read_to_string, process::Command, time::Instant};
use structopt::StructOpt;
use ariadne::{Color, Fmt, Label, Report, ReportKind, Source};

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
mod preprocessor;
mod parser;

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
    
    for token in token_arr {
        debug!("{}", token);
    }

    let timer_end_lexing = timer_start_lexing.elapsed();
    info!("Lexing of file took: {:?}", timer_end_lexing);

    let timer_start_parsing = Instant::now();
    info!("Starting Lexing-New of file: {:?}", in_file_path);
    
    let (tokens, errs) = crate::parser::lexer().parse_recovery(preprocessed_file.as_str());

    println!("tokens: {:?}",tokens);
    println!("errs: {:?}",errs);
            
    errs.into_iter()
        .map(|e| e.map(|c| c.to_string()))
        .for_each(|e| {
            let report = Report::build(ReportKind::Error, (), e.span().start);

            println!("err: {:?}",e);

            let report = match e.reason() {
                chumsky::error::SimpleReason::Unclosed { span, delimiter } => report
                    .with_message(format!(
                        "Unclosed delimiter {}",
                        delimiter.fg(Color::Yellow)
                    ))
                    .with_label(
                        Label::new(span.clone())
                            .with_message(format!(
                                "Unclosed delimiter {}",
                                delimiter.fg(Color::Yellow)
                            ))
                            .with_color(Color::Yellow),
                    )
                    .with_label(
                        Label::new(e.span())
                            .with_message(format!(
                                "Must be closed before this {}",
                                e.found()
                                    .unwrap_or(&"end of file".to_string())
                                    .fg(Color::Red)
                            ))
                            .with_color(Color::Red),
                    ),
                chumsky::error::SimpleReason::Unexpected => report
                    .with_message(format!(
                        "{}, expected {}",
                        if e.found().is_some() {
                            "Unexpected token in input"
                        } else {
                            "Unexpected end of input"
                        },
                        if e.expected().len() == 0 {
                            "something else".to_string()
                        } else {
                            e.expected()
                                .map(|expected| match expected {
                                    Some(expected) => expected.to_string(),
                                    None => "end of input".to_string(),
                                })
                                .collect::<Vec<_>>()
                                .join(", ")
                        }
                    ))
                    .with_label(
                        Label::new(e.span())
                            .with_message(format!(
                                "Unexpected token {}",
                                e.found()
                                    .unwrap_or(&"end of file".to_string())
                                    .fg(Color::Red)
                            ))
                            .with_color(Color::Red),
                    ),
                chumsky::error::SimpleReason::Custom(msg) => report.with_message(msg).with_label(
                    Label::new(e.span())
                        .with_message(format!("{}", msg.fg(Color::Red)))
                        .with_color(Color::Red),
                ),
            };

            report.finish().print(Source::from(&preprocessed_file)).unwrap();
        });
    Ok(())
}
