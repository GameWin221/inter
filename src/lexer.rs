#[derive(Clone, Debug, PartialEq)]
pub enum Keyword {
    If,
    Else,
    Loop,
    Return,
    Import,
    Global,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Eq,

    ScOpen,
    ScClose,

    BrOpen,
    BrClose,

    Quote,
    Comma,
    Dot,

    EOL,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Real,
    Int,
    String, 
    //List,
    Bool,
    Func,
}

#[derive(Clone, Debug, PartialEq)]
pub enum DataType {
    Real(f32),
    Int(i32),
    String(String),
    Bool(bool),
    Func()
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Op(Operator),
    Type(Type),
    Id(String),
    Key(Keyword),
    Data(DataType),
}

fn is_keyword(s: &str) -> bool {
    to_keyword(s).is_some()
}

fn to_keyword(s: &str) -> Option<Keyword> {
    match s {
       "if" => Some(Keyword::If),
       "else" => Some(Keyword::Else),
       "loop" => Some(Keyword::Loop),
       "ret" => Some(Keyword::Return),
       "import" => Some(Keyword::Import),
       "global" => Some(Keyword::Global),
       _ => None 
    }
}

fn is_operator(c: char) -> bool {
    to_operator(c).is_some()
}

fn to_operator(c: char) -> Option<Operator> {
    match c {
       '+' => Some(Operator::Add),
       '-' => Some(Operator::Sub),
       '*' => Some(Operator::Mul),
       '/' => Some(Operator::Div),
       '=' => Some(Operator::Eq),
       '{' => Some(Operator::ScOpen),
       '}' => Some(Operator::ScClose),
       '(' => Some(Operator::BrOpen),
       ')' => Some(Operator::BrClose),
       '\"' => Some(Operator::Quote),
       ';' => Some(Operator::EOL),
       ',' => Some(Operator::Comma),
       '.' => Some(Operator::Dot),
       _ => None 
    }
}

fn is_type(s: &str) -> bool {
    to_type(s).is_some()
}

fn to_type(s: &str) -> Option<Type> {
    match s {
       "real" => Some(Type::Real),
       "int" => Some(Type::Int),
       "string" => Some(Type::String),
       "bool" => Some(Type::Bool),
       "func" => Some(Type::Func),
       _ => None 
    }
}

pub fn process(source_code: &String) -> Vec<Token> {
    let splits = source_code.split_inclusive(|c| c == ' ' || is_operator(c)).filter(|s| s.len() > 0);
    let mut words = Vec::with_capacity(1024);

    for split in splits {
        if split.len() > 1 { 
            let (a, b) = split.split_at(split.len()-1);

            words.push(a.to_string());
            words.push(b.to_string());
        } else {
            words.push(split.to_string());
        }
    }

    let mut tokens = Vec::with_capacity(1024);

    let mut is_quoting = false;

    for word in words.clone() {
        if !is_quoting {
            let word = word.trim().to_string();
            
            if word.len() < 1 {
                continue;
            }

            if let Some(op) = to_operator(word.as_bytes()[0] as char) {
                if op == Operator::Quote {
                    is_quoting = true;
                }
    
                tokens.push(Token::Op(op));
            } else if let Some(tp) = to_type(word.as_str()) {
                tokens.push(Token::Type(tp));
            } else if let Some(key) = to_keyword(word.as_str()) {
                tokens.push(Token::Key(key));
            } else if let Ok(val) = word.parse::<i32>() {
                tokens.push(Token::Data(DataType::Int(val)));
            }else if let Ok(val) = word.parse::<f32>() {
                tokens.push(Token::Data(DataType::Real(val)));
            } else if let Ok(val) = word.parse::<bool>() {
                tokens.push(Token::Data(DataType::Bool(val)));
            } else {
                tokens.push(Token::Id(word));
            }
        } else {
            if let Some(op) = to_operator(word.as_bytes()[0] as char) {
                if op == Operator::Quote {
                    is_quoting = false;
                    tokens.push(Token::Op(op));
                } else {
                    tokens.push(Token::Data(DataType::String(word)))
                }
            } else {
                tokens.push(Token::Data(DataType::String(word)))
            }
        }
    }

    tokens
}