use std::fmt::Display;

pub enum Token {
    Plus,
    Minus,
    Asterisk,
    Slash,
    Question,
    Bang,
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

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Token::Plus => write!(f, "Plus"),
            Token::Minus => write!(f, "Minus"),
            Token::Asterisk => write!(f, "Asterisk"),
            Token::Slash => write!(f, "Slash"),
            Token::Question => write!(f, "Question"),
            Token::Bang => write!(f, "Bang"),
            Token::Assign => write!(f, "Assign"),
            Token::Equal => write!(f, "Equal"),
            Token::NotEqual => write!(f, "Not Equal"),
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
