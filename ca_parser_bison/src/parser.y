%expect 0

%define api.parser.struct {Parser}

%define api.value.type {Value}

%define parse.error custom
%define parse.trace

%code use {
    // NOTE: This (parser.rs) is auto generated by ca_parser_bison/src/parser.y working with ca_parser_bison/build.rs
    use crate::lexer::Lexer;
    use crate::lexer::Token;
    use crate::loc::Loc;
    use crate::value::*;
    use std::path::PathBuf;
}

%code parser_fields {
     result: Option<i32>,
    /// Just an extra field for demonstration
    pub name: String,
    /// Enables debug printing
    pub debug: bool,
    pub output: Option<Value>,
    source: String,
    path: PathBuf
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
    tPERIOD "."
tAMPERSAND  "&"
    tSEMICOLON ";"
    tPATHSEP"::"
    tCOMMA  ","
    tASSIGN "="
    tLET    "let"
    tSTR    "str"
    tI32    "i32"
    tI64    "i64"
    tI128   "i128"
    tU32    "u32"
    tU64    "u64"
    tI8     "i8"
    tSTRING "Text wrapped in quotes"
    kwRETURN "return"
    kwSTRUCT "struct"
    kwIMPORT "import"
    kwNEW   "new"
    kwEXTERN "extern"

    tINFER "_"
    tIDENTIFIER "local variable or method"
    tNUM    "number"
    tFN     "fn"
    tERROR  "controlled YYERROR"
    tABORT  "controlled YYABORT"
    tACCEPT "controlled YYACCEPT"

%left "-" "+"
%left "*" "/"
%left "."
%type <Value>
    function
    function_arg
    function_args
    program
    identifier
    path_segment
    path
    statement
    field_expression
    let_stmt
    return_stmt
    statements
    block_expr
    expr
    literal_expr
    ty
    call_expr
    call_params
    struct
    struct_field
    struct_fields
    item
    items
    path_expr
    import
    expr_stmt
    none
    integer_literal
    string_literal


%%
 program: items {
     self.result = Some(0);
     self.output = Some(Value::Program(Box::new($1)));
    $$ = Value::Program(Box::new($1))
 } | error {
     self.result = None;
     $$ = Value::None;
 }
items: item {
     $$ = Value::ValueList(vec![$1]);
 }
 | items item {
     let mut vl = $<ValueList>1;
     vl.push($2);
     let v = Value::ValueList(vl);
     $$ = v;
 }
 item: struct{
     $$ = Value::Item(Box::new($1))
 }
 | function {
     $$ = Value::Item(Box::new($1))
 }
 | import {
     $$ = Value::Item(Box::new($1))
 }
 struct: kwSTRUCT identifier tLBRACK struct_fields tRBRACK {
     $$ = Value::Struct(Box::new($2), Box::new($4))
 }
 struct_fields: struct_field {
     $$ = Value::ValueList(vec![$1])
 }
 | struct_fields tCOMMA struct_field {
     let mut vl = $<crate::value::ValueList>1;
     vl.push($3);
     $$ = Value::ValueList(vl);
 }
 struct_field: identifier tCOLON ty {
     $$ = Value::StructField(Box::new($1), Box::new($3))
 }

 function: tFN identifier tLPAREN function_args tRPAREN tCOLON ty block_expr {
     $$ = Value::Function($<Ident>2, Box::new($4), Box::new($7), Some(Box::new($8)), false, false);
 }
 | kwEXTERN tFN identifier tLPAREN function_args tRPAREN tCOLON ty {
     $$ = Value::Function($<Ident>3, Box::new($5), Box::new($8), None, true, false)
 }
 | kwEXTERN tFN identifier tPERIOD tPERIOD tPERIOD tLPAREN function_args tRPAREN tCOLON ty {
     $$ = Value::Function($<Ident>3, Box::new($8), Box::new($11), None, true, true)
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
  function_arg: identifier tCOLON ty {
     $$ = Value::FunctionArg($<Ident>1, Box::new($3));
 }
 | none {
     $$ = Value::None;
 }
import: kwIMPORT identifier {
    // let mut p = PathBuf::new();
    let mut p = self.path.clone();
    assert!(p.pop());
    let id = $<Ident>2;
    p.push(id.clone());
    p.set_extension("ca");
    let src = std::fs::read_to_string(p.clone()).expect(&format!("Dependency file {:?} not found.", p));
    let lexer = Lexer::new(&src);
    let parser = Parser::new(lexer, "idk", &src, p);
    let prog = parser.do_parse().2.unwrap();
    $$ = Value::Import(Box::new(Value::Ident(id)), Box::new(prog));
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

 block_expr: tLBRACK statements tRBRACK {
     $$ = Value::BlockExpr(Box::new($2))
 }
 | tLBRACK tRBRACK {
     $$ = Value::BlockExpr(Box::new(Value::ValueList(vec![])))
 }
 statements: statement {
    $$ = Value::ValueList(vec![$1]);
 }
 | statements tSEMICOLON statement {
     let mut stmts = $<ValueList>1;
     stmts.push($3);
     let v = Value::ValueList(stmts);
     $$ = v;
 } 
 statement: let_stmt {
     $$ = Value::Statement(Box::new($1));
 }
 | return_stmt {
     $$ = Value::Statement(Box::new($1));
 }
 | expr_stmt {
     $$ = Value::Statement(Box::new($1))
 }
 let_stmt: tLET identifier tCOLON ty tASSIGN expr {
     $$ = Value::LetStatement(Box::new($2), Box::new($4), Box::new($6));
 }
 return_stmt: kwRETURN expr {
     $$ = Value::ReturnStatement(Box::new($2))
 }
 expr_stmt: expr {
     $$ = Value::ExprStatement(Box::new($1))
 }


 ty: tI32 {
     $$ = Value::Ty(Box::new(Value::Int32))
 }
 | tI64 {
    $$ = Value::Ty(Box::new(Value::Int64))
 }
 | tI128 {
    $$ = Value::Ty(Box::new(Value::Int128))
 }
 | tU32 {
    $$ = Value::Ty(Box::new(Value::UInt32))
 }
 | tU64 {
    $$ = Value::Ty(Box::new(Value::UInt64))
 }
 | tI8 {
    $$ = Value::Ty(Box::new(Value::Int8))
 }
 | path {
     $$ = Value::Ty(Box::new($1))
 }
 | tINFER {
     $$ = Value::Ty(Box::new(Value::Infer))
 }
 | tAMPERSAND ty {
     $$ = Value::Ty(Box::new(Value::PointerTy(Box::new($2))))
 }
 | tLPAREN ty tSEMICOLON integer_literal tRPAREN {
     $$ = Value::Ty(Box::new(Value::ArrayTy(Box::new($2), Box::new($4))))
 }
 | tSTR {
     $$ = Value::Ty(Box::new(Value::StrTy))
 }

 call_params: none {
     $$ = Value::ValueList(vec![])
 } | expr {
     $$ = Value::ValueList(vec![$1])
 } | call_params tCOMMA expr {
     let mut params = $<ValueList>1;
     params.push($3);
     let v = Value::ValueList(params);
     $$ = v;
 }
 none: {
    $$ = Value::None
 }





 expr: literal_expr {
     $$ = Value::Expr(Box::new($1))
 }
 | kwNEW path tLPAREN call_params tRPAREN{
     $$ = Value::Expr(Box::new(Value::NewExpr(Box::new($2), Box::new($4))))
 }
 | block_expr {
     $$ = Value::Expr(Box::new($1))
 }
 | call_expr {
   $$ = Value::Expr(Box::new($1))
 }
 | tLPAREN expr tRPAREN {
     $$ = $2;
 }
 | path_expr {
     $$ = Value::Expr(Box::new($1))
 }
 | field_expression {
     $$ = Value::Expr(Box::new($1))
 }
 | expr tPLUS expr {
     $$ = Value::Expr(Box::new(Value::ArithExpr(Box::new($1), Op::Add, Box::new($3))))
 }
 | expr tMINUS expr {
     $$ = Value::Expr(Box::new(Value::ArithExpr(Box::new($1), Op::Sub, Box::new($3))))
 }
 | expr tMUL expr {
     $$ = Value::Expr(Box::new(Value::ArithExpr(Box::new($1), Op::Mul, Box::new($3))))
 }
 | expr tDIV expr {
     $$ = Value::Expr(Box::new(Value::ArithExpr(Box::new($1), Op::Div, Box::new($3))))
 }

 literal_expr: integer_literal {
    $$ = Value::LiteralExpr(Box::new($1))
 } 
 | string_literal {
    $$ = Value::LiteralExpr(Box::new($1))
 }
  integer_literal: tNUM ty {
    $$ = Value::IntegerLiteral($<Token>1.token_value, Box::new($2))
 }
 | tNUM {
     $$ = Value::IntegerLiteral($<Token>1.token_value, Box::new(Value::Ty(Box::new(Value::Int32))))
 }
 string_literal: tSTRING {
     $$ = Value::StringLiteral($<Token>1.token_value)
 }
 call_expr: path tLPAREN call_params tRPAREN {
     $$ = Value::CallExpr(Box::new($1), Box::new($3))
 }
 path_expr: path {
     $$ = Value::PathExpr(Box::new($1))
 }
 field_expression: expr tPERIOD identifier {
     $$ = Value::FieldExpr(Box::new($1), Box::new($3))
 }
 
%%

impl Parser {
        /// "Sucess" status-code of the parser
    pub const ACCEPTED: i32 = -1;

    /// "Failure" status-code of the parser
    pub const ABORTED: i32 = -2;

    /// Constructor
    pub fn new<'b> /* ' */ (lexer: Lexer, name: &str, source: &str, path: PathBuf) -> Self {
        Self {
            yy_error_verbose: true,
            yynerrs: 0,
            debug: false,
            yyerrstatus_: 0,
            yylexer: lexer,
            result: None,
            name: name.to_owned(),
            output: None,
            source: source.to_string(),
            path
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
        crate::pretty::print_error(&source, loc.to_range(), self.yylexer.line+1);
    }
}
