/* A Bison parser, made by GNU Bison 3.8.2.  */

/* Skeleton implementation for Bison LALR(1) parsers in Rust

   Copyright (C) 2007-2015, 2018-2020 Free Software Foundation, Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

/* As a special exception, you may create a larger work that contains
   part or all of the Bison parser skeleton and distribute that work
   under terms of your choice, so long as that work isn't itself a
   parser generator using the skeleton or a modified version thereof
   as a parser skeleton.  Alternatively, if you modify or redistribute
   the parser skeleton itself, you may (at your option) remove this
   special exception, which will cause the skeleton and the resulting
   Bison output files to be licensed under the GNU General Public
   License without this special exception.

   This special exception was added by the Free Software Foundation in
   version 2.2 of Bison.  */

/* DO NOT RELY ON FEATURES THAT ARE NOT DOCUMENTED in the manual,
   especially those whose name start with YY_ or yy_.  They are
   private implementation details that can be changed or removed.  */




use std::convert::TryInto;


/* "%code use" blocks.  */
/* "src/parser.y":10  */

    use crate::lexer::Lexer;
    use crate::lexer::Token;
    use crate::loc::Loc;
    use crate::value::*;

/* "src/parser.rs":52  */


/// A Bison parser, automatically generated from src/parser.y.
#[derive(Debug)]
pub struct Parser {
    /// Lexer that is used to get tokens
    pub yylexer: Lexer,
    // true if verbose error messages are enabled.
    #[allow(dead_code)]
    yy_error_verbose: bool,
    // number of errors so far
    yynerrs: i32,

    yyerrstatus_: i32,

    /* "%code parser_fields" blocks.  */
/* "src/parser.y":17  */

     result: Option<i32>,
    /// Just an extra field for demonstration
    pub name: String,
    /// Enables debug printing
    pub debug: bool,
    pub output: Option<Value>,
    source: String

/* "src/parser.rs":79  */

}

#[inline]
fn i32_to_usize(v: i32) -> usize {
    v as usize
}

/// Maps token ID into human-readable name
pub fn token_name(id: i32) -> &'static str { /* ' */
    let first_token = Lexer::YYerror;
    if id > first_token + 1 {
        let pos: usize = (id - first_token + 1)
            .try_into()
            .expect("failed to cast token id into usize, is it negative?");
        Lexer::TOKEN_NAMES[pos]
    } else if id == 0 {
        "EOF"
    } else {
        panic!("token_name fails, {} (first token = {})", id, first_token)
    }
}

/// Local alias
type YYLoc = Loc;

impl Parser {
    // Version number for the Bison executable that generated this parser.
    #[allow(dead_code)]
    const BISON_VERSION: &'static str = "30802";

}


