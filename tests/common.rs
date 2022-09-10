use std::{path::Path, process::Command};

pub fn execute_and_cleanup_capturing_stdout(path_to_c_file: &str) -> String {
    println!("path: {:?}", Path::new(&path_to_c_file).with_extension("c"));
    println!("dir: {:?}", Path::new(".").canonicalize());

    let _output_from_compiler = Command::new("cargo")
        .args(&["r", "--", "-g", path_to_c_file])
        .output()
        .unwrap();
    println!("compiler done");
    let _output_from_linker = Command::new("cc")
        .args(&[
            Path::new(&path_to_c_file)
                .with_extension("o")
                .to_str()
                .unwrap(),
            "-o",
            Path::new(&path_to_c_file)
                .with_extension("out")
                .to_str()
                .unwrap(),
            "-no-pie",
            "-static",
        ])
        .output()
        .unwrap();
    println!("linker done");
    let output_from_program = Command::new(
        Path::new(&path_to_c_file)
            .with_extension("out")
            .to_str()
            .unwrap(),
    )
    .args(&[""])
    .output()
    .unwrap();
    println!("programm done");
    let _output_from_cleanup = Command::new("rm")
        .args(&[
            Path::new(&path_to_c_file)
                .with_extension("o")
                .to_str()
                .unwrap(),
            Path::new(&path_to_c_file)
                .with_extension("o")
                .to_str()
                .unwrap(),
            Path::new(&path_to_c_file)
                .with_extension("out")
                .to_str()
                .unwrap(),
            Path::new(&path_to_c_file)
                .with_extension("i")
                .to_str()
                .unwrap(),
            Path::new(&path_to_c_file)
                .with_extension("ast")
                .to_str()
                .unwrap(),
        ])
        .output()
        .unwrap();
    println!("cleanup done");

    String::from_utf8(output_from_program.stdout).unwrap()
}
