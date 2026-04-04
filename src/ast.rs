#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Number(f64),
    Add(Box<Expr>, Box<Expr>),
    Subtract(Box<Expr>, Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
    Divide(Box<Expr>, Box<Expr>),
    Fraction(Box<Expr>, Box<Expr>), //{top}/{bottom}
}
