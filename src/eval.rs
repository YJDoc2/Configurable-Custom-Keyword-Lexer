use crate::ast::*;
use std::collections::HashMap;

pub fn eval(start: AST) {
    let mut symtab = HashMap::new();
    for stmt in start.top_block.stmts {
        eval_stmt(stmt, &mut symtab);
    }
}

fn eval_stmt(s: Stmt, symtab: &mut HashMap<String, IRV>) {
    match s {
        Stmt::Print(p) => eval_print(p, symtab),
        Stmt::Decl(d) => {
            let name = d.name;
            let v = eval_expr(d.value, symtab);
            symtab.insert(name, v);
        }
    }
}

fn eval_print(p: PrintStmt, symtab: &mut HashMap<String, IRV>) {
    let v = eval_expr(p.arg, symtab);
    println!("{}", v);
}

fn eval_expr(e: Expr, symtab: &mut HashMap<String, IRV>) -> IRV {
    match e {
        Expr::Val(v) => get_irv(v, symtab),
        Expr::Op(e1, op, e2) => {
            let e1v = eval_expr(*e1, symtab);
            let e2v = eval_expr(*e2, symtab);
            match op {
                Operator::Add => add_vals(e1v, e2v),
                Operator::Sub => match (e1v, e2v) {
                    (IRV::Num(n1), IRV::Num(n2)) => IRV::Num(n1 - n2),
                    _ => panic!("Can only subtract two numbers"),
                },
                Operator::Mul => match (e1v, e2v) {
                    (IRV::Num(n1), IRV::Num(n2)) => IRV::Num(n1 * n2),
                    _ => panic!("Can only  multiply two numbers"),
                },
                Operator::Div => match (e1v, e2v) {
                    (IRV::Num(n1), IRV::Num(n2)) => IRV::Num(n1 / n2),
                    _ => panic!("Can only divide two numbers"),
                },
            }
        }
    }
}

fn add_vals(v1: IRV, v2: IRV) -> IRV {
    match v1 {
        IRV::Num(n1) => match v2 {
            IRV::Num(n2) => return IRV::Num(n1 + n2),
            IRV::Str(s2) => return IRV::Str(format!("{}{}", n1, s2)),
        },
        IRV::Str(s) => IRV::Str(format!("{}{}", s, v2)),
    }
}

fn get_irv(v: Value, symtab: &mut HashMap<String, IRV>) -> IRV {
    match v {
        Value::Nval(n) => IRV::Num(n),
        Value::Sval(s) => IRV::Str(s),
        Value::Id(id) => match symtab.get(&id) {
            Some(v) => v.clone(),
            None => panic!("Variable `{}` used before declared", id),
        },
    }
}

#[derive(Clone, Debug)]
enum IRV {
    Num(f32),
    Str(String),
}

impl std::fmt::Display for IRV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IRV::Num(n) => write!(f, "{}", n),
            IRV::Str(s) => write!(f, "{}", s),
        }
    }
}
