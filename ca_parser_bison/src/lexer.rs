#[derive(Debug)]
pub struct Lexer {
    inner: ca_lexer::YYLexer
}
impl Lexer {
    pub fn yylex(&self) -> ca_lexer::Spanned {
        self.inner.next().unwrap()
    }
}