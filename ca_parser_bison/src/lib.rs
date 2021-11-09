use lexer::Lexer;
use parser::Parser;

pub mod parser;
// include!()
pub mod lexer;
pub mod loc;
pub mod token;
pub mod value;

fn z() {
    let source = "2+2+9*5*5*5";
    let lexer = Lexer::new(source);
    let parser = Parser::new(lexer, "Cheese");
    let result = parser.do_parse();  
    println!("Result: {:#?}", result);
    // let r = parser.

}
