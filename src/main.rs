use compiler::CompilationConfig;

pub mod lexer;
pub mod parser;
pub mod translator;
pub mod compiler;
pub mod cppstd;
pub mod types;

pub const IN_FILE_NAME: &str = "main.int";
pub const INTERMEDIATE_FILE_NAME: &str = "main_intermediate.cpp";
pub const OUT_FILE_NAME: &str = "main.exe";

fn main() {
    let source_code = std::fs::read_to_string(IN_FILE_NAME).expect("Failed to read the source file!");
    let tokens = lexer::process(&source_code);     
    let (functions, globals) = parser::process(tokens);

    match translator::translate_file(&functions, &globals) {
        Err(_error) => {
            println!("The program failed to translate due to an error\n");
        }
        Ok(time) => {
            println!("The program was translated successfully in {time} seconds!");

            compiler::compile(&CompilationConfig::default());
        }
    };
}