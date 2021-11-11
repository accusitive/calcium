// use lexer::YYLexer;
use parser::Parser;

mod lexer;
mod loc;
mod parser;
mod token;
mod value;

fn main() {
    let source = "2+2+9*5*5*5";
    let lexer = Lexer::new(source);
    let parser = Parser::new(lexer, "Cheese");
    let result = parser.do_parse();
    println!("Result: {:#?}", result);
    // let r = parser.
}