use std::fmt::Display;

pub enum Token {
    Plus,
    Minus,
    Asterisk,
    Slash,
    Question,
    Bang,
    Assign,
    Eof,
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
            Token::Eof => write!(f, "Eof"),
        };
    }
}
