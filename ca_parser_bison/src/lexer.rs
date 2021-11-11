use crate::{loc::Loc, value::Value};
use peekmore::{PeekMore, PeekMoreIterator};
use prev_iter::PrevPeekable;

#[derive(Debug)]
pub struct Lexer {
    chars: PeekMoreIterator<PrevPeekable<std::vec::IntoIter<(usize, char)>>>,
    spaces: String,
    col: usize,
}
impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            chars: PrevPeekable::new(input.char_indices().collect::<Vec<_>>().into_iter())
                .peekmore(),
            col: 0,
            spaces: String::new(),
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

    #[allow(unused_macros)]
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
        macro_rules! inc_col {
            ($i: expr) => {{
                self.col += $i;
                self.col
            }};
        }
        loop {
            let m = match self.chars.next() {
                // Some((i, 'f'))
                //     if peek_matches!('n')
                //         && self.chars.peek_nth(i + 1).map(|c| c.1) == Some(' ') =>
                // {
                //     self.advance_by(1).unwrap();
                //     Some(Token {
                //         token_type: Self::tFN,
                //         token_value: "fn".to_string(),
                //         spaces_before: std::mem::take(&mut self.spaces),

                //         loc: Loc {
                //             begin: self.col,
                //             end: inc_col!(2),
                //         },
                //     })
                // }
                Some((i, c)) if Self::bracket_to_token(c).is_some() => Some(Token {
                    token_type: Self::bracket_to_token(c).unwrap(),
                    token_value: c.to_string(),
                    spaces_before: std::mem::take(&mut self.spaces),

                    loc: Loc {
                        begin: self.col,
                        end: inc_col!(1),
                    },
                }),
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
                    let token_type = match token_value.as_str() {
                        "fn" => Self::tFN,
                        _ => Self::tIDENTIFIER
                    };
                    Some(Token {
                        loc: Loc {
                            begin: self.col,
                            end: inc_col!(tokens.len()),
                        },
                        token_type,
                        token_value,
                        spaces_before: std::mem::take(&mut self.spaces),
                    })
                }
                Some((i, ':')) => Some(Token {
                    loc: Loc {
                        begin: self.col,
                        end: inc_col!(1),
                    },
                    token_type: Self::tCOLON,
                    token_value: ":".to_string(),
                    spaces_before: std::mem::take(&mut self.spaces),
                }),
                Some((i, ',')) => Some(Token {
                    loc: Loc {
                        begin: self.col,
                        end: inc_col!(1),
                    },
                    token_type: Self::tCOMMA,
                    token_value: ",".to_string(),
                    spaces_before: std::mem::take(&mut self.spaces),
                }),

                Some((_, s @ ' ')) => {
                    self.spaces.push(s);
                    self.col += 1;
                    continue;
                }
                None => Some(Token {
                    token_type: Self::YYEOF,
                    token_value: "".to_string(),
                    spaces_before: std::mem::take(&mut self.spaces),

                    loc: Loc {
                        begin: self.col,
                        end: inc_col!(1),
                    },
                }),
                _ => continue,
            };

            // self.col+=1;
            return m;
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: i32,
    pub token_value: String, //TODO: this should be something more like bytes, string is horrible here!
    pub loc: Loc,
    pub spaces_before: String,
}
impl Token {
    pub fn from(v: Value) -> Token {
        match v {
            Value::Token(t) => t,
            _ => panic!("wrong"),
        }
    }
}