fn make_yylloc(rhs: &YYStack, n: usize) -> YYLoc {
    if 0 < n {
        YYLoc {
            begin: rhs.location_at(n - 1).begin,
            end: rhs.location_at(0).end
        }
    } else {
        YYLoc {
            begin: rhs.location_at(0).end,
            end: rhs.location_at(0).end
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SymbolKind { value: i32 }

impl SymbolKind {



    #[allow(non_upper_case_globals)]
    const S_YYEOF: i32 = 0;        /* "end of file"  */

    #[allow(non_upper_case_globals)]
    const S_YYerror: i32 = 1;      /* error  */

    #[allow(non_upper_case_globals)]
    const S_YYUNDEF: i32 = 2;      /* "invalid token"  */

    #[allow(non_upper_case_globals)]
    const S_tPLUS: i32 = 3;        /* "+"  */

    #[allow(non_upper_case_globals)]
    const S_tMINUS: i32 = 4;       /* "-"  */

    #[allow(non_upper_case_globals)]
    const S_tMUL: i32 = 5;         /* "*"  */

    #[allow(non_upper_case_globals)]
    const S_tDIV: i32 = 6;         /* "/"  */

    #[allow(non_upper_case_globals)]
    const S_tLPAREN: i32 = 7;      /* "("  */

    #[allow(non_upper_case_globals)]
    const S_tRPAREN: i32 = 8;      /* ")"  */

    #[allow(non_upper_case_globals)]
    const S_tLBRACK: i32 = 9;      /* "{"  */

    #[allow(non_upper_case_globals)]
    const S_tRBRACK: i32 = 10;     /* "}"  */

    #[allow(non_upper_case_globals)]
    const S_tCOLON: i32 = 11;      /* ":"  */

    #[allow(non_upper_case_globals)]
    const S_tPATHSEP: i32 = 12;    /* "::"  */

    #[allow(non_upper_case_globals)]
    const S_tCOMMA: i32 = 13;      /* ","  */

    #[allow(non_upper_case_globals)]
    const S_tINT: i32 = 14;        /* "int"  */

    #[allow(non_upper_case_globals)]
    const S_tIDENTIFIER: i32 = 15; /* "local variable or method"  */

    #[allow(non_upper_case_globals)]
    const S_tNUM: i32 = 16;        /* "number"  */

    #[allow(non_upper_case_globals)]
    const S_tFN: i32 = 17;         /* "fn"  */

    #[allow(non_upper_case_globals)]
    const S_tERROR: i32 = 18;      /* "controlled YYERROR"  */

    #[allow(non_upper_case_globals)]
    const S_tABORT: i32 = 19;      /* "controlled YYABORT"  */

    #[allow(non_upper_case_globals)]
    const S_tACCEPT: i32 = 20;     /* "controlled YYACCEPT"  */

    #[allow(non_upper_case_globals)]
    const S_YYACCEPT: i32 = 21;    /* $accept  */

    #[allow(non_upper_case_globals)]
    const S_program: i32 = 22;     /* program  */

    #[allow(non_upper_case_globals)]
    const S_functions: i32 = 23;   /* functions  */

    #[allow(non_upper_case_globals)]
    const S_function: i32 = 24;    /* function  */

    #[allow(non_upper_case_globals)]
    const S_function_args: i32 = 25; /* function_args  */

    #[allow(non_upper_case_globals)]
    const S_function_arg: i32 = 26; /* function_arg  */

    #[allow(non_upper_case_globals)]
    const S_identifier: i32 = 27;  /* identifier  */

    #[allow(non_upper_case_globals)]
    const S_path: i32 = 28;        /* path  */

    #[allow(non_upper_case_globals)]
    const S_path_segment: i32 = 29; /* path_segment  */


    const VALUES_: &'static [SymbolKind] = &[ 
        SymbolKind { value: SymbolKind::S_YYEOF },
        SymbolKind { value: SymbolKind::S_YYerror },
        SymbolKind { value: SymbolKind::S_YYUNDEF },
        SymbolKind { value: SymbolKind::S_tPLUS },
        SymbolKind { value: SymbolKind::S_tMINUS },
        SymbolKind { value: SymbolKind::S_tMUL },
        SymbolKind { value: SymbolKind::S_tDIV },
        SymbolKind { value: SymbolKind::S_tLPAREN },
        SymbolKind { value: SymbolKind::S_tRPAREN },
        SymbolKind { value: SymbolKind::S_tLBRACK },
        SymbolKind { value: SymbolKind::S_tRBRACK },
        SymbolKind { value: SymbolKind::S_tCOLON },
        SymbolKind { value: SymbolKind::S_tPATHSEP },
        SymbolKind { value: SymbolKind::S_tCOMMA },
        SymbolKind { value: SymbolKind::S_tINT },
        SymbolKind { value: SymbolKind::S_tIDENTIFIER },
        SymbolKind { value: SymbolKind::S_tNUM },
        SymbolKind { value: SymbolKind::S_tFN },
        SymbolKind { value: SymbolKind::S_tERROR },
        SymbolKind { value: SymbolKind::S_tABORT },
        SymbolKind { value: SymbolKind::S_tACCEPT },
        SymbolKind { value: SymbolKind::S_YYACCEPT },
        SymbolKind { value: SymbolKind::S_program },
        SymbolKind { value: SymbolKind::S_functions },
        SymbolKind { value: SymbolKind::S_function },
        SymbolKind { value: SymbolKind::S_function_args },
        SymbolKind { value: SymbolKind::S_function_arg },
        SymbolKind { value: SymbolKind::S_identifier },
        SymbolKind { value: SymbolKind::S_path },
        SymbolKind { value: SymbolKind::S_path_segment }
    ];

    pub(crate) fn get(n: i32) -> &'static SymbolKind {
        &Self::VALUES_[i32_to_usize(n)]
    }

    pub(crate) fn code(&self) -> i32 {
        self.value
    }

    /* YYNAMES_[SYMBOL-NUM] -- String name of the symbol SYMBOL-NUM.
    First, the terminals, then, starting at \a YYNTOKENS_, nonterminals.  */
    #[allow(non_upper_case_globals)]
const yynames_: &'static [&'static str] = &[ "end of file", "error", "invalid token", "+", "-", "*", "/", "(", ")",
  "{", "}", ":", "::", ",", "int", "local variable or method", "number",
  "fn", "controlled YYERROR", "controlled YYABORT", "controlled YYACCEPT",
  "$accept", "program", "functions", "function", "function_args",
  "function_arg", "identifier", "path", "path_segment", "<<NULL>>" ] ;

    /* The user-facing name of this symbol.  */
    pub(crate) fn name(&self) -> String {
        let code: usize = self.code().try_into().unwrap();
        Self::yynames_[code].to_owned()
    }
}


