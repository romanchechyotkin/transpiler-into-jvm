use std::fmt::{Debug, Display};

#[derive(Debug, PartialEq)]
pub enum Token {
    Var,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Question,
    Bang,
    Semicolon,
    NotEqual,
    Equal,
    Assign,
    Eof,
    Ident(String),
    Integer(String),
    If,
    Else,
    Func,
    Return,
    True,
    False,
}

impl Token {
    pub fn token_literal(&self) -> String {
        return match self {
            Token::Var => "Var".to_string(),
            Token::Plus => "Plus".to_string(),
            Token::Minus => "Minus".to_string(),
            Token::Asterisk => "Asterisk".to_string(),
            Token::Slash => "Slash".to_string(),
            Token::Question => "Question".to_string(),
            Token::Bang => "Bang".to_string(),
            Token::NotEqual => "Not Equal".to_string(),
            Token::Equal => "Equal".to_string(),
            Token::Assign => "Assign".to_string(),
            Token::Semicolon => "Semicolon".to_string(),
            Token::Eof => "Eof".to_string(),
            Token::Ident(ident) => ident.to_string(),
            Token::Integer(int) => int.to_string(),
            Token::If => "IF".to_string(),
            Token::Else => "ELSE".to_string(),
            Token::Func => "FUNCTION".to_string(),
            Token::Return => "Return".to_string(),
            Token::True => "True".to_string(),
            Token::False => "False".to_string(),       
        };
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Token::Var => write!(f, "VAR"),
            Token::Plus => write!(f, "Plus"),
            Token::Minus => write!(f, "Minus"),
            Token::Asterisk => write!(f, "Asterisk"),
            Token::Slash => write!(f, "Slash"),
            Token::Question => write!(f, "Question"),
            Token::Bang => write!(f, "Bang"),
            Token::Assign => write!(f, "Assign"),
            Token::Equal => write!(f, "Equal"),
            Token::NotEqual => write!(f, "Not Equal"),
            Token::Semicolon => write!(f, "Semicolon"),
            Token::Ident(ident) => write!(f, "Ident {ident}"),
            Token::Integer(int) => write!(f, "Integer {int}"),
            Token::If => write!(f, "IF"),
            Token::Else => write!(f, "ELSE"),
            Token::Func => write!(f, "FUNCTION"),
            Token::Return => write!(f, "Return"),
            Token::True => write!(f, "TRUE"),
            Token::False => write!(f, "FALSE"),
            Token::Eof => write!(f, "Eof"),
        };
    }
}
