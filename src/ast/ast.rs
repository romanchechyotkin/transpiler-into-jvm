pub type Program = Vec<Stmt>;

#[derive(PartialEq, Debug, Clone)]
pub struct Ident(pub String);

#[derive(PartialEq, Debug, Clone)]
pub enum Expr {
	IdentExpr(Ident),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Stmt {
    VarStmt(Ident, Expr),
}