const DYMMY_SYMBOL_KIND: SymbolKind = SymbolKind { value: 0 };

impl Lexer {
        /* Token kinds.  */
    /// Token `` "end of file" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const YYEOF: i32 = 0;
    /// Token `` error ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const YYerror: i32 = 256;
    /// Token `` "invalid token" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const YYUNDEF: i32 = 257;
    /// Token `` "+" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tPLUS: i32 = 258;
    /// Token `` "-" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tMINUS: i32 = 259;
    /// Token `` "*" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tMUL: i32 = 260;
    /// Token `` "/" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tDIV: i32 = 261;
    /// Token `` "(" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tLPAREN: i32 = 262;
    /// Token `` ")" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tRPAREN: i32 = 263;
    /// Token `` "{" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tLBRACK: i32 = 264;
    /// Token `` "}" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tRBRACK: i32 = 265;
    /// Token `` ":" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tCOLON: i32 = 266;
    /// Token `` "::" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tPATHSEP: i32 = 267;
    /// Token `` "," ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tCOMMA: i32 = 268;
    /// Token `` "int" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tINT: i32 = 269;
    /// Token `` "local variable or method" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tIDENTIFIER: i32 = 270;
    /// Token `` "number" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tNUM: i32 = 271;
    /// Token `` "fn" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tFN: i32 = 272;
    /// Token `` "controlled YYERROR" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tERROR: i32 = 273;
    /// Token `` "controlled YYABORT" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tABORT: i32 = 274;
    /// Token `` "controlled YYACCEPT" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tACCEPT: i32 = 275;


    // Deprecated, use YYEOF instead.
    #[allow(dead_code)]
    const EOF: i32 = Self::YYEOF;

    // Token values
    #[allow(dead_code)]
    pub(crate) const TOKEN_NAMES: &'static [&'static str] = &    [

    "YYEOF",

    "YYerror",

    "YYUNDEF",

    "tPLUS",

    "tMINUS",

    "tMUL",

    "tDIV",

    "tLPAREN",

    "tRPAREN",

    "tLBRACK",

    "tRBRACK",

    "tCOLON",

    "tPATHSEP",

    "tCOMMA",

    "tINT",

    "tIDENTIFIER",

    "tNUM",

    "tFN",

    "tERROR",

    "tABORT",

    "tACCEPT",

]
;
}


impl Parser {

    fn yycdebug(&self, s: &str) {
        if false {
            eprintln!("{}", s);
        }
    }

}

/// Local alias
type YYValue = Value;

#[derive(Debug)]
struct YYStackItem {
    state: i32,
    value: YYValue,
    loc: YYLoc,
}

#[derive(Debug)]
pub struct YYStack {
    stack: Vec<YYStackItem>,
}

impl YYStack {
    pub(crate) fn new() -> Self {
        Self {
          stack: Vec::with_capacity(20),
        }
    }

    pub(crate) fn push(&mut self, state: i32, value: YYValue, loc: YYLoc) {
        self.stack.push(YYStackItem { state, value, loc });
    }

    pub(crate) fn pop(&mut self) {
        self.stack.pop();
    }

    pub(crate) fn pop_n(&mut self, num: usize) {
        let len = self.stack.len() - num;
        self.stack.truncate(len);
    }

    pub(crate) fn state_at(&self, i: usize) -> i32 {
        self.stack[self.len() - 1 - i].state
    }

    pub(crate) fn location_at(&self, i: usize) -> &YYLoc {
        &self.stack[self.len() - 1 - i].loc
    }

