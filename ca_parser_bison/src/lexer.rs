use std::{fmt::Debug, result::IntoIter, str::CharIndices};

use crate::{loc::Loc, parser, value::Value};
use peekmore::{PeekMore, PeekMoreIterator};
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
    chars: PeekMoreIterator<PrevPeekable<std::vec::IntoIter<(usize, char)>>>,
    line: usize,
    col: usize,
}
impl Lexer {
    pub fn new(input: &str) -> Self {
        // self.chars.into_iter()
        Lexer {
            chars: PrevPeekable::new(input.char_indices().collect::<Vec<_>>().into_iter())
                .peekmore(),
            col: 0,
            line: 0,
        }
    }
    pub fn yylex(&mut self) -> Token {
        self.next().unwrap()
    }
    fn bracket_to_token(c: char) -> Option<i32> {
        match c {
            '(' => Some(Lexer::tLPAREN),
            ')' => Some(Lexer::tRPAREN),
            '{' => Some(Lexer::tLBRACK),
            '}' => Some(Lexer::tRBRACK),
            _ => None,
        }
    }
    
}

impl Iterator for Lexer {
    type Item = Token;
    // type Item = Token<'i>;

    fn next(&mut self) -> Option<Self::Item> {
        macro_rules! _matches {
            ($e: literal) => {
                self.chars.peek().map(|c| c.1) == Some($e)
            };
        }
        macro_rules! peek1_matches {
            ($e: literal) => {
                self.chars.peek_forward(1).map(|c| c.1) == Some($e)
            };
        }
        macro_rules! peek_matches {
            ($e: literal) => {
                self.chars.peek().map(|c| c.1) == Some($e)
            };
        }
        macro_rules! next_matches {
            ($e: literal) => {
                self.chars.next().map(|c| c.1) == Some($e)
            };
        }
        // macro_rules! spanned {
        //     ($token: expr, $len: expr) => {
        //         Spanned::new($token, Span::new(self.col, $len, self.line))
        //     };
        // }

        loop {
            match self.chars.next() {
                Some((i, 'f'))
                    if peek_matches!('n')
                        && self.chars.peek_nth(i + 1).map(|c| c.1) == Some(' ') =>
                {
                    self.advance_by(1).unwrap();
                    return Some(Token {
                        token_type: Self::tFN,
                        token_value: "fn".to_string(),
                        loc: Loc {
                            begin: i,
                            end: i + 2,
                        },
                    });
                }
                Some((i, c)) if Self::bracket_to_token(c).is_some() => {
                    return Some(Token {
                        token_type: Self::bracket_to_token(c).unwrap(),
                        token_value: c.to_string(),
                        loc: Loc {
                            begin: i,
                            end: i + 1,
                        },
                    })
                }
                // Some((i, '(')) => return Some(spanned!(Token::LParen, 1)),
                // Some((i, ')')) => return Some(spanned!(Token::RParen, 1)),

                // Some((i, c @ '0'..='9')) => {
                //     return Some(spanned!(Token::Number(c.to_digit(10).unwrap()), 1))
                // }
                Some((i, c)) if c.is_alphabetic() => {
                    let mut tokens = vec![c];
                    let mut current = 0;
                    while let Some((_, value)) = self.chars.peek_nth(current) {
                        if !char::is_alphanumeric(*value) {
                            break;
                        }
                        tokens.push(*value);
                        current += 1;
                    }
                    self.chars.advance_by(current).unwrap();
                    let token_value = tokens.iter().fold(String::new(), |mut s, c| {
                        s.push(*c);
                        s
                    });
                    return Some(Token {
                        loc: Loc {
                            begin: i,
                            end: i + tokens.len(),
                        },
                        token_type: Self::tIDENTIFIER,
                        token_value,
                    });
                }
                Some((i, ':')) => {
                    return Some(Token {
                        loc: Loc {
                            begin: i,
                            end: i + 1,
                        },
                        token_type: Self::tCOLON,
                        token_value: ":".to_string(),
                    });
                }
                Some((i, ',')) => {
                    return Some(Token {
                        loc: Loc {
                            begin: i,
                            end: i + 1,
                        },
                        token_type: Self::tCOMMA,
                        token_value: ",".to_string(),
                    });
                }
                // Some((i, c)) => return Some(spanned!(Token::Char(c), 1)),
                // Some((i, '\n')) => {
                //     self.col = 0;
                //     self.line = 0;
                // }
                Some((i, ' ')) => continue,
                None => {
                    // println!("Got nothing {:#?}", self);
                    return Some(Token {
                        token_type: Self::YYEOF,
                        token_value: "".to_string(),
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
#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: i32,
    pub token_value: String, //TODO: this should be something more like bytes, string is horrible here!
    pub loc: Loc,
}
impl Token {
    pub fn from(v: Value) -> Token {
        match v {
           
            Value::Token(t) => t,
           _ => panic!("wrong")
        }
    }
}
