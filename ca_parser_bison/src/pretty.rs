use std::ops::Range;

use colored::{ColoredString, Colorize};

use crate::{lexer::Lexer, parser::token_name};

// Not pretty printing

pub fn highlight(source: &str, range: Range<usize>) {
    let lexer = Lexer::new(source);
    let mut src = String::from(source);
    for token in lexer {
        if token.token_type == Lexer::YYEOF {
            break;
        }
        let mut  s = token.token_value.to_string();
    //    src.replace_range(range.start..=range.end, &"asd".und().to_string());
        println!("Token loc {:#?} {:#?}", token.loc, range);
        if  true {
            // s = s.on_red().to_string();
            let val = format!("{}{}", token.spaces_before, token.token_value);
            src.replace_range(token.loc.to_range(), "val");
        }
        // s = s.on_red().to_string();
        let s2 = match token.token_type {
            Lexer::tIDENTIFIER => s.underline(),
            Lexer::tFN => s.cyan(),
            Lexer::tLPAREN | Lexer::tRPAREN | Lexer::tRBRACK | Lexer::tLBRACK => s.yellow(),
            Lexer::tCOLON => s.bright_purple(),
            _ => s.white(),
        };
        // TODO: Get proper highlithing working, need to get bounds working on 19 but the arrow should work if you comment 19:23 out.
        
        
        // s2.on_red()

        // print!("{}{}", token.spaces_before, s2);
        
        // println!("{} {}", token_name(token.token_type), s);
    }
    println!("Source {}", &source.get(range.start..=range.end).unwrap());
    println!("{}", src);
    // println!();
    for i in 0..range.start {
        print!(" ")
    }
    for i in range.start..range.end {
        print!("^")
    }
    print!("^\n")
}