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
            b'!' => {
                if b'=' == self.peek() {
                    self.read_char();
                    Token::NotEqual
                } else {
                    Token::Bang
                }
            },
            b'=' => {
                if b'=' == self.peek() {
                    self.read_char();
                    Token::Equal
                } else {
                    Token::Assign
                }
            },
            b'a'..=b'z' | b'A'..=b'Z' => {
                let ident = match self.read_ident() {
                    Ok(i) => i,
                    Err(()) => panic!("wrong expression")                 
                };

                return Ok(match ident.as_str() {
                    "if" => Token::If,
                    "else" => Token::Else,
                    "fn" => Token::Func,
                    "return" => Token::Return,
                    "fax" => Token::True,
                    "cap" => Token::False,
                    _ => Token::Ident(ident)
                });    
            },
            b'0'..=b'9' => {
                let int: String = match self.read_number() {
                    Ok(i) => i,
                    Err(err) => panic!("{err}"),
                };
                
                return Ok(Token::Integer(int));    
            }, 
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

    fn read_ident(&mut self) -> Result<String, ()> {
        let pos = self.position;

        while self.ch.is_ascii_alphabetic() {
            self.read_char();
        } 

        if self.ch.is_ascii_digit() {
            return Err(());
        } 

        return Ok(String::from_utf8_lossy(&self.input[pos..self.position]).to_string());
    }

    fn read_number(&mut self) -> Result<String, String> {
        let pos = self.position;

        while self.ch.is_ascii_digit() {
            self.read_char();
        } 

        if self.ch.is_ascii_alphabetic() {
            return Err("wrong expression {}".to_string());
        } 

        return Ok(String::from_utf8_lossy(&self.input[pos..self.position]).to_string());
    }

    fn peek(&self) -> u8 {
        if self.read_position >= self.input.len() {
            return 0;
        } else {
            return self.input[self.read_position];
        }       
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }
}
