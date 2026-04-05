use crate::ast::Expr;

pub fn evaluate(expr: &Expr) -> f64 {
    match expr {
        // Base case, if its just a number, return a number
        Expr::Number(n) => *n,
        
        //Recursive cases
        Expr::Add(left, right) => evaluate(left) + evaluate(right),
        Expr::Subtract(left, right) => evaluate(left) - evaluate(right),
        Expr::Multiply(left, right) => evaluate(left) * evaluate(right),
        Expr::Divide(left,right) => evaluate(left) / evaluate(right),
        Expr::Fraction(left,right) => evaluate(left) / evaluate(right),
    }
}
