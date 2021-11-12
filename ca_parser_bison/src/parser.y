%expect 0

%define api.parser.struct {Parser}

%define api.value.type {Value}

%define parse.error detailed
%define parse.trace

%code use {
    // NOTE: This parser is auto generated by ca_parser_bison/src/parser.y working with ca_parser_bison.build.rs
    use crate::lexer::Lexer;
    use crate::lexer::Token;
    use crate::loc::Loc;
    use crate::value::*;
}

%code parser_fields {
     result: Option<i32>,
    /// Just an extra field for demonstration
    pub name: String,
    /// Enables debug printing
    pub debug: bool,
    pub output: Option<Value>,
    source: String
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
    tPATHSEP "::"
    tCOMMA  ","
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
%type <PathSegment> path_segment
%type <Path> path
%%
 program: functions {
     self.result = Some(0);
     self.output = Some($1);
    $$ = $1
 }

 functions: function {
     $$ = Value::ValueList(vec![$1]);
 }
 | functions tCOMMA function {
     println!("multiple functions");
     let mut fns = $<ValueList>1;
     fns.push($3);
     let v = Value::ValueList(fns);
     $$ = v;
 }
 function: tFN identifier tLPAREN function_args tRPAREN tCOLON path {
     $$ = Value::Function($<Ident>2, vec![$4], Box::new($7));
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
 function_arg: identifier tCOLON path {
     $$ = Value::FunctionArg(Box::new($1), Box::new($3));
 }
 identifier: tIDENTIFIER {
     let tok = $<Token>1;
     $$ = Value::Ident(tok.token_value);
 }
 path: path_segment {
    $$ = Value::ValueList(vec![$1]);
 }
 | path tPATHSEP path_segment {
     let mut args = $<ValueList>1;
     args.push($3);
     let v = Value::ValueList(args);
     $$ = v;
 }
 path_segment: identifier {
     $$ = Value::Ident($<Ident>1);
 }
%%

impl Parser {
        /// "Sucess" status-code of the parser
    pub const ACCEPTED: i32 = -1;

    /// "Failure" status-code of the parser
    pub const ABORTED: i32 = -2;

    /// Constructor
    pub fn new<'b> /* ' */ (lexer: Lexer, name: &str, source: &str) -> Self {
        Self {
            yy_error_verbose: true,
            yynerrs: 0,
            debug: false,
            yyerrstatus_: 0,
            yylexer: lexer,
            result: None,
            name: name.to_owned(),
            output: None,
            source: source.to_string()
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

    fn report_syntax_error(&self, _stack: &YYStack, _yytoken: &SymbolKind, loc: YYLoc) {
        //TODO: Look into using stack for error messages
        let source = self.source.to_string();
        crate::pretty::print_error(&source, loc.to_range());
    }
}
