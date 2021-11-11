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
    use crate::value::Value::Function;
    use crate::value::*;
    // use crate::value::*:
}

%code parser_fields {
     result: Option<i32>,
    /// Just an extra field for demonstration
    pub name: String,
    /// Enables debug printing
    pub debug: bool,
    pub output: Option<Value>
}

%token
    tPLUS   "+"
    tMINUS  "-"
    tMUL    "*"
    tDIV    "/"
    tLPAREN "("
    tRPAREN ")"
    tLBRACK "{"
    tRBRACK "}"
    tCOLON  ":"
    tCOMMA ","
    tINT    "int"
    tIDENTIFIER "local variable or method"
    tNUM    "number"
    tFN     "fn"
    tERROR  "controlled YYERROR"
    tABORT  "controlled YYABORT"
    tACCEPT "controlled YYACCEPT"

%left "-" "+"
%left "*" "/"
%type <Function> function
%type <Functions> functions
%type <FunctionArg> function_arg
%type <FunctionArgs> function_args
%type <Program> program
%type <Ident> identifier
%%
 program: functions {
     self.result = Some(0);
     self.output = Some($1);
    $$ = $1
 }

 functions: function {
     $$ = Value::ValueList(vec![$1]);
 } | functions function {
     let mut fns = $<ValueList>1;
     let v = Value::ValueList(fns);
     $$ = v;
 }
 function: tFN identifier tLPAREN function_args tRPAREN {
     $$ = Value::Function($<Ident>2, vec![$4]);
 }
 function_args: function_arg {
     $$ = Value::ValueList(vec![$1]);
 }
 | function_args tCOMMA function_arg {
     let mut args = $<crate::value::ValueList>1;
     args.push($3);
     let v = Value::ValueList(args);
     $$ = v;
 }
 function_arg: identifier tCOLON identifier {
     $$ = Value::FunctionArg(Box::new($1), Box::new($3));
 }
 identifier: tIDENTIFIER {
     let tok = $<Token>1;
     println!("Tok {:#?}", tok);
     $$ = Value::Ident(tok.token_value);
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
            output: None
        }
    }

    /// Wrapper around generated `parse` method that also
    /// extracts `result` field and returns it.
    pub fn do_parse(mut self) -> (Option<i32>, String, Option<Value>) {
        self.parse();
        (self.result, self.name, self.output)
    }

    fn next_token(&mut self) -> Token {
        self.yylexer.yylex()
    }

    fn report_syntax_error(&self, stack: &YYStack, yytoken: &SymbolKind, loc: YYLoc) {
        eprintln!("report_syntax_error: {:#?} {:?} {:?}", stack, yytoken, loc)
    }
}
