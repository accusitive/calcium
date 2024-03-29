use std::ops::Range;

use colored::Colorize;

use crate::lexer::Lexer;

// Not pretty printing

pub fn print_error(source: &str, range: Range<usize>, line: usize, message: Option<&str>) {
    let lexer = Lexer::new(source);
    let mut buf = String::new();
    for token in lexer {
        if token.token_type == Lexer::YYEOF {
            break;
        }
        let s = token.token_value.to_string();

        let s2 = match token.token_type {
            Lexer::tIDENTIFIER => s.blue(),
            Lexer::tFN => s.cyan(),
            Lexer::tLPAREN | Lexer::tRPAREN | Lexer::tRBRACK | Lexer::tLBRACK => s.yellow(),
            Lexer::tCOLON => s.bright_purple(),
            Lexer::kwRETURN => s.bright_yellow(),
            // Lexer::tSTRING => format!("\"{}\"", s).blue(),
            _ => s.white(),
        };
        // let line = buf.matches('\n').collect::<Vec<_>>().len();
        buf.push_str(&format!(
            "{}{}",
            token.spaces_before,
            s2.to_string().replace("\n", "\\n").replace("\t", "\\t")
        ));
    }

    for (line_str, line_no) in buf.split('\n').zip(0..) {
        // Only show 4 lines around the error
        if line_no < line - 4 || line_no > line + 4 {
            continue;
        }
        println!("{:>2}  {}", line_no + 1, line_str);
        if line_no == line - 1 {
            for _ in 0..range.start + 4 {
                print!(" ")
            }
            for _ in range.start..=range.end {
                print!("^")
            }
            let fmted = format!(
                " Line {}:{:?}: {}",
                line,
                range.start.saturating_sub(1)..range.end.saturating_sub(1),
                message.unwrap_or_default()
            );
            println!("{}", fmted.bright_red());
        }
    }
    // println!("{}", buf);
}
