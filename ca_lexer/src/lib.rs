use std::{result::IntoIter, str::CharIndices};
mod loc;

#[test]
fn main() {
    let lex = YYLexer::new("fn f() 2");
    for tok in lex {
        println!("Tok {:#?}", tok);
    }
}
#[derive(Debug, Clone)]
pub struct YYLexer {
    chars: std::vec::IntoIter<(usize, char)>,
    line: usize,
    col: usize,
}
impl YYLexer {
    pub fn new(input: &str) -> Self {
        // self.chars.into_iter()
        YYLexer {
            chars: input.char_indices().collect::<Vec<_>>().into_iter(),
            col: 0,
            line: 0,
        }
    }
}

impl Iterator for YYLexer {
    type Item = Spanned;
    // type Item = Token<'i>;

    fn next(&mut self) -> Option<Self::Item> {
        macro_rules! matches {
            ($e: literal) => {
                self.chars.next().map(|c| c.1) == Some($e)
            };
        }
        macro_rules! spanned {
            ($token: expr, $len: expr) => {
                Spanned::new($token, Span::new(self.col, $len, self.line))
            };
        }
        
        loop {
            match self.chars.next() {
                Some((i, 'f')) => {
                    if matches!('n') {
                        return Some(spanned!(Token::Fn, 2));
                    } else {
                        return Some(spanned!(Token::Char('f'), 1));
                    }
                }
                Some((i, '(')) => return Some(spanned!(Token::LParen, 1)),
                Some((i, ')')) => return Some(spanned!(Token::RParen, 1)),

                Some((i, c @ '0'..='9')) => {
                    return Some(spanned!(Token::Number(c.to_digit(10).unwrap()), 1))
                }

                Some((i, c)) => return Some(spanned!(Token::Char(c), 1)),
                Some((i, '\n')) => {
                    self.col = 0;
                    self.line = 0;
                }
                None => return None,
                _ => continue,
            }
            self.col += 1;
        }
    }
}
#[derive(Debug, Clone)]
pub struct Spanned {
    pub token: Token,
    pub token_type: i32,
    pub loc: Span,
}
impl Spanned {
    fn new(token: Token, span: Span) -> Self {
        Self { loc: span, token, token_type: 0}
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    LParen,
    RParen,
    Char(char),
    Number(u32),
    Fn,
}
#[derive(Debug, PartialEq, Clone)]
pub struct Span {
    pub col_start: usize,
    pub col_end: usize,
    pub line: usize
}

impl Span {
    fn new(start: usize, len: usize, line: usize) -> Self {
        Span {
            col_start: start,
            col_end: start + len,
            line
        }
    }
}
