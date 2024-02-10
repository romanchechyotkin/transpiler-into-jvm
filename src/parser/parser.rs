use crate::lexer::lexer::Lexer;
use crate::tokens::tokens::Token;

pub struct Parser {
    lexer: Lexer,
    
    curr_token: Option<Token>,
    peek_token: Option<Token>,
}

impl Parser {

    pub fn new(input: String) -> Self {
        let lexer = Lexer::new(input); 
        let mut p = Parser { 
            lexer: lexer,
            curr_token: None,
            peek_token: None
        };

        p.next_token();
        p.next_token();

        return p; 
    }

    pub fn parse_program(&mut self) {
        while let Ok(tok) = self.lexer.next_token() {
            println!("{}", tok);
            if let Token::Eof = tok {
                break;
            }
        }
    }

    fn next_token(&mut self) {
        self.curr_token = self.peek_token.take();

        match self.lexer.next_token() {
            Ok(tok) => self.peek_token = Some(tok),
            Err(_) => panic!(),
        };

    }

}