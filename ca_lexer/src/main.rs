use std::str::CharIndices;

fn main() {
    let lex = Lexer::new("fn f() 2");
    for tok in lex {
        println!("Tok {:#?}", tok);
    }
}

struct Lexer<'i> {
    chars: CharIndices<'i>,
}
impl<'i> Lexer<'i> {
    pub fn new(input: &'i str) -> Self {
        Lexer {
            chars: input.char_indices(),
        }
    }
}
macro_rules! spanned {
    ($token: expr, $start: expr, $len: expr) => {
        Spanned::new($token, Span::new($start, $len))
    };
}
impl<'i> Iterator for Lexer<'i> {
    type Item = Spanned<'i>;
    // type Item = Token<'i>;

    fn next(&mut self) -> Option<Self::Item> {
        macro_rules! matches {
            ($e: literal) => {
                self.chars.next().map(|c| c.1) == Some($e)
            };
        }
        loop {
            match self.chars.next() {
                Some((i, 'f')) => {
                    if matches!('n') {
                        return Some(spanned!(Token::Fn, i, 2));
                    } else {
                        return Some(spanned!(Token::Char('f'), i, 1));
                    }
                }
                Some((i, '(')) => return Some(spanned!(Token::LParen, i, 1)),
                Some((i, ')')) => return Some(spanned!(Token::RParen, i, 1)),

                Some((i, c @ '0'..='9')) => {
                    return Some(spanned!(Token::Number(c.to_digit(10).unwrap()), i, 1))
                }

                Some((i, c)) => return Some(spanned!(Token::Char(c), i, 1)),
                None => return None,
                _ => continue,
            }
        }
    }
}
#[derive(Debug)]
struct Spanned<'a> {
    token: Token<'a>,
    span: Span,
}
impl<'a> Spanned<'a> {
    fn new(token: Token<'a>, span: Span) -> Self {
        Self { span, token }
    }
}
#[derive(Debug, PartialEq)]
enum Token<'t> {
    Identifier(&'t str),
    LParen,
    RParen,
    Char(char),
    Number(u32),
    Fn,
}
#[derive(Debug)]
struct Span {
    // line_start: usize,
    // line_end: usize,
    col_start: usize,
    col_end: usize,
}

impl Span {
    fn new(start: usize, len: usize) -> Self {
        Span {
            col_start: start,
            col_end: start + len,
        }
    }
}
