use crate::ast::ast::{Expr, Ident, Program, Stmt};
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

    pub fn parse_program(&mut self) -> Program {
        let mut program: Program = Vec::new();    
        
        while let Some(tok) = &self.curr_token {
            dbg!(&tok.token_literal());
            if let Token::Eof = tok {
                break;
            }

            match self.parse_statement() {
                Ok(stmt) => program.push(stmt), 
                Err(_) => panic!(),              
            }

            self.next_token();
        }

        return program;
    }

    fn next_token(&mut self) {
        self.curr_token = self.peek_token.take();

        match self.lexer.next_token() {
            Ok(tok) => self.peek_token = Some(tok),
            Err(_) => panic!(),
        };
    }

    fn parse_statement(&mut self) -> Result<Stmt, ()> {
        return match self.curr_token {
            Some(Token::Var) => {
                Ok(self.parse_var_statement().ok().unwrap())
            }
            _ => {
                Err(())
            },
        }
    }

    fn parse_var_statement(&mut self) -> Result<Stmt, ()> {
        if !self.expect_peek(Token::Ident(self.peek_token.as_ref().unwrap().token_literal())) {
            return Err(());
        }   

        let var_ident = self.curr_token.as_ref().unwrap().token_literal();
        dbg!(&var_ident);

        if !self.expect_peek(Token::Assign) {
            return Err(());
        }

        // TODO: parse expression

        let stmt = Stmt::VarStmt(
            Ident(var_ident),
            Expr::IdentExpr(Ident(self.peek_token.as_ref().unwrap().token_literal())),
        );

        dbg!(&stmt);
        dbg!(&self.curr_token);
        dbg!(&self.peek_token);

        while !self.curr_token_is(Token::Semicolon) {
            self.next_token();
        }

        return Ok(stmt);
    }

    fn expect_peek(&mut self, tok: Token) -> bool {
        if self.peek_token_is(tok) {
            self.next_token();
            return true;
        }

        return false;
    }

    fn curr_token_is(&self, tok: Token) -> bool {
        match &self.curr_token {
            Some(t) => {
                t == &tok
            },
            None => false,
        }
    }

    fn peek_token_is(&self, tok: Token) -> bool {
        match &self.peek_token {
            Some(t) => {
                t == &tok
            },
            None => false,
        }
    }

}