    pub(crate) fn borrow_value_at(&self, i: usize) -> &YYValue {
        &self.stack[self.len() - 1 - i].value
    }

    pub(crate) fn owned_value_at(&mut self, i: usize) -> YYValue {
        let len = self.len();
        std::mem::take(&mut self.stack[len - 1 - i].value)
    }

    pub(crate) fn len(&self) -> usize {
      self.stack.len()
    }
}

impl std::fmt::Display for YYStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let states = self.stack.iter().map(|e| e.state.to_string()).collect::<Vec<String>>().join(" ");
        let values = self.stack.iter().map(|e| format!("{:?}", e.value)).collect::<Vec<String>>().join(" ");
        f.write_fmt(format_args!("Stack now states = {} / values = {:?} ", states, values))
    }
}

impl Parser {
  /// Returned by a Bison action in order to stop the parsing process and
  /// return success (true).
  pub(crate) const YYACCEPT: i32 = 0;

  /// Returned by a Bison action in order to stop the parsing process and
  /// return failure (false).
  pub(crate) const YYABORT: i32 = 1;

  /// Returned by a Bison action in order to start error recovery without
  /// printing an error message.
  pub(crate) const YYERROR: i32 = 2;

  /// Internal return codes that are not supported for user semantic
  /// actions.
  pub(crate) const YYERRLAB: i32 = 3;
  pub(crate) const YYNEWSTATE: i32 = 4;
  pub(crate) const YYDEFAULT: i32 = 5;
  pub(crate) const YYREDUCE: i32 = 6;
  pub(crate) const YYERRLAB1: i32 = 7;
  #[allow(dead_code)]
  pub(crate) const YYRETURN: i32 = 8;

  /// Whether error recovery is being done.  In this state, the parser
  /// reads token until it reaches a known state, and then restarts normal
  /// operation.
  #[allow(dead_code)]
  pub(crate) fn recovering(&self) -> bool {
      self.yyerrstatus_ == 0
  }

    // Compute post-reduction state.
    // yystate:   the current state
    // yysym:     the nonterminal to push on the stack
    fn yy_lr_goto_state(&self, yystate: i32, yysym: i32) -> i32 {
        let idx = i32_to_usize(yysym - Self::YYNTOKENS_);
        let yyr = Self::yypgoto_[idx] + yystate;
        if (0..=Self::YYLAST_).contains(&yyr) {
            let yyr = i32_to_usize(yyr);
            if Self::yycheck_[yyr] == yystate {
                return Self::yytable_[yyr];
            }
        }
        Self::yydefgoto_[idx]
    }

