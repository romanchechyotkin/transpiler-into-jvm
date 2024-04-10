use crate::ast::ast::{Expr, Ident, Literal, Program, Stmt};
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
            Some(Token::Print) => {
                Ok(self.parse_print_statement().ok().unwrap())
            }
            Some(_) => {
                Ok(self.parse_expression_statement().ok().unwrap())   
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

        if !self.expect_peek(Token::Assign) {
            return Err(());
        }
        self.next_token();


        let exp = self.parse_expression().ok().unwrap();

        let stmt = Stmt::VarStmt(
            Ident(var_ident),
            exp,
        );

        while !self.curr_token_is(Token::Semicolon) {
            self.next_token();
        }

        return Ok(stmt);
    }

    fn parse_print_statement(&mut self) -> Result<Stmt, ()> {
        if !self.expect_peek(Token::LParen) {
            return Err(());
        }   
        self.next_token();
        
        dbg!(&self.curr_token);

        let exp = self.parse_expression().ok().unwrap();

        dbg!(&self.curr_token);
        dbg!(&self.curr_token);

        if !self.expect_peek(Token::RParen) {
            return Err(());
        } 

        let stmt = Stmt::PrintStmt(exp);

        while !self.curr_token_is(Token::Semicolon) {
            self.next_token();
        }
        
        return Ok(stmt);
    }

    fn parse_expression_statement(&mut self) -> Result<Stmt, ()> {
        let exp: Expr = self.parse_expression().ok().unwrap();
        
        let stmt = Stmt::ExprStmt(exp); 

        // if self.peek_token_is(Token::Semicolon) {
        //     self.next_token();
        // }

        Ok(stmt)
    }

    fn parse_expression(&mut self) -> Result<Expr, ()> {
        match self.curr_token {
            Some(Token::Ident(_)) => {
                Ok(self.parse_ident())
            }
            Some(Token::Integer(_)) => {
                Ok(self.parse_int())
            }
            _ => Err(()),
        }
    }

    fn parse_ident(&mut self) -> Expr {
        return Expr::IdentExpr(Ident(self.curr_token.as_ref().unwrap().token_literal()));
    }

    fn parse_int(&mut self) -> Expr {
        return Expr::LitExpr(Literal::IntLit(self.curr_token.as_ref().unwrap().token_literal().parse::<i64>().unwrap()));
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