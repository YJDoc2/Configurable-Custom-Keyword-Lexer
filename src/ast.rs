#[derive(Debug)]
pub struct AST {
    pub top_block: Block,
}

#[derive(Debug)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}

#[derive(Debug)]
pub enum Stmt {
    Print(PrintStmt),
    Decl(Decl),
    If(IfStmt),
    Assign(Assign),
}

#[derive(Debug)]
pub struct Assign {
    pub id: String,
    pub value: Expr,
}

#[derive(Debug)]
pub struct Decl {
    pub name: String,
    pub value: Expr,
}

#[derive(Debug)]
pub struct PrintStmt {
    pub arg: Expr,
}

#[derive(Debug)]
pub enum Expr {
    Val(Value),
    Op(Box<Expr>, Operator, Box<Expr>),
}

#[derive(Debug)]
pub struct IfStmt {
    pub cond: Condition,
    pub if_blk: Block,
    pub else_blk: Block,
}

#[derive(Debug)]
pub enum Condition {
    Comparison(Expr, ComparisonOp, Expr),
    Not(Box<Condition>),
    LogicalOp(Box<Condition>, LogicalOp, Box<Condition>),
}

#[derive(Debug)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum LogicalOp {
    And,
    Or,
}

#[derive(Debug)]
pub enum ComparisonOp {
    LT,
    LTE,
    GT,
    GTE,
    NotEq,
    Eq,
}

#[derive(Debug)]
pub enum Value {
    Sval(String),
    Nval(f32),
    Id(String),
}