    fn yyaction(&mut self, yyn: i32, yystack: &mut YYStack, yylen: &mut usize) -> Result<i32, ()> {
        // If YYLEN is nonzero, implement the default value of the action:
        // '$$ = $1'.  Otherwise, use the top of the stack.
        //
        // Otherwise, the following line sets YYVAL to garbage.
        // This behavior is undocumented and Bison
        // users should not rely upon it.
        #[allow(unused_assignments)]
        let mut yyval: YYValue = YYValue::Uninitialized;
        let yyloc: YYLoc = make_yylloc(yystack, *yylen);

        self.yy_reduce_print(yyn, yystack);

        match yyn {
              2 =>  /* program: functions  */
  /* "src/parser.y":58  */
                    {
     self.result = Some(0);
     self.output = Some( yystack.owned_value_at(0));
    yyval =  yystack.owned_value_at(0)
 },


  3 =>  /* functions: function  */
  /* "src/parser.y":64  */
                     {
     yyval = Value::ValueList(vec![ yystack.owned_value_at(0)]);
 },


  4 =>  /* functions: functions "," function  */
  /* "src/parser.y":67  */
                             {
     println!("multiple functions");
     let mut fns =  ValueList::from(yystack.owned_value_at(2));
     fns.push( yystack.owned_value_at(0));
     let v = Value::ValueList(fns);
     yyval = v;
 },


  5 =>  /* function: "fn" identifier "(" function_args ")" ":" path  */
  /* "src/parser.y":74  */
                                                                    {
     yyval = Value::Function( Ident::from(yystack.owned_value_at(5)), vec![ yystack.owned_value_at(3)], Box::new( yystack.owned_value_at(0)));
 },


  6 =>  /* function_args: function_arg  */
  /* "src/parser.y":77  */
                             {
     yyval = Value::ValueList(vec![ yystack.owned_value_at(0)]);
 },


  7 =>  /* function_args: function_args "," function_arg  */
  /* "src/parser.y":80  */
                                     {
     let mut args =  crate::value::ValueList::from(yystack.owned_value_at(2));
     args.push( yystack.owned_value_at(0));
     let v = Value::ValueList(args);
     yyval = v;
 },


  8 =>  /* function_arg: identifier ":" path  */
  /* "src/parser.y":86  */
                                      {
     yyval = Value::FunctionArg(Box::new( yystack.owned_value_at(2)), Box::new( yystack.owned_value_at(0)));
 },


  9 =>  /* identifier: "local variable or method"  */
  /* "src/parser.y":89  */
                         {
     let tok =  Token::from(yystack.owned_value_at(0));
     yyval = Value::Ident(tok.token_value);
 },


  10 =>  /* path: path_segment  */
  /* "src/parser.y":93  */
                    {
    yyval = Value::ValueList(vec![ yystack.owned_value_at(0)]);
 },


  11 =>  /* path: path "::" path_segment  */
  /* "src/parser.y":96  */
                              {
     let mut args =  ValueList::from(yystack.owned_value_at(2));
     args.push( yystack.owned_value_at(0));
     let v = Value::ValueList(args);
     yyval = v;
 },


  12 =>  /* path_segment: identifier  */
  /* "src/parser.y":102  */
                          {
     yyval = Value::Ident( Ident::from(yystack.owned_value_at(0)));
 },



/* "src/parser.rs":634  */

            _ => {}
        }

        if let YYValue::Uninitialized = yyval {
            panic!("yyval is Uninitialized in rule at line {}", Self::yyrline_[i32_to_usize(yyn)]);
        }

        self.yy_symbol_print("-> $$ =", SymbolKind::get(Self::yyr1_[i32_to_usize(yyn)]), &yyval, &yyloc);

        yystack.pop_n(*yylen);
        *yylen = 0;
        /* Shift the result of the reduction.  */
        let yystate = self.yy_lr_goto_state(yystack.state_at(0), Self::yyr1_[i32_to_usize(yyn)]);
        yystack.push(yystate, yyval, yyloc);
        Ok(Self::YYNEWSTATE)
    }

    // Print this symbol on YYOUTPUT.
    fn yy_symbol_print(&self, s: &str, yykind: &SymbolKind, yyvalue: &YYValue, yylocation: &YYLoc) {
        if false {
            self.yycdebug(
                &format!("{}{} {:?} ( {:?}: {:?} )", // " fix highlighting
                s,
                if yykind.code() < Self::YYNTOKENS_ { " token " } else { " nterm " },
                yykind.name(),
                yylocation,
                yyvalue
                )
            )
        }
    }

