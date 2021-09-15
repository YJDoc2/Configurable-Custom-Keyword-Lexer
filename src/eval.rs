use crate::ast::*;
use std::collections::HashMap;

#[derive(Default)]
struct Scope {
    scopes: Vec<HashMap<String, IRV>>,
}

impl Scope {
    pub fn push(&mut self) {
        self.scopes.push(HashMap::new());
    }
    pub fn pop(&mut self) {
        self.scopes.pop();
    }
    pub fn set(&mut self, name: String, v: IRV) {
        let last = self.scopes.len() - 1;
        self.scopes[last].insert(name, v);
    }
    pub fn resolve(&self, var: &str) -> Option<IRV> {
        for s in self.scopes.iter().rev() {
            if let Some(v) = s.get(var) {
                return Some(v.clone());
            }
        }
        None
    }
}

pub fn eval(start: AST) {
    let mut symtab = Scope::default();
    symtab.push();
    eval_block(start.top_block, &mut symtab);
}

fn eval_block(blk: Block, symtab: &mut Scope) {
    for stmt in blk.stmts {
        eval_stmt(stmt, symtab);
    }
}

fn eval_stmt(s: Stmt, symtab: &mut Scope) {
    match s {
        Stmt::Print(p) => eval_print(p, symtab),
        Stmt::Decl(d) => {
            let name = d.name;
            let v = eval_expr(d.value, symtab);
            symtab.set(name, v);
        }
        Stmt::If(i) => eval_if(i, symtab),
    }
}

fn eval_if(i: IfStmt, symtab: &mut Scope) {
    let cond = eval_cond(i.cond, symtab);
    if cond {
        eval_block(i.if_blk, symtab);
    } else {
        eval_block(i.else_blk, symtab);
    }
}

fn eval_cond(c: Condition, symtab: &mut Scope) -> bool {
    match c {
        Condition::Comparison(e1, op, e2) => {
            let v1 = eval_expr(e1, symtab);
            let v2 = eval_expr(e2, symtab);
            match (v1, v2) {
                (IRV::Num(n1), IRV::Num(n2)) => match op {
                    ComparisonOp::Eq => n1 == n2,
                    ComparisonOp::GT => n1 > n2,
                    ComparisonOp::GTE => n1 >= n2,
                    ComparisonOp::LT => n1 < n2,
                    ComparisonOp::LTE => n1 <= n2,
                    ComparisonOp::NotEq => n1 != n2,
                },
                (IRV::Str(s1), IRV::Str(s2)) => match op {
                    ComparisonOp::Eq => return s1 == s2,
                    ComparisonOp::GT => s1 > s2,
                    ComparisonOp::GTE => s1 >= s2,
                    ComparisonOp::LT => s1 < s2,
                    ComparisonOp::LTE => s1 <= s2,
                    ComparisonOp::NotEq => s1 != s2,
                },
                _ => panic!("Cannot compare between string and number"),
            }
        }
        Condition::Not(c) => return !eval_cond(*c, symtab),
        Condition::LogicalOp(c1, op, c2) => {
            let c1 = eval_cond(*c1, symtab);
            match op {
                LogicalOp::And => {
                    if !c1 {
                        return false;
                    } else {
                        return eval_cond(*c2, symtab);
                    }
                }
                LogicalOp::Or => {
                    if c1 {
                        return true;
                    } else {
                        return eval_cond(*c2, symtab);
                    }
                }
            }
        }
    }
}

fn eval_print(p: PrintStmt, symtab: &mut Scope) {
    let v = eval_expr(p.arg, symtab);
    println!("{}", v);
}

fn eval_expr(e: Expr, symtab: &mut Scope) -> IRV {
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

fn get_irv(v: Value, symtab: &mut Scope) -> IRV {
    match v {
        Value::Nval(n) => IRV::Num(n),
        Value::Sval(s) => IRV::Str(s),
        Value::Id(id) => match symtab.resolve(&id) {
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
