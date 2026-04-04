use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1},
    sequence::delimited,
    IResult,
    Parser,
    combinator::{map_res, map},
};


use std::str::FromStr;

fn parse_i32(input: &str) -> IResult<&str, i32> {

    map_res(digit1, i32::from_str).parse(input)

} 

fn parse_braced_number(input: &str) -> IResult<&str, i32> {
    delimited(
        char('{'),
        parse_i32,
        char('}'),
    ).parse(input)
}


#[derive(Debug, PartialEq)]
enum Math {
    SquareRoot(i32)
}

fn parse_sqrt(input: &str) -> IResult<&str, Math> {
    let sqrt_parsers = (tag("\\sqrt"), parse_braced_number);

    map(
        sqrt_parsers,
        |(_, number)| Math::SquareRoot(number)
    ).parse(input)
}

fn main() {
    let math_string = "\\sqrt{144} = 12";

    match parse_sqrt(math_string) {
        Ok((leftover, parsed_ast)) => {
            println!("Success!!");
            // This should print: Math::SquareRoot(144)
            println!("Parsed AST: '{:?}'", parsed_ast); 
            println!("Leftover: '{}'", leftover);
        }
        Err(e) => println!("Error: {:?}", e),
    }
}