    /// Parses given input. Returns true if the parsing was successful.
    pub fn parse(&mut self) -> bool {
        /* @$.  */
        let mut yyloc: YYLoc;
        
    /* Lookahead token kind.  */
    let mut yychar: i32 = Self::YYEMPTY_;
    /* Lookahead symbol kind.  */
    let mut yytoken = &DYMMY_SYMBOL_KIND;

    /* State.  */
    let mut yyn: i32 = 0;
    let mut yylen: usize = 0;
    let mut yystate: i32 = 0;
    let mut yystack = YYStack::new();
    let mut label: i32 = Self::YYNEWSTATE;

    /* The location where the error started.  */
    let mut yyerrloc: YYLoc = YYLoc { begin: 0, end: 0 };

    /* Location. */
    let mut yylloc: YYLoc = YYLoc { begin: 0, end: 0 };

    /* Semantic value of the lookahead.  */
    let mut yylval: YYValue = YYValue::Uninitialized;

        self.yycdebug("Starting parse");
        self.yyerrstatus_ = 0;
        self.yynerrs = 0;

        /* Initialize the stack.  */
        yystack.push(yystate, yylval.clone(), yylloc);

        loop {
            match label {
                // New state.  Unlike in the C/C++ skeletons, the state is already
                // pushed when we come here.

                Self::YYNEWSTATE => {
                    if false {
                        self.yycdebug(&format!("Entering state {}", yystate));
                        eprintln!("{}", yystack);
                    }

                    /* Accept? */
                    if yystate == Self::YYFINAL_ {
                        return true;
                    }

                    /* Take a decision.  First try without lookahead.  */
                    yyn = Self::yypact_[i32_to_usize(yystate)];
                    if yy_pact_value_is_default(yyn) {
                        label = Self::YYDEFAULT;
                        continue;
                    }

                    /* Read a lookahead token.  */
                    if yychar == Self::YYEMPTY_ {
                        self.yycdebug("Reading a token");
                        let token = self.next_token();
                        yychar = token.token_type;
                        yylloc = token.loc;
                        yylval = YYValue::from_token(token);
                    }

                    /* Convert token to internal form.  */
                    yytoken = Self::yytranslate_(yychar);
                    self.yy_symbol_print("Next token is", yytoken, &yylval, &yylloc);

                    if yytoken == SymbolKind::get(1) {
                        // The scanner already issued an error message, process directly
                        // to error recovery.  But do not keep the error token as
                        // lookahead, it is too special and may lead us to an endless
                        // loop in error recovery. */
                        yychar = Lexer::YYUNDEF;
                        yytoken = SymbolKind::get(2);
                        yyerrloc = yylloc;
                        label = Self::YYERRLAB1;
                    } else {
                        // If the proper action on seeing token YYTOKEN is to reduce or to
                        // detect an error, take that action.
                        yyn += yytoken.code();
                        if yyn < 0 || Self::YYLAST_ < yyn || Self::yycheck_[i32_to_usize(yyn)] != yytoken.code() {
                            label = Self::YYDEFAULT;
                        }

                        /* <= 0 means reduce or error.  */
                        else {
                            yyn = Self::yytable_[i32_to_usize(yyn)];
                            if yyn <= 0 {
                                if yy_table_value_is_error(yyn) {
                                    label = Self::YYERRLAB;
                                } else {
                                    yyn = -yyn;
                                    label = Self::YYREDUCE;
                                }
                            } else {
                                /* Shift the lookahead token.  */
                                self.yy_symbol_print("Shifting", yytoken, &yylval, &yylloc);

                                /* Discard the token being shifted.  */
                                yychar = Self::YYEMPTY_;

                                /* Count tokens shifted since error; after three, turn off error status.  */
                                if self.yyerrstatus_ > 0 {
                                    self.yyerrstatus_ -= 1;
                                }

                                yystate = yyn;
                                yystack.push(yystate, std::mem::take(&mut yylval), std::mem::take(&mut yylloc));
                                label = Self::YYNEWSTATE;
                            }
                        }
                    }
                    continue;
                }, // YYNEWSTATE

                // yydefault -- do the default action for the current state.
                Self::YYDEFAULT => {
                    yyn = Self::yydefact_[i32_to_usize(yystate)];
                    if yyn == 0 {
                        label = Self::YYERRLAB;
                    } else {
                        label = Self::YYREDUCE;
                    }
                    continue;
                } // YYDEFAULT

                // yyreduce -- Do a reduction.
                Self::YYREDUCE => {
                    yylen = i32_to_usize(Self::yyr2_[i32_to_usize(yyn)]);
                    label = match self.yyaction(yyn, &mut yystack, &mut yylen) {
                        Ok(label) => label,
                        Err(_) => Self::YYERROR
                    };
                    yystate = yystack.state_at(0);
                    continue;
                }, // YYREDUCE

                // yyerrlab -- here on detecting error
                Self::YYERRLAB => {
                    /* If not already recovering from an error, report this error.  */
                    if self.yyerrstatus_ == 0 {
                        self.yynerrs += 1;
                        if yychar == Self::YYEMPTY_ {
                            yytoken = &DYMMY_SYMBOL_KIND;
                        }
                        self.report_syntax_error(&yystack, yytoken, yylloc);
                    }
                    yyerrloc = yylloc;
                    if self.yyerrstatus_ == 3 {
                        // If just tried and failed to reuse lookahead token after an error, discard it.

                        if yychar <= Lexer::YYEOF {
                            /* Return failure if at end of input.  */
                            if yychar == Lexer::YYEOF {
                                return false;
                            }
                        }
                        else {
                            yychar = Self::YYEMPTY_;
                        }
                    }

                    // Else will try to reuse lookahead token after shifting the error token.
                    label = Self::YYERRLAB1;
                    continue;
                }, // YYERRLAB

                // errorlab -- error raised explicitly by YYERROR.
                Self::YYERROR => {
                    /* Do not reclaim the symbols of the rule which action triggered
                    this YYERROR.  */
                    yystack.pop_n(yylen);
                    yylen = 0;
                    yystate = yystack.state_at(0);
                    label = Self::YYERRLAB1;
                    continue;
                }, // YYERROR

                // yyerrlab1 -- common code for both syntax error and YYERROR.
                Self::YYERRLAB1 => {
                    self.yyerrstatus_ = 3;       /* Each real token shifted decrements this.  */

                    // Pop stack until we find a state that shifts the error token.
                    loop {
                        yyn = Self::yypact_[i32_to_usize(yystate)];
                        if !yy_pact_value_is_default(yyn) {
                            yyn += SymbolKind { value: SymbolKind::S_YYerror }.code();
                            if (0..=Self::YYLAST_).contains(&yyn) {
                                let yyn_usize = i32_to_usize(yyn);
                                if Self::yycheck_[yyn_usize] == SymbolKind::S_YYerror {
                                    yyn = Self::yytable_[yyn_usize];
                                    if 0 < yyn {
                                        break;
                                    }
                                }
                            }
                        }

                        // Pop the current state because it cannot handle the error token.
                        if yystack.len() == 1 {
                            return false;
                        }

                        yyerrloc = *yystack.location_at(0);
                        yystack.pop();
                        yystate = yystack.state_at(0);
                        if false {
                            eprintln!("{}", yystack);
                        }
                    }

                    if label == Self::YYABORT {
                        /* Leave the switch.  */
                        continue;
                    }

                    /* Muck with the stack to setup for yylloc.  */
                    yystack.push(0, YYValue::Uninitialized, yylloc);
                    yystack.push(0, YYValue::Uninitialized, yyerrloc);
                    yyloc = make_yylloc(&yystack, 2);
                    yystack.pop_n(2);

                    /* Shift the error token.  */
                    self.yy_symbol_print("Shifting", SymbolKind::get(Self::yystos_[i32_to_usize(yyn)]), &yylval, &yyloc);

                    yystate = yyn;
                    yystack.push(yyn, yylval.clone(), yyloc);
                    label = Self::YYNEWSTATE;
                    continue;
                }, // YYERRLAB1

                // Accept
                Self::YYACCEPT => {
                    return true;
                }

                // Abort.
                Self::YYABORT => {
                    return false;
                },

                _ => {
                    panic!("internal bison error: unknown label {}", label);
                }
            }
        }
    }
}

