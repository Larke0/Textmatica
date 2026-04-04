use nom::{
    character::complete::{digit1, space0},
    combinator::{map, map_res},
    sequence::delimited,
    IResult,
};
use std::str::FromStr;
use crate::ast::Expr;



// Helper to eat spaces, tabs and new lines before the parses
fn ws<'a, F, O, E: nom::error::ParseError<&'a str>>(
    inner: F,
) -> impl Parser<&'a str, O, E>
where
    F: Parser<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}


// Parse a string into a f64
fn parse_f64(input: &str) -> IResult<&str, f64> {
    map_res(digit1, f64::from_str)(input)
}

// Wrap that f64 in the AST Enum
pub fn parse_number_expr(input: &str) -> IResult<&str, Expr> {
    // `map` takes the successful result of `parse_f64` and runs a closure on it
    map(parse_f64, |num| Expr::Number(num))(input)
}



