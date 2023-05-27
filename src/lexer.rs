use crate::types::*;

pub fn process(source_code: &String) -> Vec<Token> {
    let mut words = Vec::with_capacity(source_code.len() / 4);

    // Do not filter out whitespaces yet because they might be needed in strings
    for split in source_code.split_inclusive(
        |c| c == ' ' || is_operator(c)
    ).filter(
        |&s| s.len() > 0
    ) {
        if split.len() > 1 && split.ends_with(|c| is_operator(c)) { 
            let (a, b) = split.split_at(split.len()-1);

            words.push(a.to_string());
            words.push(b.to_string());
        } else {
            words.push(split.to_string());
        }
    }

    let mut tokens = Vec::with_capacity(words.len());

    let mut is_quoting = false;
    let mut is_commenting = false;

    for word in &words {
        if !is_commenting {
            if !is_quoting {
                // Filter out whitespaces
                let word = word.trim();
                
                if word.len() < 1 {
                    continue;
                }
    
                if let Some(op) = to_operator(word.as_bytes()[0] as char) {
                    if op == Operator::Quote {
                        is_quoting = true;
                    } else if op == Operator::Hash {
                        is_commenting = true;
                        continue;
                    }
        
                    tokens.push(Token::Op(op));
                } else if let Some(tp) = to_type(word) {
                    tokens.push(Token::Type(tp));
                } else if let Some(key) = to_keyword(word) {
                    tokens.push(Token::Key(key));
                } else if let Ok(val) = word.parse::<i32>() {
                    tokens.push(Token::Data(DataType::Int(val)));
                }else if let Ok(val) = word.parse::<f32>() {
                    tokens.push(Token::Data(DataType::Real(val)));
                } else if let Ok(val) = word.parse::<bool>() {
                    tokens.push(Token::Data(DataType::Bool(val)));
                } else {
                    tokens.push(Token::Id(format!("_std_inter_{}", word)));
                }
            } else {
                if let Some(op) = to_operator(word.as_bytes()[0] as char) {
                    if op == Operator::Quote {
                        is_quoting = false;
                        tokens.push(Token::Op(op));
                    } else {
                        tokens.push(Token::Data(DataType::String(word.clone())))
                    }
                } else {
                    tokens.push(Token::Data(DataType::String(word.clone())))
                }
            }
        } else {
            if let Some(op) = to_operator(word.as_bytes()[0] as char) {
                if op == Operator::Hash {
                    is_commenting = false;
                }
            }
        }
    }

    tokens
}