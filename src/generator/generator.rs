use std::collections::HashMap;
use crate::ast::ast::{Expr, Literal, Stmt, Ident};

#[derive(PartialEq, Debug, Clone)]
pub enum JVMInstruction {
    IConst(i64),
    IStore(i64),
    ILoad(i64),
    LPrint(i64),
    IPrint(i64),
}

impl JVMInstruction {

    pub fn format(&self) -> String {
        match &self {
            JVMInstruction::IConst(val) => format!("IConst_{}", val),   
            JVMInstruction::IStore(val) => format!("IStore_{}", val),   
            JVMInstruction::ILoad(val) => format!("ILoad_{}", val),   
            JVMInstruction::LPrint(val) => format!("LPrint_{}", val),   
            JVMInstruction::IPrint(val) => format!("IPrint_{}", val),   
        }  
    }
}

pub struct Generator {
    var_map: HashMap<String, i64>,
    counter: i64
}

impl Generator {
    pub fn new() -> Self {
        Generator{var_map: HashMap::new(), counter: 0}
    }

    pub fn generate_code(&mut self, stmt: &Stmt) -> Vec<JVMInstruction> {
        match stmt {
            Stmt::VarStmt(var_name, var_value) => {
                let mut instructions = Vec::new();
                match var_value {
                    Expr::LitExpr(Literal::IntLit(num)) => {
                        self.counter += 1;
                        self.var_map.insert(var_name.0.clone(), self.counter);

                        instructions.push(JVMInstruction::IConst(*num));
                        instructions.push(JVMInstruction::IStore(self.var_map.get(&var_name.0).unwrap().clone()));
                        instructions
                    }
                    Expr::IdentExpr(ident) => {
                        // var a = 5; => map[a] = 1
                        // var b = a; => map[b] = 1
                        // get value from value above
                        // store value in new jvm local variable 
                        // store the value in map and stack 
                        instructions.push(JVMInstruction::ILoad(self.var_map.get(&ident.0).unwrap().clone()));
                        
                        self.counter += 1;
                        self.var_map.insert(var_name.0.clone(), self.counter);

                        instructions.push(JVMInstruction::IStore(self.var_map.get(&var_name.0).unwrap().clone()));
                        instructions   
                    }
                }
            }
            Stmt::PrintStmt(exp) => {
                let mut instructions = Vec::new();
                match exp {
                    Expr::IdentExpr(ident) => {
                        let num = self.var_map.get(&ident.0).unwrap();

                        instructions.push(JVMInstruction::IPrint(*num));
                        instructions
                    },
                    Expr::LitExpr(Literal::IntLit(num)) => {

                        instructions.push(JVMInstruction::LPrint(*num));
                        instructions
                        
                    },
                }
            }
            _ => todo!(),
        }
    }
}