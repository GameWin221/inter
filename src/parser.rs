use crate::lexer::{Keyword, Token, Type, Operator, DataType};
use core::panic;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct FnParam {
    pub typename: Type,
    pub name: String,
    pub value: DataType
}

#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    pub return_type: Type,
    pub name: String,
    pub params: Vec<FnParam>,
    pub content: Vec<Token>
}

pub fn parse_globals(tokens: &Vec<Token>) -> Vec<Token> {
    if let Some(global_key_id) = tokens.iter().position(|tok| *tok == Token::Key(Keyword::Global)) {
        if tokens[global_key_id+1] != Token::Op(Operator::ScOpen) {
            println!("global scope opening must be a {{ - (tokens[{global_key_id}+1])");
            return Vec::new();
        }

        let global_start = global_key_id + 2;
        let mut global_end = global_start + 1;

        for i in global_start..tokens.len() {
            if tokens[i] == Token::Op(Operator::ScClose) {
                global_end = i;
                break;
            }
        }

        return tokens[global_start..global_end].to_vec()
    } else {
        return Vec::new();
    }
}
pub fn parse_functions(tokens: &Vec<Token>) -> HashMap<String, Function> {
    let mut functions = HashMap::new();
    
    let mut function_positions = Vec::new();

    for i in 0..tokens.len()-2 {
        if matches!(tokens[i], Token::Type(_)) && matches!(tokens[i+1], Token::Id(_)) && matches!(tokens[i+2], Token::Op(Operator::BrOpen)) {
            function_positions.push(i);
        }
    }

    for position in function_positions {
        let mut fn_start = 0;
        let mut fn_end = 0;

        let mut params_start = position+3;
        let mut params_end = 0;

        for i in position+2..tokens.len() {
            if tokens[i] == Token::Op(Operator::ScOpen) {
                fn_start = i+1;
                break;
            }

            params_end = i;
        }

        let mut params = Vec::new();

        for i in params_start..params_end {
            if matches!(tokens[i], Token::Type(_)) {
                let arg_type = match tokens[i].clone() {
                    Token::Type(typename) => typename,
                    _ => panic!("Unknown data type!")
                };

                let arg_name = match tokens[i+1].clone() {
                    Token::Id(id) => id,
                    _ => panic!("Unknown arg name!")
                };

                let arg_value = match arg_type {
                    Type::Real => DataType::Real(0.0),
                    Type::Int => DataType::Int(0),
                    Type::String => DataType::String(String::from("")), 
                    Type::Bool => DataType::Bool(false),
                    Type::Func => DataType::Func(),
                };

                params.push(FnParam { 
                    typename: arg_type,
                    name: arg_name,
                    value: arg_value 
                });
            }
        }

        let mut depth = 0;
        for i in fn_start..tokens.len() {
            if tokens[i] == Token::Op(Operator::ScOpen) {
                depth += 1;
            } else if tokens[i] == Token::Op(Operator::ScClose) {
                depth -= 1;
            }

            if depth == -1 {
                fn_end = i;
                break;
            }
        }

        let return_type = match tokens[position].clone() {
            Token::Type(typename) => typename,
            _ => Type::Func
        };
        let name = match tokens[position+1].clone() {
            Token::Id(id) => id,
            _ => panic!("Unknown function name!")
        };

        let content = tokens[fn_start..fn_end].to_vec();

        functions.insert(name.clone(), Function { 
            return_type, 
            name,
            params,
            content 
        });
    }

    functions
}