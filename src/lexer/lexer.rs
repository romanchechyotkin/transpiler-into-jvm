use crate::tokens::tokens::Token;

pub struct Lexer {
    input: Vec<u8>,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lex = Lexer {
            input: input.into_bytes(),
            position: 0,
            read_position: 0,
            ch: 0,
        };

        lex.read_char();

        return lex;
    }

    pub fn next_token(&mut self) -> Result<Token, ()> {
        self.skip_whitespace();

        let tok: Token = match self.ch {
            b'+' => Token::Plus,
            b'-' => Token::Minus,
            b'*' => Token::Asterisk,
            b'/' => Token::Slash,

            b'?' => Token::Question,
            b'!' => Token::Bang,
            b'=' => Token::Assign,
            0 => Token::Eof,
            _ => todo!(),
        };

        self.read_char();

        Ok(tok)
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.read_position];
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }
}
