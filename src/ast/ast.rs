pub type Program = Vec<Stmt>;

pub struct Ident(pub String);

pub enum Expr {
	IdentExpr(Ident),
}

pub enum Stmt {
    VarStmt(Ident, Expr),
}
