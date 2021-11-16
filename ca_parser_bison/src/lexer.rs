use crate::{loc::Loc, value::Value};
use peekmore::{PeekMore, PeekMoreIterator};

#[derive(Debug)]
pub struct Lexer {
    chars: PeekMoreIterator<std::vec::IntoIter<char>>,
    pub spaces: String,
    pub col: usize,
    pub line: usize,
    pub is_comment: bool,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            chars: input.chars().collect::<Vec<_>>().into_iter().peekmore(),
            col: 0,
            line: 0,
            spaces: String::new(),
            is_comment: false,
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
static TOKS: &[(char, i32)] = &[
    (':', Lexer::tCOLON),
    (',', Lexer::tCOMMA),
    ('.', Lexer::tPERIOD),
    ('=', Lexer::tASSIGN),
    ('+', Lexer::tPLUS),
    ('-', Lexer::tMINUS),
    ('*', Lexer::tMUL),
    ('/', Lexer::tDIV),
    ('_', Lexer::tINFER),
    (';', Lexer::tSEMICOLON),
    ('&', Lexer::tAMPERSAND),
    ('>', Lexer::tGT),
    ('<', Lexer::tLT),
];
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
        macro_rules! loc {
            ($i: expr) => {
                Loc {
                    begin: self.col,
                    end: inc_col!($i),
                }
            };
        }
        loop {
            let m = match self.chars.next() {
                // Brackets () {}
                Some('#') => {
                    self.is_comment = true;
                    continue;
                }
                Some(n @ '\n') => {
                    self.spaces.push(n);
                    self.col = 0;
                    self.line += 1;
                    self.is_comment = false;
                    continue;
                    // self.line +=1;
                }
                Some(_) if self.is_comment => continue,
                Some(c) if Self::bracket_to_token(c).is_some() => Some(Token {
                    token_type: Self::bracket_to_token(c).unwrap(),
                    token_value: c.to_string(),
                    spaces_before: std::mem::take(&mut self.spaces),

                    loc: loc!(1),
                }),
                // Double colon
                Some(':') if self.chars.peek_nth(0) == Some(&':') => {
                    self.chars.next().unwrap();
                    Some(Token {
                        loc: loc!(2),
                        token_type: Self::tPATHSEP,
                        token_value: "::".to_string(),
                        spaces_before: std::mem::take(&mut self.spaces),
                    })
                }
                // String literal
                Some('"') => {
                    let mut tokens = vec![];
                    let mut current = 0;
                    while let Some(value) = self.chars.peek_nth(current) {
                        if *value == '\"' {
                            break;
                        }
                        if *value == '\\' {
                            // let c = self.chars.peek_nth(3).unwrap();
                            let c = { self.chars.peek_nth(current + 1).unwrap() };
                            // self.chars.next().unwrap();

                            let escaped = match c {
                                'n' => '\n',
                                't' => '\t',
                                '\\' => '\\',
                                '"' => '"',
                                'r' => '\r',
                                ' ' => panic!("space"),
                                _ => panic!("Invalid escaped character {:#?}", c.escape_unicode()),
                            };
                            current += 1;
                            // self.chars.next().unwrap();
                            tokens.push(escaped);
                        } else {
                            tokens.push(*value);
                        }
                        current += 1;
                    }
                    // Has to be plus one to account for the second quote.
                    for _ in 0..current + 1 {
                        self.chars.next();
                    }
                    let token_value = tokens.iter().fold(String::new(), |mut s, c| {
                        s.push(*c);
                        s
                    });
                    Some(Token {
                        loc: loc!(tokens.len()),
                        token_type: Self::tSTRING,
                        token_value,
                        spaces_before: std::mem::take(&mut self.spaces),
                    })
                }
                // Identifier
                Some(c) if c.is_alphabetic() => {
                    let mut tokens = vec![c];
                    let mut current = 0;
                    while let Some(value) = self.chars.peek_nth(current) {
                        if !char::is_alphanumeric(*value) {
                            break;
                        }
                        tokens.push(*value);
                        current += 1;
                    }

                    for _ in 0..current {
                        self.chars.next();
                    }
                    let token_value = tokens.iter().fold(String::new(), |mut s, c| {
                        s.push(*c);
                        s
                    });
                    let token_type = match token_value.as_str() {
                        "fn" => Self::tFN,
                        "let" => Self::tLET,
                        "return" => Self::kwRETURN,
                        "extern" => Self::kwEXTERN,
                        "struct" => Self::kwSTRUCT,
                        "import" => Self::kwIMPORT,
                        "new" => Self::kwNEW,
                        "i32" => Self::tI32,
                        "i64" => Self::tI64,
                        "true" => Self::tBOOL,
                        "false" => Self::tBOOL,
                        "bool" => Self::kwBOOL,
                        "u32" => Self::tU32,
                        "u64" => Self::tU64,
                        "i8" => Self::tI8,
                        "if" => Self::tIF,
                        "else" => Self::tELSE,
                        _ => Self::tIDENTIFIER,
                    };
                    Some(Token {
                        loc: loc!(tokens.len()),
                        token_type,
                        token_value,
                        spaces_before: std::mem::take(&mut self.spaces),
                    })
                }
                // Number literal
                Some(c) if c.is_numeric() => {
                    let mut tokens = vec![c];
                    let mut current = 0;
                    while let Some(value) = self.chars.peek_nth(current) {
                        let valid = char::is_numeric(*value) || c == '_';
                        if !valid {
                            break;
                        }
                        tokens.push(*value);
                        current += 1;
                    }

                    for _ in 0..current {
                        self.chars.next();
                    }
                    // self.chars.advance_by(current).unwrap();
                    let token_value = tokens.iter().fold(String::new(), |mut s, c| {
                        s.push(*c);
                        s
                    });

                    Some(Token {
                        loc: loc!(tokens.len()),
                        token_type: Self::tNUM,
                        token_value,
                        spaces_before: std::mem::take(&mut self.spaces),
                    })
                }

                Some(c) if TOKS.iter().map(|t| t.0).collect::<Vec<_>>().contains(&c) => {
                    let (_c, ty) = *TOKS.iter().find(|t| t.0 == c).unwrap();
                    // let ty = match c {
                    //     ':' => Self::tCOLON,
                    //     ',' => Self::tCOMMA,
                    //     '=' => Self::tASSIGN,
                    //     '+' => Self::tPLUS,
                    //     '-' => Self::tMINUS,
                    //     '*' => Self::tMUL,
                    //     '/' => Self::tDIV,
                    //     '_' => Self::tINFER,
                    //     ';' => Self::tSEMICOLON,
                    //     '&' => Self::tAMPERSAND,
                    //     _ => panic!("Invalid single character token, not possible."),
                    // };
                    Some(Token {
                        loc: loc!(1),
                        token_type: ty,
                        token_value: c.to_string(),
                        spaces_before: std::mem::take(&mut self.spaces),
                    })
                }

                Some(s @ ' ') => {
                    self.spaces.push(s);
                    self.col += 1;
                    continue;
                }

                None => Some(Token {
                    token_type: Self::YYEOF,
                    token_value: "".to_string(),
                    spaces_before: std::mem::take(&mut self.spaces),

                    loc: loc!(1),
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
#[test]
fn test_lex() {
    let source = "\"test\n\"";
    let mut lex = Lexer::new(source);
    assert_eq!(lex.next().unwrap().token_value, "test\n");
}
