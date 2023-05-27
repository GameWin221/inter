use std::{
    process::Command
};

use crate::{
    INTERMEDIATE_FILE_NAME,
    OUT_FILE_NAME
};

#[derive(Default, Debug, Clone, Copy)]
pub enum Compiler {
    #[default] 
    GXX,
}

#[derive(Default, Debug, Clone, Copy)]
pub enum STD {
    CXX98,
    CXX11,
    CXX14,
    #[default] 
    CXX17,
    CXX20,
}

#[derive(Default, Debug, Clone, Copy)]
pub enum Optimization {
    None,
    Size,
    #[default] 
    Speed,
    Balanced
}

#[derive(Default, Debug, Clone)]
pub struct CompilationConfig {
    compiler: Compiler,
    std: STD,
    optimization: Optimization
}

pub fn compile(config: &CompilationConfig) {
    let working_dir = std::env::current_dir().expect("Failed to get the current working directory!");
    let working_dir = working_dir.to_str().unwrap();

    let intermediate_file_path = format!("{working_dir}/{INTERMEDIATE_FILE_NAME}");

    let compiler = match config.compiler {
        Compiler::GXX => "g++",
    };

    let std = match config.std {
        STD::CXX98 => "-std=c++98",
        STD::CXX11 => "-std=c++11",
        STD::CXX14 => "-std=c++14",
        STD::CXX17 => "-std=c++17",
        STD::CXX20 => "-std=c++20",
    };

    let optimization = match config.optimization {
        Optimization::None => "-O0",
        Optimization::Size => "-Os",
        Optimization::Speed => "-Ofast",
        Optimization::Balanced => "-O2",
    };

    Command::new(compiler)
        .arg(intermediate_file_path)
        .arg(std)
        .arg(optimization)
        .arg("-o")
        .arg(OUT_FILE_NAME)
        .arg("-Wall")
        .spawn()
        .expect("Failed to run the compiler!");
}