// Whether the given `yypact_` value indicates a defaulted state.
fn yy_pact_value_is_default(yyvalue: i32) -> bool {
    yyvalue == YYPACT_NINF_
}

// Whether the given `yytable_`
// value indicates a syntax error.
// yyvalue: the value to check
fn yy_table_value_is_error(yyvalue: i32) -> bool {
    yyvalue == YYTABLE_NINF_
}

const YYPACT_NINF_: i32 = -16;
const YYTABLE_NINF_: i32 = -1;

impl Parser {

/* YYPACT[STATE-NUM] -- Index in YYTABLE of the portion describing
   STATE-NUM.  */
  #[allow(non_upper_case_globals)]
const yypact_: &'static [i32] = &[    -15,   -12,     4,    -8,   -16,   -16,     0,   -16,   -15,   -12,
     -16,    -7,   -16,    -2,     1,   -12,   -12,   -12,   -16,   -16,
       3,   -16,     3,   -12,   -16 ];

/* YYDEFACT[STATE-NUM] -- Default reduction number in state STATE-NUM.
   Performed when YYTABLE does not specify something else to do.  Zero
   means the default is an error.  */
  #[allow(non_upper_case_globals)]
const yydefact_: &'static [i32] = &[      0,     0,     0,     2,     3,     9,     0,     1,     0,     0,
       4,     0,     6,     0,     0,     0,     0,     0,     7,    12,
       8,    10,     5,     0,    11 ];

/* YYPGOTO[NTERM-NUM].  */
  #[allow(non_upper_case_globals)]
const yypgoto_: &'static [i32] = &[    -16,   -16,   -16,     2,   -16,    -4,    -1,     5,   -10 ];

/* YYDEFGOTO[NTERM-NUM].  */
  #[allow(non_upper_case_globals)]
