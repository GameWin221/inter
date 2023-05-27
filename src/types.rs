#[derive(Clone, Debug, PartialEq)]
pub enum Keyword {
    If,
    Else,
    While,
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

    Ref,
    Hash,

    Less,
    Greater,

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

pub fn is_keyword(s: &str) -> bool {
    to_keyword(s).is_some()
}

pub fn to_keyword(s: &str) -> Option<Keyword> {
    match s {
       "if" => Some(Keyword::If),
       "else" => Some(Keyword::Else),
       "while" => Some(Keyword::While),
       "ret" => Some(Keyword::Return),
       "import" => Some(Keyword::Import),
       "global" => Some(Keyword::Global),
       _ => None 
    }
}

pub fn is_operator(c: char) -> bool {
    to_operator(c).is_some()
}

pub fn to_operator(c: char) -> Option<Operator> {
    match c {
       '+' => Some(Operator::Add),
       '-' => Some(Operator::Sub),
       '*' => Some(Operator::Mul),
       '/' => Some(Operator::Div),
       '=' => Some(Operator::Eq),
       '&' => Some(Operator::Ref),
       '#' => Some(Operator::Hash),
       '<' => Some(Operator::Less),
       '>' => Some(Operator::Greater),
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

pub fn is_type(s: &str) -> bool {
    to_type(s).is_some()
}

pub fn to_type(s: &str) -> Option<Type> {
    match s {
       "real" => Some(Type::Real),
       "int" => Some(Type::Int),
       "string" => Some(Type::String),
       "bool" => Some(Type::Bool),
       "func" => Some(Type::Func),
       _ => None 
    }
}

pub fn default_type_value(t: &Type) -> DataType {
    match t {
        Type::Real => DataType::Real(0.0),
        Type::Int => DataType::Int(0),
        Type::String => DataType::String(String::from("")), 
        Type::Bool => DataType::Bool(false),
        Type::Func => DataType::Func(),
    }
}

pub fn translate_data(data: &DataType) -> String {
    match data {
        DataType::Real(val) => val.to_string(),
        DataType::Int(val) => val.to_string(),
        DataType::String(val) => val.to_owned(), 
        DataType::Bool(val) => val.to_string(),
        DataType::Func() => String::from(" "),
    }
}

pub fn translate_type(typename: &Type) -> String {
    match typename {
        Type::Real => String::from("float "),
        Type::Int => String::from("int "),
        Type::String => String::from("std::string "), 
        Type::Bool => String::from("bool "),
        Type::Func => String::from("void "),
    }
}

pub fn translate_key(key: &Keyword) -> String {
    match key {
        Keyword::Else => String::from("else "),
        Keyword::If => String::from("if "),
        Keyword::Return => String::from("return "),
        Keyword::While => String::from("while "),
        Keyword::Import => String::from(""),
        Keyword::Global => String::from(""),
    }
}

pub fn translate_operator(op: &Operator) -> String {
    match op {
        Operator::Add => String::from("+"),
        Operator::Sub => String::from("-"),
        Operator::Mul => String::from("*"),
        Operator::Div => String::from("/"),
        Operator::Eq => String::from("="),

        Operator::Ref => String::from("&"),
        Operator::Hash => String::from("#"),

        Operator::Less => String::from("<"),
        Operator::Greater => String::from(">"),

        Operator::ScOpen => String::from("{\n"),
        Operator::ScClose => String::from("}\n\n"),

        Operator::BrOpen => String::from("("),
        Operator::BrClose => String::from(")"),

        Operator::Quote => String::from("\""),
        Operator::Comma => String::from(","),
        Operator::Dot => String::from("."),

        Operator::EOL => String::from(";\n"),
    }
}

pub fn translate_token(token: &Token) -> String {
    match token {
        Token::Data(val) => translate_data(val),
        Token::Id(val) => val.to_owned(),
        Token::Key(val) => translate_key(val),
        Token::Op(val) => translate_operator(val),
        Token::Type(val) => translate_type(val),
    }
}
