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

        println!("{}", self.curr_token.take().unwrap());
        println!("{}", self.peek_token.take().unwrap());
        
        while let Ok(tok) = self.lexer.next_token() {
            println!("{}", tok);
            if let Token::Eof = tok {
                break;
            }

            match self.parse_statement() {
                Ok(stmt) => program.statements.push(&stmt), 
                Err(_) => panic!(),              
            }

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

        let mut stmt = Stmt::VarStmt(
            Ident(self.curr_token.as_ref().unwrap().token_literal()),
            Expr::IdentExpr(Ident(self.curr_token.as_ref().unwrap().token_literal())),
        );

        if !self.expect_peek(Token::Assign) {
            return Err(());
        }

        // TODO: parse expression
        stmt;

        while !self.curr_token_is(Token::Semicolon) {
            self.next_token();
        }

        return Ok(stmt);
    }

    fn expect_peek(&self, tok: Token) -> bool {
        if self.peek_token_is(tok) {
            self.next_token();
            return true;
        }

        return false;
    }

    fn curr_token_is(&self, tok: Token) -> bool {
        match self.curr_token {
            Some(t) => {
                t == tok
            },
            None => false,
        }
    }

    fn peek_token_is(&self, tok: Token) -> bool {
        match self.peek_token {
            Some(t) => {
                t == tok
            },
            None => false,
        }
    }

}