use crate::types::{Keyword, Token, Type, Operator, DataType, default_type_value};
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

pub fn process(tokens: Vec<Token>) -> (HashMap<String, Function>, Vec<Token>) {
    (parse_functions(&tokens), parse_globals(&tokens))
}

fn parse_tokens(tokens: &[Token]) -> Vec<Token> {
    tokens.to_vec()
}   

fn parse_globals(tokens: &Vec<Token>) -> Vec<Token> {
    if let Some(global_keyword_index) = tokens.iter().position(|tok| *tok == Token::Key(Keyword::Global)) {

        // The first global token must be Token::Op(Operator::ScOpen) - {
        let global_start = tokens[global_keyword_index..global_keyword_index+2].iter().position(
            |tok| *tok == Token::Op(Operator::ScOpen)
        ).expect("The \"global\" region must have an opening scope!") + global_keyword_index + 1;

        // The last global token must be Token::Op(Operator::ScClose) - }
        let global_end = tokens[global_start..].iter().position(
            |tok| *tok == Token::Op(Operator::ScClose)
        ).expect("The \"global\" region must have a closing scope!") + global_start;

        return parse_tokens(&tokens[global_start..global_end])
    } else {
        return Vec::new();
    }
}

fn parse_functions(tokens: &Vec<Token>) -> HashMap<String, Function> {
    let mut functions = HashMap::new();
    
    let mut function_positions = Vec::new();
    
    for i in 0..tokens.len()-2 {
        if matches!(tokens[i], Token::Type(_)) {
            if matches!(tokens[i+1], Token::Id(_)) && matches!(tokens[i+2], Token::Op(Operator::BrOpen)) {
                function_positions.push(i);
            }
        }
    }

    for position in function_positions {
        let params_start = tokens[position+2..position+3].iter().position(
            |tok| *tok == Token::Op(Operator::BrOpen)
        ).expect("Function must have its param opening bracket!") + position + 3;
        
        let params_end = tokens[params_start..].iter().position(
            |tok| *tok == Token::Op(Operator::BrClose)
        ).expect("Function must have its param closing bracket!") + params_start;

        let fn_start = tokens[params_end+1..params_end+2].iter().position(
            |tok| *tok == Token::Op(Operator::ScOpen)
        ).expect("Function must have its opening scope!") + params_end + 2;

        let mut fn_end = fn_start;

        let return_type = match tokens[position].clone() {
            Token::Type(typename) => typename,
            _ => panic!("Function type must be a valid type!")
        };
        let name = match tokens[position+1].clone() {
            Token::Id(id) => id,
            _ => panic!("Function name must not be a reserved keyword or a type!")
        };

        let params_iter = tokens[params_start..params_end].iter().filter(
            |&tok| *tok != Token::Op(Operator::Comma)
        );

        // Take pairs of tokens (param_type: &Type, param_name: &String), with offset of 1 element and step by 2 elements
        let params = params_iter.clone().step_by(2).zip(params_iter.skip(1).step_by(2)).map(|(pt, pn)| {
            let param_type = match pt {
                Token::Type(typename) => typename,
                _ => panic!("Param type must be a valid type!")
            }; 

            let param_name = match pn {
                Token::Id(id) => id,
                _ => panic!("Param name must not be a reserved keyword or a type!")
            };

            FnParam { 
                typename: param_type.clone(),
                name: param_name.clone(),
                value: default_type_value(param_type)
            }
        }).collect();

        // Find the first Token::Op(Operator::ScClose) that is out of the function's body (not nested inside, i.e. depth = -1)
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

        let content = parse_tokens(&tokens[fn_start..fn_end]);

        functions.insert(name.clone(), Function { 
            return_type, 
            name,
            params,
            content 
        });
    }

    functions
}