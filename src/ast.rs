#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    Assignment { name: String, expr: Expr },
    Print { expr: Expr },
}

#[derive(Debug)]
pub enum Expr {
    Number(i64),
    Var(String),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
}
