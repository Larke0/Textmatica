use nom::{
    branch::alt,
    character::complete::{digit1, space0, multispace0, char},
    combinator::{map, map_res, opt, recognize},
    IResult,
    Parser,
    bytes::complete::tag,
    multi::many0,
    sequence::{delimited, tuple},
};
use std::str::FromStr;
use crate::ast::Expr;



// Helper to eat spaces, tabs and new lines before the parses
fn ws<'a, F, O, E: nom::error::ParseError<&'a str>>(
    inner: F,
) -> impl Parser<&'a str, Output = O, Error = E>
where
    F: Parser<&'a str, Output = O, Error = E>,
{
    delimited(multispace0, inner, multispace0)
}


// Parse a string into a f64
fn parse_f64(input: &str) -> IResult<&str, f64> {
    let float_string_parser = recognize (
        (
            opt(alt((char('-'), char('+')))), // Check if there is a minus or a plus sign
            digit1,         // Main digits
            opt((char('.'), digit1)) // Check if there is a dot and more numbers (decimals)
        )
    );
    ws(map_res(float_string_parser, f64::from_str)).parse(input)
}

// Wrap that f64 in the AST Enum
pub fn parse_number_expr(input: &str) -> IResult<&str, Expr> {
    // `map` takes the successful result of `parse_f64` and runs a closure on it
    map(parse_f64, |num| Expr::Number(num)).parse(input)
}


// The Fraction Parser
pub fn parse_fraction(input: &str) -> IResult<&str, Expr> {
    let frac_parsers = (
        ws(tag("\\frac")), 
        delimited(ws(char('{')), parse_math_expr, ws(char('}'))),
        delimited(ws(char('{')), parse_math_expr, ws(char('}')))
    );

    map(frac_parsers, |(_, top_ast, bottom_ast)| {
        Expr::Fraction(Box::new(top_ast), Box::new(bottom_ast))
    }).parse(input)
}


pub fn parse_term(input: &str) -> IResult<&str, Expr> {
    alt((
        parse_fraction,
        parse_parens,
        parse_number_expr
    )).parse(input)
}

// Parse anything inside parentheses:
pub fn parse_parens(input: &str) -> IResult<&str, Expr> {
    delimited(
        ws(char('(')),
        parse_math_expr,
        ws(char(')'))
        ).parse(input)
}

pub fn parse_factor(input: &str) -> IResult<&str, Expr> {
    let (input, mut left_expr) = parse_term(input)?;

    // Now we use many0 to organize the expression into  "packets" of ["*" or "/", Number]:
    let (input, operations) = many0(
        tuple((
            ws(alt((char('*'), char('/')))),
            parse_term
        ))
    ).parse(input)?;

    // Now we build the AST tree
    for (operator, right_expr) in operations {
        if operator == '*' {
            left_expr = Expr::Multiply(Box::new(left_expr), Box::new(right_expr));
        } else if operator == '/' {
            left_expr = Expr::Divide(Box::new(left_expr), Box::new(right_expr));
        }
    }

    // Return the finished tree
    Ok((input, left_expr))
}


// Generic expression parser (also handles addition and subtraction)
pub fn parse_math_expr(input: &str) -> IResult<&str, Expr> {
    // Grab the first number and return the rest of the expression
    let (input, mut left_expr) = parse_factor(input)?;

    // Now we use many0 to organize the expression into "packets" of ["+" or "-", Number]:
    let (input, operations) = many0(
        tuple((
            ws(alt((char('+'), char('-')))),
            parse_factor
        ))
    ).parse(input)?;

    // Now we build the AST tree
    for (operator, right_expr) in operations {
        if operator == '+' {
            left_expr = Expr::Add(Box::new(left_expr), Box::new(right_expr));
        } else if operator == '-' {
            left_expr = Expr::Subtract(Box::new(left_expr), Box::new(right_expr));
        }
    }


    // Return the finished tree
    Ok((input, left_expr))
}


