use crate::tokens::tokens::Token;

// pub trait Node {
//   	fn token_literal(&self) -> String;
//   	fn string(&self) -> String;
// } 

pub type Program = Vec<Stmt>;


// impl Node for Program<'_> {
//     fn token_literal(&self) -> String {
// 		if self.statements.len() > 0 {
//         	return self.statements[0].token_literal();
//       	} 

//       	return "".to_string();
//     }

// 	fn string(&self) -> String {
// 		return "".to_string();
//     }
// }


pub struct Ident(pub String);

pub enum Expr {
	IdentExpr(Ident),
}

pub enum Stmt {
    VarStmt(Ident, Expr),
}

// pub struct VarStatement<'a> {
// 	pub token: Token, // var token
//  	pub ident: Option<&'a dyn Node>, // ident of var; ident node below for this
// 	pub value: Option<&'a dyn Node>, // variable value
// }

// impl Node for VarStatement<'_> {
// 	fn token_literal(&self) -> String {
// 		return self.token.token_literal();
// 	}

// 	fn string(&self) -> String {
// 		return "".to_string();
// 	}
// }

// pub struct Identifier<'a> {
// 	pub token: &'a Token, // ident token
// 	pub value: &'a String, // ident value
// }

// impl Node for Identifier<'_> {
// 	fn token_literal(&self) -> String {
// 		return self.token.token_literal();
// 	}

// 	fn string(&self) -> String {
// 		return self.value.clone();
// 	}
// }

