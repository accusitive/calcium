use std::{fmt::Debug, result::IntoIter, str::CharIndices};

use crate::{loc::Loc, parser};
use prev_iter::PrevPeekable;

#[test]
fn main() {
    let lex = Lexer::new("fn f() 2");
    for tok in lex {
        println!("Tok {:#?}", tok);
    }
}
#[derive(Debug)]
pub struct Lexer {
    chars: PrevPeekable<std::vec::IntoIter<(usize, char)>>,
    line: usize,
    col: usize,
}
impl Lexer {
    pub fn new(input: &str) -> Self {
        // self.chars.into_iter()
        Lexer {
            chars: PrevPeekable::new(input.char_indices().collect::<Vec<_>>().into_iter()),
            col: 0,
            line: 0,
        }
    }
    pub fn yylex(&mut self) -> Token {
        self.next().unwrap()
    }
}

impl Iterator for Lexer {
    type Item = Token;
    // type Item = Token<'i>;

    fn next(&mut self) -> Option<Self::Item> {
        macro_rules! matches {
            ($e: literal) => {
                self.chars.peek().map(|c| c.1) == Some($e)
            };
        }
        // macro_rules! spanned {
        //     ($token: expr, $len: expr) => {
        //         Spanned::new($token, Span::new(self.col, $len, self.line))
        //     };
        // }

        loop {
            match self.chars.next() {
                Some((i, 'f')) => {
                    if matches!('n') {
                        return Some(Token {
                            token_type: Self::tFN,
                            loc: Loc {
                                begin: i,
                                end: i + 2,
                            },
                        });
                    } else {
                        return Some(Token {
                            token_type: Self::tRPAREN,
                            loc: Loc {
                                begin: i,
                                end: i + 2,
                            },
                        });
                    }
                }
                // Some((i, '(')) => return Some(spanned!(Token::LParen, 1)),
                // Some((i, ')')) => return Some(spanned!(Token::RParen, 1)),

                // Some((i, c @ '0'..='9')) => {
                //     return Some(spanned!(Token::Number(c.to_digit(10).unwrap()), 1))
                // }

                // Some((i, c)) => return Some(spanned!(Token::Char(c), 1)),
                // Some((i, '\n')) => {
                //     self.col = 0;
                //     self.line = 0;
                // }
                None => {
                    // println!("Got nothing {:#?}", self);
                    return Some(Token {
                        token_type: Self::YYEOF,
                        loc: Loc {
                            begin: self.col,
                            end: self.col,
                        },
                    });
                }
                _ => continue,
            }
        }
    }
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Token {
    pub token_type: i32,
    pub loc: Loc,
}
