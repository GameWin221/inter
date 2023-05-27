use std::{
    time::Instant,
    collections::HashMap,
    fs::File, 
    io::prelude::*
};

use crate::{
    parser::{Function, FnParam}, 
    types::*,
    INTERMEDIATE_FILE_NAME
};

fn translate_func_declaration(function: &Function) -> String {
    let mut buffer = String::with_capacity(32);

    buffer.push_str(translate_type(&function.return_type).as_str());
    buffer.push_str(function.name.as_str());
    buffer.push_str(&translate_operator(&Operator::BrOpen));

    for (i, param) in function.params.iter().enumerate() {
        buffer.push_str(&translate_type(&param.typename));
        buffer.push_str(&param.name);

        if i < function.params.len() - 1 {
            buffer.push_str(&translate_operator(&Operator::Comma));
        }
    }

    buffer.push_str(&translate_operator(&Operator::BrClose));

    buffer
}

fn translate_func_definition(function: &Function) -> String {
    let mut buffer = String::with_capacity(64);

    buffer.push_str(&translate_func_declaration(function));
    buffer.push_str(&translate_operator(&Operator::ScOpen));

    for token in &function.content {
        buffer.push_str(translate_token(token).as_str());
    }

    buffer.push_str(&translate_operator(&Operator::ScClose));

    buffer
}

pub fn translate_file(functions: &HashMap<String, Function>, globals: &Vec<Token>) -> Result<f32, ()> {
    let start_time = Instant::now();

    let mut buffer = String::with_capacity(512);

    buffer.push_str("/* ===STD=== */\n");
    buffer.push_str("#include <string>\n");
    buffer.push_str("#include <vector>\n");
    buffer.push_str("#define _std_inter_main main\n");

    buffer.push_str("\n/* ===GLOBALS=== */\n");

    let mut skip_keywords = 0;
    for i in 0..globals.len() {
        if skip_keywords > 0 {
            skip_keywords -= 1 ;
            continue;
        }

        if let Token::Key(Keyword::Import) = globals[i] {
            if let Token::Id(s) = &globals[i+1] {
                buffer.push_str(&crate::cppstd::get_cpp_header(&s));
                skip_keywords = 2;
            }
        } else {
            buffer.push_str(&translate_token(&globals[i]));
        }
    }

    buffer.push_str("\n/* ===FUNCTION DECLARATIONS=== */\n");

    for (_name, func) in functions {
        buffer.push_str(&translate_func_declaration(func));
        buffer.push_str(&translate_operator(&Operator::EOL));
    }

    buffer.push_str("\n/* ===FUNCTION DEFINITIONS=== */\n");

    for (_name, func) in functions {
        buffer.push_str(&translate_func_definition(func));
    }

    let mut file = File::create(INTERMEDIATE_FILE_NAME).expect("Failed to create/open the intermediate file!");
    file.write_all(buffer.as_bytes()).expect("Failed to write to the intermediate file!");

    Ok((Instant::now() - start_time).as_secs_f32())
}