use Textmatica::parser::parse_math_expr;
use Textmatica::evaluator::evaluate;

fn main() {
    // The Final Boss Equation: 
    // -10.5 * 2 + \frac{-100}{5} - (-3.14)
    // Mathematically: (-21) + (-20) - (-3.14) = -37.86

    let math_string = "+10.5 * 2 + \\frac{-100}{5} - (-3.14)";
    let correct_result = 10.5*2.0 + (-100.0/5.0) - (-3.14);

    println!("Parsing: {}", math_string);
    
    match parse_math_expr(math_string) {
        Ok((leftover, ast)) => {
            // Using a clean print so it doesn't take up 5 pages in your terminal!
            println!("\nParsed AST successfully!"); 
            
            let result = evaluate(&ast);
            println!("FINAL CALCULATED RESULT: {}, should have been: {}", result, correct_result); 
            
            if leftover.is_empty() {
                println!("No leftovers! Perfect parse.");
            } else {
                println!("Leftover string: '{}'", leftover);
            }
        }
        Err(e) => println!("Error: {:?}", e),
    }
}