const yydefgoto_: &'static [i32] = &[      0,     2,     3,     4,    11,    12,    19,    20,    21 ];

/* YYTABLE[YYPACT[STATE-NUM]] -- What to do in state STATE-NUM.  If
   positive, shift that token.  If negative, reduce the rule whose
   number is the opposite.  If YYTABLE_NINF, syntax error.  */
  #[allow(non_upper_case_globals)]
const yytable_: &'static [i32] = &[      6,    14,     1,     5,     7,     8,    15,     9,    13,    16,
      10,    18,    17,    24,    13,    23,     0,     0,     0,     0,
       0,     0,    22 ];

#[allow(non_upper_case_globals)]
const yycheck_: &'static [i32] = &[      1,     8,    17,    15,     0,    13,    13,     7,     9,    11,
       8,    15,    11,    23,    15,    12,    -1,    -1,    -1,    -1,
      -1,    -1,    17 ];

/* YYSTOS[STATE-NUM] -- The symbol kind of the accessing symbol of
   state STATE-NUM.  */
  #[allow(non_upper_case_globals)]
const yystos_: &'static [i32] = &[      0,    17,    22,    23,    24,    15,    27,     0,    13,     7,
      24,    25,    26,    27,     8,    13,    11,    11,    26,    27,
      28,    29,    28,    12,    29 ];

/* YYR1[RULE-NUM] -- Symbol kind of the left-hand side of rule RULE-NUM.  */
  #[allow(non_upper_case_globals)]
const yyr1_: &'static [i32] = &[      0,    21,    22,    23,    23,    24,    25,    25,    26,    27,
      28,    28,    29 ];

/* YYR2[RULE-NUM] -- Number of symbols on the right-hand side of rule RULE-NUM.  */
  #[allow(non_upper_case_globals)]
const yyr2_: &'static [i32] = &[      0,     2,     1,     1,     3,     7,     1,     3,     3,     1,
       1,     3,     1 ];


/* YYRLINE[YYN] -- Source line where rule number YYN was defined.  */
  #[allow(non_upper_case_globals)]
const yyrline_: &'static [i32] = &[      0,    58,    58,    64,    67,    74,    77,    80,    86,    89,
      93,    96,   102 ];


  // Report on the debug stream that the rule yyrule is going to be reduced.
  fn yy_reduce_print(&self, yyrule: i32, yystack: &YYStack) {
        if !(false) {
            return;
        }

        let yylno = Self::yyrline_[i32_to_usize(yyrule)];
        let yynrhs = Self::yyr2_[i32_to_usize(yyrule)];
        // Print the symbols being reduced, and their result.
        self.yycdebug(&format!("Reducing stack by rule {} (line {}):", /* " fix */ yyrule - 1, yylno));

        // The symbols being reduced.
        for yyi in 0..yynrhs {
            let state: usize = i32_to_usize(yystack.state_at(i32_to_usize(yynrhs - (yyi + 1))));
            self.yy_symbol_print(
                &format!("   ${} =", yyi + 1),
                SymbolKind::get(Self::yystos_[state]),
                yystack.borrow_value_at(i32_to_usize(yynrhs - (yyi + 1))),
                yystack.location_at(i32_to_usize(yynrhs - (yyi + 1)))
            );
        }
  }

  /* YYTRANSLATE_(TOKEN-NUM) -- Symbol number corresponding to TOKEN-NUM
     as returned by yylex, with out-of-bounds checking.  */
  fn yytranslate_(t: i32) -> &'static SymbolKind
  {
        // Last valid token kind.
        let code_max: i32 = 275;
        if t <= 0 {
            SymbolKind::get(0)
        } else if t <= code_max {
            let t = i32_to_usize(t);
            SymbolKind::get(Self::yytranslate_table_[t])
        } else {
            SymbolKind::get(2)
        }
  }
  #[allow(non_upper_case_globals)]
const yytranslate_table_: &'static [i32] = &[      0,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     1,     2,     3,     4,
       5,     6,     7,     8,     9,    10,    11,    12,    13,    14,
      15,    16,    17,    18,    19,    20 ];


const YYLAST_: i32 = 22;
const YYEMPTY_: i32 = -2;
const YYFINAL_: i32 = 7;
const YYNTOKENS_: i32 = 21;


}



/* "src/parser.y":105  */


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
