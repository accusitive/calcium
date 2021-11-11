%expect 0

%define api.parser.struct {Parser}

%define api.value.type {Value}

%define parse.error custom
%define parse.trace

%code use {
    // all use goes here
    // use crate::lexer::Lexer;
    // use ca_lexer::Token;
    // use ca_lexer::YYLexer;
    use crate::lexer::Lexer;
    use crate::lexer::Token;
    use crate::loc::Loc;
    use crate::value::Value;
    use crate::value::Number;

}

%code parser_fields {
     result: Option<i32>,
    /// Just an extra field for demonstration
    pub name: String,
    /// Enables debug printing
    pub debug: bool,
}

%token
    tPLUS   "+"
    tMINUS  "-"
    tMUL    "*"
    tDIV    "/"
    tLPAREN "("
    tRPAREN ")"
    tINT    "int"
    tIDENTIFIER "local variable or method"
    tNUM    "number"
    tFN     "fn"
    tERROR  "controlled YYERROR"
    tABORT  "controlled YYABORT"
    tACCEPT "controlled YYACCEPT"


%left "-" "+"
%left "*" "/"
%type <Number> expr number program

%%

 program: expr
            {
                self.result = Some($<Number>1);
                $$ = Value::None;
            }
        | error
            {
                self.result = None;
                $$ = Value::None;
            }

    expr: number
            {
                $$ = $1;
            }
        | tLPAREN expr tRPAREN
            {
                $$ = $2;
            }
        | expr tPLUS expr
            {
                $$ = Value::Number($<Number>1 + $<Number>3);
            }
        | expr tMINUS expr
            {
                $$ = Value::Number($<Number>1 - $<Number>3);
            }
        | expr tMUL expr
            {
                $$ = Value::Number($<Number>1 * $<Number>3);
            }
        | expr tDIV expr
            {
                $$ = Value::Number($<Number>1 / $<Number>3);
            }
        | tFN {
                $$ = Value::Number(5)
            }

  number: tNUM
            {
                // $$ = Value::Number($<Token>1.token_value);
               $$ = Value::Number(55);
            }

%%

impl Parser {
        /// "Sucess" status-code of the parser
    pub const ACCEPTED: i32 = -1;

    /// "Failure" status-code of the parser
    pub const ABORTED: i32 = -2;

    /// Constructor
    pub fn new<'b> /* ' */ (lexer: Lexer, name: &str) -> Self {
        Self {
            yy_error_verbose: true,
            yynerrs: 0,
            debug: false,
            yyerrstatus_: 0,
            yylexer: lexer,
            result: None,
            name: name.to_owned(),
        }
    }

    /// Wrapper around generated `parse` method that also
    /// extracts `result` field and returns it.
    pub fn do_parse(mut self) -> (Option<i32>, String) {
        self.parse();
        (self.result, self.name)
    }

    fn next_token(&mut self) -> Token {
        self.yylexer.yylex()
    }

    fn report_syntax_error(&self, stack: &YYStack, yytoken: &SymbolKind, loc: YYLoc) {
        eprintln!("report_syntax_error: {:#?} {:?} {:?}", stack, yytoken, loc)
    }
}
