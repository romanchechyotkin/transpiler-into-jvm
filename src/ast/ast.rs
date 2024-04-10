use std::fmt::Display;

pub type Program = Vec<Stmt>;

#[derive(PartialEq, Debug, Clone)]
pub struct Ident(pub String);

impl Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Expr {
	IdentExpr(Ident),
    LitExpr(Literal),
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::IdentExpr(ident) => write!(f, "{}", ident),
            Expr::LitExpr(lit) => write!(f, "{}", lit),
        }        
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Stmt {
    VarStmt(Ident, Expr),
    PrintStmt(Expr),
    ExprStmt(Expr)
}

#[derive(PartialEq, Debug, Clone)]
pub enum Literal {
    IntLit(i64),     
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::IntLit(num) => write!(f, "{}", num),               
        }
    }
}