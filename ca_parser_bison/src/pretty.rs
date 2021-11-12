use std::ops::Range;

use colored::Colorize;

use crate::lexer::Lexer;

// Not pretty printing

pub fn print_error(source: &str, range: Range<usize>, line: usize) {
    let lexer = Lexer::new(source);
    for token in lexer {
        if token.token_type == Lexer::YYEOF {
            break;
        }
        let s = token.token_value.to_string();

        let s2 = match token.token_type {
            Lexer::tIDENTIFIER => s.underline(),
            Lexer::tFN => s.cyan(),
            Lexer::tLPAREN | Lexer::tRPAREN | Lexer::tRBRACK | Lexer::tLBRACK => s.yellow(),
            Lexer::tCOLON => s.bright_purple(),
            _ => s.white(),
        };

        print!("{}{}", token.spaces_before, s2);
    }
    println!();

    for _ in 0..range.start {
        print!(" ")
    }
    for _ in range.start - 1..range.end {
        print!("^")
    }
    let fmted = format!(" Line {}:{:?}", line, range.start - 1..range.end - 1);
    println!("{}", fmted.bright_red());
}
