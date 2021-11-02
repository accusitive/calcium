#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use lalrpop_util::lalrpop_mod;
mod ast {
    pub struct Function {
        name: String,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Function {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Function {
                    name: ref __self_0_0,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "Function");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "name",
                        &&(*__self_0_0),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    impl Function {
        pub fn new<S: Into<String>>(name: S) -> Self {
            Self { name: name.into() }
        }
    }
    pub struct FunctionArg {
        pub ty: String,
        pub ident: String,
    }
}
pub mod gram {
    use std::str::FromStr;
    use crate::ast::{Function, FunctionArg};
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate core;
    extern crate alloc;
    mod __parse__Function {
        #![allow(
            non_snake_case,
            non_camel_case_types,
            unused_mut,
            unused_variables,
            unused_imports,
            unused_parens
        )]
        use std::str::FromStr;
        use crate::ast::{Function, FunctionArg};
        #[allow(unused_extern_crates)]
        extern crate lalrpop_util as __lalrpop_util;
        #[allow(unused_imports)]
        use self::__lalrpop_util::state_machine as __state_machine;
        extern crate core;
        extern crate alloc;
        use self::__lalrpop_util::lexer::Token;
        #[allow(dead_code)]
        pub(crate) enum __Symbol<'input> {
            Variant0(&'input str),
            Variant1(FunctionArg),
            Variant2(alloc::vec::Vec<FunctionArg>),
            Variant3(Box<Function>),
            Variant4(String),
        }
        const __ACTION: &[i8] = &[
            0, 0, 0, 4, 0, 0, 0, 0, 5, 0, 0, 0, -11, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0,
        ];
        fn __action(state: i8, integer: usize) -> i8 {
            __ACTION[(state as usize) * 4 + integer]
        }
        const __EOF_ACTION: &[i8] = &[0, -12, 0, 0, 0, -6];
        fn __goto(state: i8, nt: usize) -> i8 {
            match nt {
                3 => 1,
                6 => 2,
                _ => 0,
            }
        }
        fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
            const __TERMINAL: &[&str] = &[
                r###""(""###,
                r###"")""###,
                r###"",""###,
                r###"r#"[A-z]*"#"###,
            ];
            __TERMINAL
                .iter()
                .enumerate()
                .filter_map(|(index, terminal)| {
                    let next_state = __action(__state, index);
                    if next_state == 0 {
                        None
                    } else {
                        Some(alloc::string::ToString::to_string(terminal))
                    }
                })
                .collect()
        }
        pub(crate) struct __StateMachine<'input> {
            input: &'input str,
            __phantom: core::marker::PhantomData<(&'input ())>,
        }
        impl<'input> __state_machine::ParserDefinition for __StateMachine<'input> {
            type Location = usize;
            type Error = &'static str;
            type Token = Token<'input>;
            type TokenIndex = usize;
            type Symbol = __Symbol<'input>;
            type Success = Box<Function>;
            type StateIndex = i8;
            type Action = i8;
            type ReduceIndex = i8;
            type NonterminalIndex = usize;
            #[inline]
            fn start_location(&self) -> Self::Location {
                Default::default()
            }
            #[inline]
            fn start_state(&self) -> Self::StateIndex {
                0
            }
            #[inline]
            fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
                __token_to_integer(token, core::marker::PhantomData::<(&())>)
            }
            #[inline]
            fn action(&self, state: i8, integer: usize) -> i8 {
                __action(state, integer)
            }
            #[inline]
            fn error_action(&self, state: i8) -> i8 {
                __action(state, 4 - 1)
            }
            #[inline]
            fn eof_action(&self, state: i8) -> i8 {
                __EOF_ACTION[state as usize]
            }
            #[inline]
            fn goto(&self, state: i8, nt: usize) -> i8 {
                __goto(state, nt)
            }
            fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
                __token_to_symbol(token_index, token, core::marker::PhantomData::<(&())>)
            }
            fn expected_tokens(&self, state: i8) -> alloc::vec::Vec<alloc::string::String> {
                __expected_tokens(state)
            }
            #[inline]
            fn uses_error_recovery(&self) -> bool {
                false
            }
            #[inline]
            fn error_recovery_symbol(
                &self,
                recovery: __state_machine::ErrorRecovery<Self>,
            ) -> Self::Symbol {
                ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                    &["error recovery not enabled for this grammar"],
                    &match () {
                        () => [],
                    },
                ))
            }
            fn reduce(
                &mut self,
                action: i8,
                start_location: Option<&Self::Location>,
                states: &mut alloc::vec::Vec<i8>,
                symbols: &mut alloc::vec::Vec<__state_machine::SymbolTriple<Self>>,
            ) -> Option<__state_machine::ParseResult<Self>> {
                __reduce(
                    self.input,
                    action,
                    start_location,
                    states,
                    symbols,
                    core::marker::PhantomData::<(&())>,
                )
            }
            fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
                ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                    &["error recovery not enabled for this grammar"],
                    &match () {
                        () => [],
                    },
                ))
            }
        }
        fn __token_to_integer<'input>(
            __token: &Token<'input>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> Option<usize> {
            match *__token {
                Token(1, _) if true => Some(0),
                Token(2, _) if true => Some(1),
                Token(3, _) if true => Some(2),
                Token(0, _) if true => Some(3),
                _ => None,
            }
        }
        fn __token_to_symbol<'input>(
            __token_index: usize,
            __token: Token<'input>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> __Symbol<'input> {
            match __token_index {
                0 | 1 | 2 | 3 => match __token {
                    Token(1, __tok0) | Token(2, __tok0) | Token(3, __tok0) | Token(0, __tok0)
                        if true =>
                    {
                        __Symbol::Variant0(__tok0)
                    }
                    _ => ::core::panicking::panic("internal error: entered unreachable code"),
                },
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            }
        }
        pub struct FunctionParser {
            builder: __lalrpop_util::lexer::MatcherBuilder,
            _priv: (),
        }
        impl FunctionParser {
            pub fn new() -> FunctionParser {
                let __builder = super::__intern_token::new_builder();
                FunctionParser {
                    builder: __builder,
                    _priv: (),
                }
            }
            #[allow(dead_code)]
            pub fn parse<'input>(
                &self,
                input: &'input str,
            ) -> Result<Box<Function>, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
            {
                let mut __tokens = self.builder.matcher(input);
                __state_machine::Parser::drive(
                    __StateMachine {
                        input,
                        __phantom: core::marker::PhantomData::<(&())>,
                    },
                    __tokens,
                )
            }
        }
        pub(crate) fn __reduce<'input>(
            input: &'input str,
            __action: i8,
            __lookahead_start: Option<&usize>,
            __states: &mut alloc::vec::Vec<i8>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> Option<
            Result<Box<Function>, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>,
        > {
            let (__pop_states, __nonterminal) = match __action {
                0 => __reduce0(
                    input,
                    __lookahead_start,
                    __symbols,
                    core::marker::PhantomData::<(&())>,
                ),
                1 => __reduce1(
                    input,
                    __lookahead_start,
                    __symbols,
                    core::marker::PhantomData::<(&())>,
                ),
                2 => __reduce2(
                    input,
                    __lookahead_start,
                    __symbols,
                    core::marker::PhantomData::<(&())>,
                ),
                3 => __reduce3(
                    input,
                    __lookahead_start,
                    __symbols,
                    core::marker::PhantomData::<(&())>,
                ),
                4 => __reduce4(
                    input,
                    __lookahead_start,
                    __symbols,
                    core::marker::PhantomData::<(&())>,
                ),
                5 => __reduce5(
                    input,
                    __lookahead_start,
                    __symbols,
                    core::marker::PhantomData::<(&())>,
                ),
                6 => __reduce6(
                    input,
                    __lookahead_start,
                    __symbols,
                    core::marker::PhantomData::<(&())>,
                ),
                7 => __reduce7(
                    input,
                    __lookahead_start,
                    __symbols,
                    core::marker::PhantomData::<(&())>,
                ),
                8 => __reduce8(
                    input,
                    __lookahead_start,
                    __symbols,
                    core::marker::PhantomData::<(&())>,
                ),
                9 => __reduce9(
                    input,
                    __lookahead_start,
                    __symbols,
                    core::marker::PhantomData::<(&())>,
                ),
                10 => __reduce10(
                    input,
                    __lookahead_start,
                    __symbols,
                    core::marker::PhantomData::<(&())>,
                ),
                11 => {
                    let __sym0 = __pop_Variant3(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action0(input, __sym0);
                    return Some(Ok(__nt));
                }
                12 => __reduce12(
                    input,
                    __lookahead_start,
                    __symbols,
                    core::marker::PhantomData::<(&())>,
                ),
                13 => __reduce13(
                    input,
                    __lookahead_start,
                    __symbols,
                    core::marker::PhantomData::<(&())>,
                ),
                14 => __reduce14(
                    input,
                    __lookahead_start,
                    __symbols,
                    core::marker::PhantomData::<(&())>,
                ),
                _ => ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                    &["invalid action code "],
                    &match (&__action,) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                )),
            };
            let __states_len = __states.len();
            __states.truncate(__states_len - __pop_states);
            let __state = *__states.last().unwrap();
            let __next_state = __goto(__state, __nonterminal);
            __states.push(__next_state);
            None
        }
        #[inline(never)]
        fn __symbol_type_mismatch() -> ! {
            ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                &["symbol type mismatch"],
                &match () {
                    () => [],
                },
            ))
        }
        fn __pop_Variant3<'input>(
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        ) -> (usize, Box<Function>, usize) {
            match __symbols.pop() {
                Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
                _ => __symbol_type_mismatch(),
            }
        }
        fn __pop_Variant1<'input>(
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        ) -> (usize, FunctionArg, usize) {
            match __symbols.pop() {
                Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
                _ => __symbol_type_mismatch(),
            }
        }
        fn __pop_Variant4<'input>(
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        ) -> (usize, String, usize) {
            match __symbols.pop() {
                Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
                _ => __symbol_type_mismatch(),
            }
        }
        fn __pop_Variant2<'input>(
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        ) -> (usize, alloc::vec::Vec<FunctionArg>, usize) {
            match __symbols.pop() {
                Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
                _ => __symbol_type_mismatch(),
            }
        }
        fn __pop_Variant0<'input>(
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        ) -> (usize, &'input str, usize) {
            match __symbols.pop() {
                Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
                _ => __symbol_type_mismatch(),
            }
        }
        pub(crate) fn __reduce0<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            if !(__symbols.len() >= 2) {
                ::core::panicking::panic("assertion failed: __symbols.len() >= 2")
            };
            let __sym1 = __pop_Variant0(__symbols);
            let __sym0 = __pop_Variant1(__symbols);
            let __start = __sym0.0.clone();
            let __end = __sym1.2.clone();
            let __nt = super::__action11(input, __sym0, __sym1);
            __symbols.push((__start, __Symbol::Variant1(__nt), __end));
            (2, 0)
        }
        pub(crate) fn __reduce1<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            let __start = __lookahead_start
                .cloned()
                .or_else(|| __symbols.last().map(|s| s.2.clone()))
                .unwrap_or_default();
            let __end = __start.clone();
            let __nt = super::__action9(input, &__start, &__end);
            __symbols.push((__start, __Symbol::Variant2(__nt), __end));
            (0, 1)
        }
        pub(crate) fn __reduce2<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            let __sym0 = __pop_Variant2(__symbols);
            let __start = __sym0.0.clone();
            let __end = __sym0.2.clone();
            let __nt = super::__action10(input, __sym0);
            __symbols.push((__start, __Symbol::Variant2(__nt), __end));
            (1, 1)
        }
        pub(crate) fn __reduce3<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            if !(__symbols.len() >= 2) {
                ::core::panicking::panic("assertion failed: __symbols.len() >= 2")
            };
            let __sym1 = __pop_Variant0(__symbols);
            let __sym0 = __pop_Variant1(__symbols);
            let __start = __sym0.0.clone();
            let __end = __sym1.2.clone();
            let __nt = super::__action14(input, __sym0, __sym1);
            __symbols.push((__start, __Symbol::Variant2(__nt), __end));
            (2, 2)
        }
        pub(crate) fn __reduce4<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            if !(__symbols.len() >= 3) {
                ::core::panicking::panic("assertion failed: __symbols.len() >= 3")
            };
            let __sym2 = __pop_Variant0(__symbols);
            let __sym1 = __pop_Variant1(__symbols);
            let __sym0 = __pop_Variant2(__symbols);
            let __start = __sym0.0.clone();
            let __end = __sym2.2.clone();
            let __nt = super::__action15(input, __sym0, __sym1, __sym2);
            __symbols.push((__start, __Symbol::Variant2(__nt), __end));
            (3, 2)
        }
        pub(crate) fn __reduce5<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            if !(__symbols.len() >= 3) {
                ::core::panicking::panic("assertion failed: __symbols.len() >= 3")
            };
            let __sym2 = __pop_Variant0(__symbols);
            let __sym1 = __pop_Variant0(__symbols);
            let __sym0 = __pop_Variant4(__symbols);
            let __start = __sym0.0.clone();
            let __end = __sym2.2.clone();
            let __nt = super::__action4(input, __sym0, __sym1, __sym2);
            __symbols.push((__start, __Symbol::Variant3(__nt), __end));
            (3, 3)
        }
        pub(crate) fn __reduce6<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            if !(__symbols.len() >= 2) {
                ::core::panicking::panic("assertion failed: __symbols.len() >= 2")
            };
            let __sym1 = __pop_Variant4(__symbols);
            let __sym0 = __pop_Variant4(__symbols);
            let __start = __sym0.0.clone();
            let __end = __sym1.2.clone();
            let __nt = super::__action7(input, __sym0, __sym1);
            __symbols.push((__start, __Symbol::Variant1(__nt), __end));
            (2, 4)
        }
        pub(crate) fn __reduce7<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            let __sym0 = __pop_Variant1(__symbols);
            let __start = __sym0.0.clone();
            let __end = __sym0.2.clone();
            let __nt = super::__action5(input, __sym0);
            __symbols.push((__start, __Symbol::Variant2(__nt), __end));
            (1, 5)
        }
        pub(crate) fn __reduce8<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            let __start = __lookahead_start
                .cloned()
                .or_else(|| __symbols.last().map(|s| s.2.clone()))
                .unwrap_or_default();
            let __end = __start.clone();
            let __nt = super::__action16(input, &__start, &__end);
            __symbols.push((__start, __Symbol::Variant2(__nt), __end));
            (0, 5)
        }
        pub(crate) fn __reduce9<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            let __sym0 = __pop_Variant2(__symbols);
            let __start = __sym0.0.clone();
            let __end = __sym0.2.clone();
            let __nt = super::__action17(input, __sym0);
            __symbols.push((__start, __Symbol::Variant2(__nt), __end));
            (1, 5)
        }
        pub(crate) fn __reduce10<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            let __sym0 = __pop_Variant0(__symbols);
            let __start = __sym0.0.clone();
            let __end = __sym0.2.clone();
            let __nt = super::__action8(input, __sym0);
            __symbols.push((__start, __Symbol::Variant4(__nt), __end));
            (1, 6)
        }
        pub(crate) fn __reduce12<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            let __sym0 = __pop_Variant1(__symbols);
            let __start = __sym0.0.clone();
            let __end = __sym0.2.clone();
            let __nt = super::__action2(input, __sym0);
            __symbols.push((__start, __Symbol::Variant1(__nt), __end));
            (1, 8)
        }
        pub(crate) fn __reduce13<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            let __sym0 = __pop_Variant2(__symbols);
            let __start = __sym0.0.clone();
            let __end = __sym0.2.clone();
            let __nt = super::__action1(input, __sym0);
            __symbols.push((__start, __Symbol::Variant2(__nt), __end));
            (1, 9)
        }
        pub(crate) fn __reduce14<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            let __sym0 = __pop_Variant4(__symbols);
            let __start = __sym0.0.clone();
            let __end = __sym0.2.clone();
            let __nt = super::__action3(input, __sym0);
            __symbols.push((__start, __Symbol::Variant4(__nt), __end));
            (1, 10)
        }
    }
    pub use self::__parse__Function::FunctionParser;
    mod __parse__FunctionArg {
        #![allow(
            non_snake_case,
            non_camel_case_types,
            unused_mut,
            unused_variables,
            unused_imports,
            unused_parens
        )]
        use std::str::FromStr;
        use crate::ast::{Function, FunctionArg};
        #[allow(unused_extern_crates)]
        extern crate lalrpop_util as __lalrpop_util;
        #[allow(unused_imports)]
        use self::__lalrpop_util::state_machine as __state_machine;
        extern crate core;
        extern crate alloc;
        use self::__lalrpop_util::lexer::Token;
        #[allow(dead_code)]
        pub(crate) enum __Symbol<'input> {
            Variant0(&'input str),
            Variant1(FunctionArg),
            Variant2(alloc::vec::Vec<FunctionArg>),
            Variant3(Box<Function>),
            Variant4(String),
        }
        const __ACTION: &[i8] = &[0, 0, 0, 4, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, -11, 0, 0, 0, 0];
        fn __action(state: i8, integer: usize) -> i8 {
            __ACTION[(state as usize) * 4 + integer]
        }
        const __EOF_ACTION: &[i8] = &[0, 0, -13, -11, -7];
        fn __goto(state: i8, nt: usize) -> i8 {
            match nt {
                4 => 2,
                6 => match state {
                    1 => 4,
                    _ => 1,
                },
                _ => 0,
            }
        }
        fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
            const __TERMINAL: &[&str] = &[
                r###""(""###,
                r###"")""###,
                r###"",""###,
                r###"r#"[A-z]*"#"###,
            ];
            __TERMINAL
                .iter()
                .enumerate()
                .filter_map(|(index, terminal)| {
                    let next_state = __action(__state, index);
                    if next_state == 0 {
                        None
                    } else {
                        Some(alloc::string::ToString::to_string(terminal))
                    }
                })
                .collect()
        }
        pub(crate) struct __StateMachine<'input> {
            input: &'input str,
            __phantom: core::marker::PhantomData<(&'input ())>,
        }
        impl<'input> __state_machine::ParserDefinition for __StateMachine<'input> {
            type Location = usize;
            type Error = &'static str;
            type Token = Token<'input>;
            type TokenIndex = usize;
            type Symbol = __Symbol<'input>;
            type Success = FunctionArg;
            type StateIndex = i8;
            type Action = i8;
            type ReduceIndex = i8;
            type NonterminalIndex = usize;
            #[inline]
            fn start_location(&self) -> Self::Location {
                Default::default()
            }
            #[inline]
            fn start_state(&self) -> Self::StateIndex {
                0
            }
            #[inline]
            fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
                __token_to_integer(token, core::marker::PhantomData::<(&())>)
            }
            #[inline]
            fn action(&self, state: i8, integer: usize) -> i8 {
                __action(state, integer)
            }
            #[inline]
            fn error_action(&self, state: i8) -> i8 {
                __action(state, 4 - 1)
            }
            #[inline]
            fn eof_action(&self, state: i8) -> i8 {
                __EOF_ACTION[state as usize]
            }
            #[inline]
            fn goto(&self, state: i8, nt: usize) -> i8 {
                __goto(state, nt)
            }
            fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
                __token_to_symbol(token_index, token, core::marker::PhantomData::<(&())>)
            }
            fn expected_tokens(&self, state: i8) -> alloc::vec::Vec<alloc::string::String> {
                __expected_tokens(state)
            }
            #[inline]
            fn uses_error_recovery(&self) -> bool {
                false
            }
            #[inline]
            fn error_recovery_symbol(
                &self,
                recovery: __state_machine::ErrorRecovery<Self>,
            ) -> Self::Symbol {
                ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                    &["error recovery not enabled for this grammar"],
                    &match () {
                        () => [],
                    },
                ))
            }
            fn reduce(
                &mut self,
                action: i8,
                start_location: Option<&Self::Location>,
                states: &mut alloc::vec::Vec<i8>,
                symbols: &mut alloc::vec::Vec<__state_machine::SymbolTriple<Self>>,
            ) -> Option<__state_machine::ParseResult<Self>> {
                __reduce(
                    self.input,
                    action,
                    start_location,
                    states,
                    symbols,
                    core::marker::PhantomData::<(&())>,
                )
            }
            fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
                ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                    &["error recovery not enabled for this grammar"],
                    &match () {
                        () => [],
                    },
                ))
            }
        }
        fn __token_to_integer<'input>(
            __token: &Token<'input>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> Option<usize> {
            match *__token {
                Token(1, _) if true => Some(0),
                Token(2, _) if true => Some(1),
                Token(3, _) if true => Some(2),
                Token(0, _) if true => Some(3),
                _ => None,
            }
        }
        fn __token_to_symbol<'input>(
            __token_index: usize,
            __token: Token<'input>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> __Symbol<'input> {
            match __token_index {
                0 | 1 | 2 | 3 => match __token {
                    Token(1, __tok0) | Token(2, __tok0) | Token(3, __tok0) | Token(0, __tok0)
                        if true =>
                    {
                        __Symbol::Variant0(__tok0)
                    }
                    _ => ::core::panicking::panic("internal error: entered unreachable code"),
                },
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            }
        }
        pub struct FunctionArgParser {
            builder: __lalrpop_util::lexer::MatcherBuilder,
            _priv: (),
        }
        impl FunctionArgParser {
            pub fn new() -> FunctionArgParser {
                let __builder = super::__intern_token::new_builder();
                FunctionArgParser {
                    builder: __builder,
                    _priv: (),
                }
            }
            #[allow(dead_code)]
            pub fn parse<'input>(
                &self,
                input: &'input str,
            ) -> Result<FunctionArg, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
            {
                let mut __tokens = self.builder.matcher(input);
                __state_machine::Parser::drive(
                    __StateMachine {
                        input,
                        __phantom: core::marker::PhantomData::<(&())>,
                    },
                    __tokens,
                )
            }
        }
        pub(crate) fn __reduce<'input>(
            input: &'input str,
            __action: i8,
            __lookahead_start: Option<&usize>,
            __states: &mut alloc::vec::Vec<i8>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> Option<
            Result<FunctionArg, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>,
        > {
            let (__pop_states, __nonterminal) = match __action {
                0 => __reduce0(
                    input,
                    __lookahead_start,
                    __symbols,
                    core::marker::PhantomData::<(&())>,
                ),
                1 => __reduce1(
                    input,
                    __lookahead_start,
                    __symbols,
                    core::marker::PhantomData::<(&())>,
                ),
                2 => __reduce2(
                    input,
                    __lookahead_start,
                    __symbols,
                    core::marker::PhantomData::<(&())>,
                ),
                3 => __reduce3(
                    input,
                    __lookahead_start,
                    __symbols,
                    core::marker::PhantomData::<(&())>,
                ),
                4 => __reduce4(
                    input,
                    __lookahead_start,
                    __symbols,
                    core::marker::PhantomData::<(&())>,
                ),
                5 => __reduce5(
                    input,
                    __lookahead_start,
                    __symbols,
                    core::marker::PhantomData::<(&())>,
                ),
                6 => __reduce6(
                    input,
                    __lookahead_start,
                    __symbols,
                    core::marker::PhantomData::<(&())>,
                ),
                7 => __reduce7(
                    input,
                    __lookahead_start,
                    __symbols,
                    core::marker::PhantomData::<(&())>,
                ),
                8 => __reduce8(
                    input,
                    __lookahead_start,
                    __symbols,
                    core::marker::PhantomData::<(&())>,
                ),
                9 => __reduce9(
                    input,
                    __lookahead_start,
                    __symbols,
                    core::marker::PhantomData::<(&())>,
                ),
                10 => __reduce10(
                    input,
                    __lookahead_start,
                    __symbols,
                    core::marker::PhantomData::<(&())>,
                ),
                11 => __reduce11(
                    input,
                    __lookahead_start,
                    __symbols,
                    core::marker::PhantomData::<(&())>,
                ),
                12 => {
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action2(input, __sym0);
                    return Some(Ok(__nt));
                }
                13 => __reduce13(
                    input,
                    __lookahead_start,
                    __symbols,
                    core::marker::PhantomData::<(&())>,
                ),
                14 => __reduce14(
                    input,
                    __lookahead_start,
                    __symbols,
                    core::marker::PhantomData::<(&())>,
                ),
                _ => ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                    &["invalid action code "],
                    &match (&__action,) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                )),
            };
            let __states_len = __states.len();
            __states.truncate(__states_len - __pop_states);
            let __state = *__states.last().unwrap();
            let __next_state = __goto(__state, __nonterminal);
            __states.push(__next_state);
            None
        }
        #[inline(never)]
        fn __symbol_type_mismatch() -> ! {
            ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                &["symbol type mismatch"],
                &match () {
                    () => [],
                },
            ))
        }
        fn __pop_Variant3<'input>(
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        ) -> (usize, Box<Function>, usize) {
            match __symbols.pop() {
                Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
                _ => __symbol_type_mismatch(),
            }
        }
        fn __pop_Variant1<'input>(
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        ) -> (usize, FunctionArg, usize) {
            match __symbols.pop() {
                Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
                _ => __symbol_type_mismatch(),
            }
        }
        fn __pop_Variant4<'input>(
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        ) -> (usize, String, usize) {
            match __symbols.pop() {
                Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
                _ => __symbol_type_mismatch(),
            }
        }
        fn __pop_Variant2<'input>(
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        ) -> (usize, alloc::vec::Vec<FunctionArg>, usize) {
            match __symbols.pop() {
                Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
                _ => __symbol_type_mismatch(),
            }
        }
        fn __pop_Variant0<'input>(
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        ) -> (usize, &'input str, usize) {
            match __symbols.pop() {
                Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
                _ => __symbol_type_mismatch(),
            }
        }
        pub(crate) fn __reduce0<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            if !(__symbols.len() >= 2) {
                ::core::panicking::panic("assertion failed: __symbols.len() >= 2")
            };
            let __sym1 = __pop_Variant0(__symbols);
            let __sym0 = __pop_Variant1(__symbols);
            let __start = __sym0.0.clone();
            let __end = __sym1.2.clone();
            let __nt = super::__action11(input, __sym0, __sym1);
            __symbols.push((__start, __Symbol::Variant1(__nt), __end));
            (2, 0)
        }
        pub(crate) fn __reduce1<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            let __start = __lookahead_start
                .cloned()
                .or_else(|| __symbols.last().map(|s| s.2.clone()))
                .unwrap_or_default();
            let __end = __start.clone();
            let __nt = super::__action9(input, &__start, &__end);
            __symbols.push((__start, __Symbol::Variant2(__nt), __end));
            (0, 1)
        }
        pub(crate) fn __reduce2<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            let __sym0 = __pop_Variant2(__symbols);
            let __start = __sym0.0.clone();
            let __end = __sym0.2.clone();
            let __nt = super::__action10(input, __sym0);
            __symbols.push((__start, __Symbol::Variant2(__nt), __end));
            (1, 1)
        }
        pub(crate) fn __reduce3<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            if !(__symbols.len() >= 2) {
                ::core::panicking::panic("assertion failed: __symbols.len() >= 2")
            };
            let __sym1 = __pop_Variant0(__symbols);
            let __sym0 = __pop_Variant1(__symbols);
            let __start = __sym0.0.clone();
            let __end = __sym1.2.clone();
            let __nt = super::__action14(input, __sym0, __sym1);
            __symbols.push((__start, __Symbol::Variant2(__nt), __end));
            (2, 2)
        }
        pub(crate) fn __reduce4<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            if !(__symbols.len() >= 3) {
                ::core::panicking::panic("assertion failed: __symbols.len() >= 3")
            };
            let __sym2 = __pop_Variant0(__symbols);
            let __sym1 = __pop_Variant1(__symbols);
            let __sym0 = __pop_Variant2(__symbols);
            let __start = __sym0.0.clone();
            let __end = __sym2.2.clone();
            let __nt = super::__action15(input, __sym0, __sym1, __sym2);
            __symbols.push((__start, __Symbol::Variant2(__nt), __end));
            (3, 2)
        }
        pub(crate) fn __reduce5<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            if !(__symbols.len() >= 3) {
                ::core::panicking::panic("assertion failed: __symbols.len() >= 3")
            };
            let __sym2 = __pop_Variant0(__symbols);
            let __sym1 = __pop_Variant0(__symbols);
            let __sym0 = __pop_Variant4(__symbols);
            let __start = __sym0.0.clone();
            let __end = __sym2.2.clone();
            let __nt = super::__action4(input, __sym0, __sym1, __sym2);
            __symbols.push((__start, __Symbol::Variant3(__nt), __end));
            (3, 3)
        }
        pub(crate) fn __reduce6<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            if !(__symbols.len() >= 2) {
                ::core::panicking::panic("assertion failed: __symbols.len() >= 2")
            };
            let __sym1 = __pop_Variant4(__symbols);
            let __sym0 = __pop_Variant4(__symbols);
            let __start = __sym0.0.clone();
            let __end = __sym1.2.clone();
            let __nt = super::__action7(input, __sym0, __sym1);
            __symbols.push((__start, __Symbol::Variant1(__nt), __end));
            (2, 4)
        }
        pub(crate) fn __reduce7<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            let __sym0 = __pop_Variant1(__symbols);
            let __start = __sym0.0.clone();
            let __end = __sym0.2.clone();
            let __nt = super::__action5(input, __sym0);
            __symbols.push((__start, __Symbol::Variant2(__nt), __end));
            (1, 5)
        }
        pub(crate) fn __reduce8<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            let __start = __lookahead_start
                .cloned()
                .or_else(|| __symbols.last().map(|s| s.2.clone()))
                .unwrap_or_default();
            let __end = __start.clone();
            let __nt = super::__action16(input, &__start, &__end);
            __symbols.push((__start, __Symbol::Variant2(__nt), __end));
            (0, 5)
        }
        pub(crate) fn __reduce9<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            let __sym0 = __pop_Variant2(__symbols);
            let __start = __sym0.0.clone();
            let __end = __sym0.2.clone();
            let __nt = super::__action17(input, __sym0);
            __symbols.push((__start, __Symbol::Variant2(__nt), __end));
            (1, 5)
        }
        pub(crate) fn __reduce10<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            let __sym0 = __pop_Variant0(__symbols);
            let __start = __sym0.0.clone();
            let __end = __sym0.2.clone();
            let __nt = super::__action8(input, __sym0);
            __symbols.push((__start, __Symbol::Variant4(__nt), __end));
            (1, 6)
        }
        pub(crate) fn __reduce11<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            let __sym0 = __pop_Variant3(__symbols);
            let __start = __sym0.0.clone();
            let __end = __sym0.2.clone();
            let __nt = super::__action0(input, __sym0);
            __symbols.push((__start, __Symbol::Variant3(__nt), __end));
            (1, 7)
        }
        pub(crate) fn __reduce13<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            let __sym0 = __pop_Variant2(__symbols);
            let __start = __sym0.0.clone();
            let __end = __sym0.2.clone();
            let __nt = super::__action1(input, __sym0);
            __symbols.push((__start, __Symbol::Variant2(__nt), __end));
            (1, 9)
        }
        pub(crate) fn __reduce14<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            let __sym0 = __pop_Variant4(__symbols);
            let __start = __sym0.0.clone();
            let __end = __sym0.2.clone();
            let __nt = super::__action3(input, __sym0);
            __symbols.push((__start, __Symbol::Variant4(__nt), __end));
            (1, 10)
        }
    }
    pub use self::__parse__FunctionArg::FunctionArgParser;
    mod __parse__FunctionArgs {
        #![allow(
            non_snake_case,
            non_camel_case_types,
            unused_mut,
            unused_variables,
            unused_imports,
            unused_parens
        )]
        use std::str::FromStr;
        use crate::ast::{Function, FunctionArg};
        #[allow(unused_extern_crates)]
        extern crate lalrpop_util as __lalrpop_util;
        #[allow(unused_imports)]
        use self::__lalrpop_util::state_machine as __state_machine;
        extern crate core;
        extern crate alloc;
        use self::__lalrpop_util::lexer::Token;
        #[allow(dead_code)]
        pub(crate) enum __Symbol<'input> {
            Variant0(&'input str),
            Variant1(FunctionArg),
            Variant2(alloc::vec::Vec<FunctionArg>),
            Variant3(Box<Function>),
            Variant4(String),
        }
        const __ACTION: &[i8] = &[
            0, 0, 0, 6, 0, 0, 0, 6, 0, 0, 0, 6, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, -11, -11, 0, 0, 10,
            0, 0, 0, 0, -4, 0, 0, -7, 0, 0, 0, 0, -5,
        ];
        fn __action(state: i8, integer: usize) -> i8 {
            __ACTION[(state as usize) * 4 + integer]
        }
        const __EOF_ACTION: &[i8] = &[-9, -10, 0, -8, -14, -11, 0, -4, -7, -5];
        fn __goto(state: i8, nt: usize) -> i8 {
            match nt {
                2 => 1,
                4 => match state {
                    1 => 6,
                    _ => 3,
                },
                5 => 4,
                6 => match state {
                    2 => 8,
                    _ => 2,
                },
                _ => 0,
            }
        }
        fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
            const __TERMINAL: &[&str] = &[
                r###""(""###,
                r###"")""###,
                r###"",""###,
                r###"r#"[A-z]*"#"###,
            ];
            __TERMINAL
                .iter()
                .enumerate()
                .filter_map(|(index, terminal)| {
                    let next_state = __action(__state, index);
                    if next_state == 0 {
                        None
                    } else {
                        Some(alloc::string::ToString::to_string(terminal))
                    }
                })
                .collect()
        }
        pub(crate) struct __StateMachine<'input> {
            input: &'input str,
            __phantom: core::marker::PhantomData<(&'input ())>,
        }
        impl<'input> __state_machine::ParserDefinition for __StateMachine<'input> {
            type Location = usize;
            type Error = &'static str;
            type Token = Token<'input>;
            type TokenIndex = usize;
            type Symbol = __Symbol<'input>;
            type Success = alloc::vec::Vec<FunctionArg>;
            type StateIndex = i8;
            type Action = i8;
            type ReduceIndex = i8;
            type NonterminalIndex = usize;
            #[inline]
            fn start_location(&self) -> Self::Location {
                Default::default()
            }
            #[inline]
            fn start_state(&self) -> Self::StateIndex {
                0
            }
            #[inline]
            fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
                __token_to_integer(token, core::marker::PhantomData::<(&())>)
            }
            #[inline]
            fn action(&self, state: i8, integer: usize) -> i8 {
                __action(state, integer)
            }
            #[inline]
            fn error_action(&self, state: i8) -> i8 {
                __action(state, 4 - 1)
            }
            #[inline]
            fn eof_action(&self, state: i8) -> i8 {
                __EOF_ACTION[state as usize]
            }
            #[inline]
            fn goto(&self, state: i8, nt: usize) -> i8 {
                __goto(state, nt)
            }
            fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
                __token_to_symbol(token_index, token, core::marker::PhantomData::<(&())>)
            }
            fn expected_tokens(&self, state: i8) -> alloc::vec::Vec<alloc::string::String> {
                __expected_tokens(state)
            }
            #[inline]
            fn uses_error_recovery(&self) -> bool {
                false
            }
            #[inline]
            fn error_recovery_symbol(
                &self,
                recovery: __state_machine::ErrorRecovery<Self>,
            ) -> Self::Symbol {
                ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                    &["error recovery not enabled for this grammar"],
                    &match () {
                        () => [],
                    },
                ))
            }
            fn reduce(
                &mut self,
                action: i8,
                start_location: Option<&Self::Location>,
                states: &mut alloc::vec::Vec<i8>,
                symbols: &mut alloc::vec::Vec<__state_machine::SymbolTriple<Self>>,
            ) -> Option<__state_machine::ParseResult<Self>> {
                __reduce(
                    self.input,
                    action,
                    start_location,
                    states,
                    symbols,
                    core::marker::PhantomData::<(&())>,
                )
            }
            fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
                ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                    &["error recovery not enabled for this grammar"],
                    &match () {
                        () => [],
                    },
                ))
            }
        }
        fn __token_to_integer<'input>(
            __token: &Token<'input>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> Option<usize> {
            match *__token {
                Token(1, _) if true => Some(0),
                Token(2, _) if true => Some(1),
                Token(3, _) if true => Some(2),
                Token(0, _) if true => Some(3),
                _ => None,
            }
        }
        fn __token_to_symbol<'input>(
            __token_index: usize,
            __token: Token<'input>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> __Symbol<'input> {
            match __token_index {
                0 | 1 | 2 | 3 => match __token {
                    Token(1, __tok0) | Token(2, __tok0) | Token(3, __tok0) | Token(0, __tok0)
                        if true =>
                    {
                        __Symbol::Variant0(__tok0)
                    }
                    _ => ::core::panicking::panic("internal error: entered unreachable code"),
                },
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            }
        }
        pub struct FunctionArgsParser {
            builder: __lalrpop_util::lexer::MatcherBuilder,
            _priv: (),
        }
        impl FunctionArgsParser {
            pub fn new() -> FunctionArgsParser {
                let __builder = super::__intern_token::new_builder();
                FunctionArgsParser {
                    builder: __builder,
                    _priv: (),
                }
            }
            #[allow(dead_code)]
            pub fn parse<'input>(
                &self,
                input: &'input str,
            ) -> Result<
                alloc::vec::Vec<FunctionArg>,
                __lalrpop_util::ParseError<usize, Token<'input>, &'static str>,
            > {
                let mut __tokens = self.builder.matcher(input);
                __state_machine::Parser::drive(
                    __StateMachine {
                        input,
                        __phantom: core::marker::PhantomData::<(&())>,
                    },
                    __tokens,
                )
            }
        }
        pub(crate) fn __reduce<'input>(
            input: &'input str,
            __action: i8,
            __lookahead_start: Option<&usize>,
            __states: &mut alloc::vec::Vec<i8>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> Option<
            Result<
                alloc::vec::Vec<FunctionArg>,
                __lalrpop_util::ParseError<usize, Token<'input>, &'static str>,
            >,
        > {
            let (__pop_states, __nonterminal) = match __action {
                0 => __reduce0(
                    input,
                    __lookahead_start,
                    __symbols,
                    core::marker::PhantomData::<(&())>,
                ),
                1 => __reduce1(
                    input,
                    __lookahead_start,
                    __symbols,
                    core::marker::PhantomData::<(&())>,
                ),
                2 => __reduce2(
                    input,
                    __lookahead_start,
                    __symbols,
                    core::marker::PhantomData::<(&())>,
                ),
                3 => __reduce3(
                    input,
                    __lookahead_start,
                    __symbols,
                    core::marker::PhantomData::<(&())>,
                ),
                4 => __reduce4(
                    input,
                    __lookahead_start,
                    __symbols,
                    core::marker::PhantomData::<(&())>,
                ),
                5 => __reduce5(
                    input,
                    __lookahead_start,
                    __symbols,
                    core::marker::PhantomData::<(&())>,
                ),
                6 => __reduce6(
                    input,
                    __lookahead_start,
                    __symbols,
                    core::marker::PhantomData::<(&())>,
                ),
                7 => __reduce7(
                    input,
                    __lookahead_start,
                    __symbols,
                    core::marker::PhantomData::<(&())>,
                ),
                8 => __reduce8(
                    input,
                    __lookahead_start,
                    __symbols,
                    core::marker::PhantomData::<(&())>,
                ),
                9 => __reduce9(
                    input,
                    __lookahead_start,
                    __symbols,
                    core::marker::PhantomData::<(&())>,
                ),
                10 => __reduce10(
                    input,
                    __lookahead_start,
                    __symbols,
                    core::marker::PhantomData::<(&())>,
                ),
                11 => __reduce11(
                    input,
                    __lookahead_start,
                    __symbols,
                    core::marker::PhantomData::<(&())>,
                ),
                12 => __reduce12(
                    input,
                    __lookahead_start,
                    __symbols,
                    core::marker::PhantomData::<(&())>,
                ),
                13 => {
                    let __sym0 = __pop_Variant2(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action1(input, __sym0);
                    return Some(Ok(__nt));
                }
                14 => __reduce14(
                    input,
                    __lookahead_start,
                    __symbols,
                    core::marker::PhantomData::<(&())>,
                ),
                _ => ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                    &["invalid action code "],
                    &match (&__action,) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                )),
            };
            let __states_len = __states.len();
            __states.truncate(__states_len - __pop_states);
            let __state = *__states.last().unwrap();
            let __next_state = __goto(__state, __nonterminal);
            __states.push(__next_state);
            None
        }
        #[inline(never)]
        fn __symbol_type_mismatch() -> ! {
            ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                &["symbol type mismatch"],
                &match () {
                    () => [],
                },
            ))
        }
        fn __pop_Variant3<'input>(
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        ) -> (usize, Box<Function>, usize) {
            match __symbols.pop() {
                Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
                _ => __symbol_type_mismatch(),
            }
        }
        fn __pop_Variant1<'input>(
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        ) -> (usize, FunctionArg, usize) {
            match __symbols.pop() {
                Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
                _ => __symbol_type_mismatch(),
            }
        }
        fn __pop_Variant4<'input>(
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        ) -> (usize, String, usize) {
            match __symbols.pop() {
                Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
                _ => __symbol_type_mismatch(),
            }
        }
        fn __pop_Variant2<'input>(
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        ) -> (usize, alloc::vec::Vec<FunctionArg>, usize) {
            match __symbols.pop() {
                Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
                _ => __symbol_type_mismatch(),
            }
        }
        fn __pop_Variant0<'input>(
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        ) -> (usize, &'input str, usize) {
            match __symbols.pop() {
                Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
                _ => __symbol_type_mismatch(),
            }
        }
        pub(crate) fn __reduce0<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            if !(__symbols.len() >= 2) {
                ::core::panicking::panic("assertion failed: __symbols.len() >= 2")
            };
            let __sym1 = __pop_Variant0(__symbols);
            let __sym0 = __pop_Variant1(__symbols);
            let __start = __sym0.0.clone();
            let __end = __sym1.2.clone();
            let __nt = super::__action11(input, __sym0, __sym1);
            __symbols.push((__start, __Symbol::Variant1(__nt), __end));
            (2, 0)
        }
        pub(crate) fn __reduce1<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            let __start = __lookahead_start
                .cloned()
                .or_else(|| __symbols.last().map(|s| s.2.clone()))
                .unwrap_or_default();
            let __end = __start.clone();
            let __nt = super::__action9(input, &__start, &__end);
            __symbols.push((__start, __Symbol::Variant2(__nt), __end));
            (0, 1)
        }
        pub(crate) fn __reduce2<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            let __sym0 = __pop_Variant2(__symbols);
            let __start = __sym0.0.clone();
            let __end = __sym0.2.clone();
            let __nt = super::__action10(input, __sym0);
            __symbols.push((__start, __Symbol::Variant2(__nt), __end));
            (1, 1)
        }
        pub(crate) fn __reduce3<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            if !(__symbols.len() >= 2) {
                ::core::panicking::panic("assertion failed: __symbols.len() >= 2")
            };
            let __sym1 = __pop_Variant0(__symbols);
            let __sym0 = __pop_Variant1(__symbols);
            let __start = __sym0.0.clone();
            let __end = __sym1.2.clone();
            let __nt = super::__action14(input, __sym0, __sym1);
            __symbols.push((__start, __Symbol::Variant2(__nt), __end));
            (2, 2)
        }
        pub(crate) fn __reduce4<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            if !(__symbols.len() >= 3) {
                ::core::panicking::panic("assertion failed: __symbols.len() >= 3")
            };
            let __sym2 = __pop_Variant0(__symbols);
            let __sym1 = __pop_Variant1(__symbols);
            let __sym0 = __pop_Variant2(__symbols);
            let __start = __sym0.0.clone();
            let __end = __sym2.2.clone();
            let __nt = super::__action15(input, __sym0, __sym1, __sym2);
            __symbols.push((__start, __Symbol::Variant2(__nt), __end));
            (3, 2)
        }
        pub(crate) fn __reduce5<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            if !(__symbols.len() >= 3) {
                ::core::panicking::panic("assertion failed: __symbols.len() >= 3")
            };
            let __sym2 = __pop_Variant0(__symbols);
            let __sym1 = __pop_Variant0(__symbols);
            let __sym0 = __pop_Variant4(__symbols);
            let __start = __sym0.0.clone();
            let __end = __sym2.2.clone();
            let __nt = super::__action4(input, __sym0, __sym1, __sym2);
            __symbols.push((__start, __Symbol::Variant3(__nt), __end));
            (3, 3)
        }
        pub(crate) fn __reduce6<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            if !(__symbols.len() >= 2) {
                ::core::panicking::panic("assertion failed: __symbols.len() >= 2")
            };
            let __sym1 = __pop_Variant4(__symbols);
            let __sym0 = __pop_Variant4(__symbols);
            let __start = __sym0.0.clone();
            let __end = __sym1.2.clone();
            let __nt = super::__action7(input, __sym0, __sym1);
            __symbols.push((__start, __Symbol::Variant1(__nt), __end));
            (2, 4)
        }
        pub(crate) fn __reduce7<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            let __sym0 = __pop_Variant1(__symbols);
            let __start = __sym0.0.clone();
            let __end = __sym0.2.clone();
            let __nt = super::__action5(input, __sym0);
            __symbols.push((__start, __Symbol::Variant2(__nt), __end));
            (1, 5)
        }
        pub(crate) fn __reduce8<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            let __start = __lookahead_start
                .cloned()
                .or_else(|| __symbols.last().map(|s| s.2.clone()))
                .unwrap_or_default();
            let __end = __start.clone();
            let __nt = super::__action16(input, &__start, &__end);
            __symbols.push((__start, __Symbol::Variant2(__nt), __end));
            (0, 5)
        }
        pub(crate) fn __reduce9<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            let __sym0 = __pop_Variant2(__symbols);
            let __start = __sym0.0.clone();
            let __end = __sym0.2.clone();
            let __nt = super::__action17(input, __sym0);
            __symbols.push((__start, __Symbol::Variant2(__nt), __end));
            (1, 5)
        }
        pub(crate) fn __reduce10<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            let __sym0 = __pop_Variant0(__symbols);
            let __start = __sym0.0.clone();
            let __end = __sym0.2.clone();
            let __nt = super::__action8(input, __sym0);
            __symbols.push((__start, __Symbol::Variant4(__nt), __end));
            (1, 6)
        }
        pub(crate) fn __reduce11<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            let __sym0 = __pop_Variant3(__symbols);
            let __start = __sym0.0.clone();
            let __end = __sym0.2.clone();
            let __nt = super::__action0(input, __sym0);
            __symbols.push((__start, __Symbol::Variant3(__nt), __end));
            (1, 7)
        }
        pub(crate) fn __reduce12<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            let __sym0 = __pop_Variant1(__symbols);
            let __start = __sym0.0.clone();
            let __end = __sym0.2.clone();
            let __nt = super::__action2(input, __sym0);
            __symbols.push((__start, __Symbol::Variant1(__nt), __end));
            (1, 8)
        }
        pub(crate) fn __reduce14<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            let __sym0 = __pop_Variant4(__symbols);
            let __start = __sym0.0.clone();
            let __end = __sym0.2.clone();
            let __nt = super::__action3(input, __sym0);
            __symbols.push((__start, __Symbol::Variant4(__nt), __end));
            (1, 10)
        }
    }
    pub use self::__parse__FunctionArgs::FunctionArgsParser;
    mod __parse__Identifier {
        #![allow(
            non_snake_case,
            non_camel_case_types,
            unused_mut,
            unused_variables,
            unused_imports,
            unused_parens
        )]
        use std::str::FromStr;
        use crate::ast::{Function, FunctionArg};
        #[allow(unused_extern_crates)]
        extern crate lalrpop_util as __lalrpop_util;
        #[allow(unused_imports)]
        use self::__lalrpop_util::state_machine as __state_machine;
        extern crate core;
        extern crate alloc;
        use self::__lalrpop_util::lexer::Token;
        #[allow(dead_code)]
        pub(crate) enum __Symbol<'input> {
            Variant0(&'input str),
            Variant1(FunctionArg),
            Variant2(alloc::vec::Vec<FunctionArg>),
            Variant3(Box<Function>),
            Variant4(String),
        }
        const __ACTION: &[i8] = &[0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0];
        fn __action(state: i8, integer: usize) -> i8 {
            __ACTION[(state as usize) * 4 + integer]
        }
        const __EOF_ACTION: &[i8] = &[0, -15, -11];
        fn __goto(state: i8, nt: usize) -> i8 {
            match nt {
                6 => 1,
                _ => 0,
            }
        }
        fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
            const __TERMINAL: &[&str] = &[
                r###""(""###,
                r###"")""###,
                r###"",""###,
                r###"r#"[A-z]*"#"###,
            ];
            __TERMINAL
                .iter()
                .enumerate()
                .filter_map(|(index, terminal)| {
                    let next_state = __action(__state, index);
                    if next_state == 0 {
                        None
                    } else {
                        Some(alloc::string::ToString::to_string(terminal))
                    }
                })
                .collect()
        }
        pub(crate) struct __StateMachine<'input> {
            input: &'input str,
            __phantom: core::marker::PhantomData<(&'input ())>,
        }
        impl<'input> __state_machine::ParserDefinition for __StateMachine<'input> {
            type Location = usize;
            type Error = &'static str;
            type Token = Token<'input>;
            type TokenIndex = usize;
            type Symbol = __Symbol<'input>;
            type Success = String;
            type StateIndex = i8;
            type Action = i8;
            type ReduceIndex = i8;
            type NonterminalIndex = usize;
            #[inline]
            fn start_location(&self) -> Self::Location {
                Default::default()
            }
            #[inline]
            fn start_state(&self) -> Self::StateIndex {
                0
            }
            #[inline]
            fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
                __token_to_integer(token, core::marker::PhantomData::<(&())>)
            }
            #[inline]
            fn action(&self, state: i8, integer: usize) -> i8 {
                __action(state, integer)
            }
            #[inline]
            fn error_action(&self, state: i8) -> i8 {
                __action(state, 4 - 1)
            }
            #[inline]
            fn eof_action(&self, state: i8) -> i8 {
                __EOF_ACTION[state as usize]
            }
            #[inline]
            fn goto(&self, state: i8, nt: usize) -> i8 {
                __goto(state, nt)
            }
            fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
                __token_to_symbol(token_index, token, core::marker::PhantomData::<(&())>)
            }
            fn expected_tokens(&self, state: i8) -> alloc::vec::Vec<alloc::string::String> {
                __expected_tokens(state)
            }
            #[inline]
            fn uses_error_recovery(&self) -> bool {
                false
            }
            #[inline]
            fn error_recovery_symbol(
                &self,
                recovery: __state_machine::ErrorRecovery<Self>,
            ) -> Self::Symbol {
                ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                    &["error recovery not enabled for this grammar"],
                    &match () {
                        () => [],
                    },
                ))
            }
            fn reduce(
                &mut self,
                action: i8,
                start_location: Option<&Self::Location>,
                states: &mut alloc::vec::Vec<i8>,
                symbols: &mut alloc::vec::Vec<__state_machine::SymbolTriple<Self>>,
            ) -> Option<__state_machine::ParseResult<Self>> {
                __reduce(
                    self.input,
                    action,
                    start_location,
                    states,
                    symbols,
                    core::marker::PhantomData::<(&())>,
                )
            }
            fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
                ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                    &["error recovery not enabled for this grammar"],
                    &match () {
                        () => [],
                    },
                ))
            }
        }
        fn __token_to_integer<'input>(
            __token: &Token<'input>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> Option<usize> {
            match *__token {
                Token(1, _) if true => Some(0),
                Token(2, _) if true => Some(1),
                Token(3, _) if true => Some(2),
                Token(0, _) if true => Some(3),
                _ => None,
            }
        }
        fn __token_to_symbol<'input>(
            __token_index: usize,
            __token: Token<'input>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> __Symbol<'input> {
            match __token_index {
                0 | 1 | 2 | 3 => match __token {
                    Token(1, __tok0) | Token(2, __tok0) | Token(3, __tok0) | Token(0, __tok0)
                        if true =>
                    {
                        __Symbol::Variant0(__tok0)
                    }
                    _ => ::core::panicking::panic("internal error: entered unreachable code"),
                },
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            }
        }
        pub struct IdentifierParser {
            builder: __lalrpop_util::lexer::MatcherBuilder,
            _priv: (),
        }
        impl IdentifierParser {
            pub fn new() -> IdentifierParser {
                let __builder = super::__intern_token::new_builder();
                IdentifierParser {
                    builder: __builder,
                    _priv: (),
                }
            }
            #[allow(dead_code)]
            pub fn parse<'input>(
                &self,
                input: &'input str,
            ) -> Result<String, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
            {
                let mut __tokens = self.builder.matcher(input);
                __state_machine::Parser::drive(
                    __StateMachine {
                        input,
                        __phantom: core::marker::PhantomData::<(&())>,
                    },
                    __tokens,
                )
            }
        }
        pub(crate) fn __reduce<'input>(
            input: &'input str,
            __action: i8,
            __lookahead_start: Option<&usize>,
            __states: &mut alloc::vec::Vec<i8>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> Option<Result<String, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>
        {
            let (__pop_states, __nonterminal) = match __action {
                0 => __reduce0(
                    input,
                    __lookahead_start,
                    __symbols,
                    core::marker::PhantomData::<(&())>,
                ),
                1 => __reduce1(
                    input,
                    __lookahead_start,
                    __symbols,
                    core::marker::PhantomData::<(&())>,
                ),
                2 => __reduce2(
                    input,
                    __lookahead_start,
                    __symbols,
                    core::marker::PhantomData::<(&())>,
                ),
                3 => __reduce3(
                    input,
                    __lookahead_start,
                    __symbols,
                    core::marker::PhantomData::<(&())>,
                ),
                4 => __reduce4(
                    input,
                    __lookahead_start,
                    __symbols,
                    core::marker::PhantomData::<(&())>,
                ),
                5 => __reduce5(
                    input,
                    __lookahead_start,
                    __symbols,
                    core::marker::PhantomData::<(&())>,
                ),
                6 => __reduce6(
                    input,
                    __lookahead_start,
                    __symbols,
                    core::marker::PhantomData::<(&())>,
                ),
                7 => __reduce7(
                    input,
                    __lookahead_start,
                    __symbols,
                    core::marker::PhantomData::<(&())>,
                ),
                8 => __reduce8(
                    input,
                    __lookahead_start,
                    __symbols,
                    core::marker::PhantomData::<(&())>,
                ),
                9 => __reduce9(
                    input,
                    __lookahead_start,
                    __symbols,
                    core::marker::PhantomData::<(&())>,
                ),
                10 => __reduce10(
                    input,
                    __lookahead_start,
                    __symbols,
                    core::marker::PhantomData::<(&())>,
                ),
                11 => __reduce11(
                    input,
                    __lookahead_start,
                    __symbols,
                    core::marker::PhantomData::<(&())>,
                ),
                12 => __reduce12(
                    input,
                    __lookahead_start,
                    __symbols,
                    core::marker::PhantomData::<(&())>,
                ),
                13 => __reduce13(
                    input,
                    __lookahead_start,
                    __symbols,
                    core::marker::PhantomData::<(&())>,
                ),
                14 => {
                    let __sym0 = __pop_Variant4(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action3(input, __sym0);
                    return Some(Ok(__nt));
                }
                _ => ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                    &["invalid action code "],
                    &match (&__action,) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                )),
            };
            let __states_len = __states.len();
            __states.truncate(__states_len - __pop_states);
            let __state = *__states.last().unwrap();
            let __next_state = __goto(__state, __nonterminal);
            __states.push(__next_state);
            None
        }
        #[inline(never)]
        fn __symbol_type_mismatch() -> ! {
            ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                &["symbol type mismatch"],
                &match () {
                    () => [],
                },
            ))
        }
        fn __pop_Variant3<'input>(
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        ) -> (usize, Box<Function>, usize) {
            match __symbols.pop() {
                Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
                _ => __symbol_type_mismatch(),
            }
        }
        fn __pop_Variant1<'input>(
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        ) -> (usize, FunctionArg, usize) {
            match __symbols.pop() {
                Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
                _ => __symbol_type_mismatch(),
            }
        }
        fn __pop_Variant4<'input>(
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        ) -> (usize, String, usize) {
            match __symbols.pop() {
                Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
                _ => __symbol_type_mismatch(),
            }
        }
        fn __pop_Variant2<'input>(
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        ) -> (usize, alloc::vec::Vec<FunctionArg>, usize) {
            match __symbols.pop() {
                Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
                _ => __symbol_type_mismatch(),
            }
        }
        fn __pop_Variant0<'input>(
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        ) -> (usize, &'input str, usize) {
            match __symbols.pop() {
                Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
                _ => __symbol_type_mismatch(),
            }
        }
        pub(crate) fn __reduce0<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            if !(__symbols.len() >= 2) {
                ::core::panicking::panic("assertion failed: __symbols.len() >= 2")
            };
            let __sym1 = __pop_Variant0(__symbols);
            let __sym0 = __pop_Variant1(__symbols);
            let __start = __sym0.0.clone();
            let __end = __sym1.2.clone();
            let __nt = super::__action11(input, __sym0, __sym1);
            __symbols.push((__start, __Symbol::Variant1(__nt), __end));
            (2, 0)
        }
        pub(crate) fn __reduce1<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            let __start = __lookahead_start
                .cloned()
                .or_else(|| __symbols.last().map(|s| s.2.clone()))
                .unwrap_or_default();
            let __end = __start.clone();
            let __nt = super::__action9(input, &__start, &__end);
            __symbols.push((__start, __Symbol::Variant2(__nt), __end));
            (0, 1)
        }
        pub(crate) fn __reduce2<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            let __sym0 = __pop_Variant2(__symbols);
            let __start = __sym0.0.clone();
            let __end = __sym0.2.clone();
            let __nt = super::__action10(input, __sym0);
            __symbols.push((__start, __Symbol::Variant2(__nt), __end));
            (1, 1)
        }
        pub(crate) fn __reduce3<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            if !(__symbols.len() >= 2) {
                ::core::panicking::panic("assertion failed: __symbols.len() >= 2")
            };
            let __sym1 = __pop_Variant0(__symbols);
            let __sym0 = __pop_Variant1(__symbols);
            let __start = __sym0.0.clone();
            let __end = __sym1.2.clone();
            let __nt = super::__action14(input, __sym0, __sym1);
            __symbols.push((__start, __Symbol::Variant2(__nt), __end));
            (2, 2)
        }
        pub(crate) fn __reduce4<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            if !(__symbols.len() >= 3) {
                ::core::panicking::panic("assertion failed: __symbols.len() >= 3")
            };
            let __sym2 = __pop_Variant0(__symbols);
            let __sym1 = __pop_Variant1(__symbols);
            let __sym0 = __pop_Variant2(__symbols);
            let __start = __sym0.0.clone();
            let __end = __sym2.2.clone();
            let __nt = super::__action15(input, __sym0, __sym1, __sym2);
            __symbols.push((__start, __Symbol::Variant2(__nt), __end));
            (3, 2)
        }
        pub(crate) fn __reduce5<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            if !(__symbols.len() >= 3) {
                ::core::panicking::panic("assertion failed: __symbols.len() >= 3")
            };
            let __sym2 = __pop_Variant0(__symbols);
            let __sym1 = __pop_Variant0(__symbols);
            let __sym0 = __pop_Variant4(__symbols);
            let __start = __sym0.0.clone();
            let __end = __sym2.2.clone();
            let __nt = super::__action4(input, __sym0, __sym1, __sym2);
            __symbols.push((__start, __Symbol::Variant3(__nt), __end));
            (3, 3)
        }
        pub(crate) fn __reduce6<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            if !(__symbols.len() >= 2) {
                ::core::panicking::panic("assertion failed: __symbols.len() >= 2")
            };
            let __sym1 = __pop_Variant4(__symbols);
            let __sym0 = __pop_Variant4(__symbols);
            let __start = __sym0.0.clone();
            let __end = __sym1.2.clone();
            let __nt = super::__action7(input, __sym0, __sym1);
            __symbols.push((__start, __Symbol::Variant1(__nt), __end));
            (2, 4)
        }
        pub(crate) fn __reduce7<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            let __sym0 = __pop_Variant1(__symbols);
            let __start = __sym0.0.clone();
            let __end = __sym0.2.clone();
            let __nt = super::__action5(input, __sym0);
            __symbols.push((__start, __Symbol::Variant2(__nt), __end));
            (1, 5)
        }
        pub(crate) fn __reduce8<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            let __start = __lookahead_start
                .cloned()
                .or_else(|| __symbols.last().map(|s| s.2.clone()))
                .unwrap_or_default();
            let __end = __start.clone();
            let __nt = super::__action16(input, &__start, &__end);
            __symbols.push((__start, __Symbol::Variant2(__nt), __end));
            (0, 5)
        }
        pub(crate) fn __reduce9<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            let __sym0 = __pop_Variant2(__symbols);
            let __start = __sym0.0.clone();
            let __end = __sym0.2.clone();
            let __nt = super::__action17(input, __sym0);
            __symbols.push((__start, __Symbol::Variant2(__nt), __end));
            (1, 5)
        }
        pub(crate) fn __reduce10<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            let __sym0 = __pop_Variant0(__symbols);
            let __start = __sym0.0.clone();
            let __end = __sym0.2.clone();
            let __nt = super::__action8(input, __sym0);
            __symbols.push((__start, __Symbol::Variant4(__nt), __end));
            (1, 6)
        }
        pub(crate) fn __reduce11<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            let __sym0 = __pop_Variant3(__symbols);
            let __start = __sym0.0.clone();
            let __end = __sym0.2.clone();
            let __nt = super::__action0(input, __sym0);
            __symbols.push((__start, __Symbol::Variant3(__nt), __end));
            (1, 7)
        }
        pub(crate) fn __reduce12<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            let __sym0 = __pop_Variant1(__symbols);
            let __start = __sym0.0.clone();
            let __end = __sym0.2.clone();
            let __nt = super::__action2(input, __sym0);
            __symbols.push((__start, __Symbol::Variant1(__nt), __end));
            (1, 8)
        }
        pub(crate) fn __reduce13<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            let __sym0 = __pop_Variant2(__symbols);
            let __start = __sym0.0.clone();
            let __end = __sym0.2.clone();
            let __nt = super::__action1(input, __sym0);
            __symbols.push((__start, __Symbol::Variant2(__nt), __end));
            (1, 9)
        }
    }
    pub use self::__parse__Identifier::IdentifierParser;
    mod __intern_token {
        #![allow(unused_imports)]
        use std::str::FromStr;
        use crate::ast::{Function, FunctionArg};
        #[allow(unused_extern_crates)]
        extern crate lalrpop_util as __lalrpop_util;
        #[allow(unused_imports)]
        use self::__lalrpop_util::state_machine as __state_machine;
        extern crate core;
        extern crate alloc;
        pub fn new_builder() -> __lalrpop_util::lexer::MatcherBuilder {
            let __strs: &[(&str, bool)] = &[
                ("^([A-z]*)", false),
                ("^(\\()", false),
                ("^(\\))", false),
                ("^(,)", false),
                (r"^(\s*)", true),
            ];
            __lalrpop_util::lexer::MatcherBuilder::new(__strs.iter().copied()).unwrap()
        }
    }
    pub(crate) use self::__lalrpop_util::lexer::Token;
    #[allow(unused_variables)]
    fn __action0<'input>(
        input: &'input str,
        (_, __0, _): (usize, Box<Function>, usize),
    ) -> Box<Function> {
        __0
    }
    #[allow(unused_variables)]
    fn __action1<'input>(
        input: &'input str,
        (_, __0, _): (usize, alloc::vec::Vec<FunctionArg>, usize),
    ) -> alloc::vec::Vec<FunctionArg> {
        __0
    }
    #[allow(unused_variables)]
    fn __action2<'input>(
        input: &'input str,
        (_, __0, _): (usize, FunctionArg, usize),
    ) -> FunctionArg {
        __0
    }
    #[allow(unused_variables)]
    fn __action3<'input>(input: &'input str, (_, __0, _): (usize, String, usize)) -> String {
        __0
    }
    #[allow(unused_variables)]
    fn __action4<'input>(
        input: &'input str,
        (_, i, _): (usize, String, usize),
        (_, _, _): (usize, &'input str, usize),
        (_, _, _): (usize, &'input str, usize),
    ) -> Box<Function> {
        Box::new(Function::new(i))
    }
    #[allow(unused_variables)]
    fn __action5<'input>(
        input: &'input str,
        (_, __0, _): (usize, FunctionArg, usize),
    ) -> alloc::vec::Vec<FunctionArg> {
        Vec::from(__0)
    }
    #[allow(unused_variables)]
    fn __action6<'input>(
        input: &'input str,
        (_, __0, _): (usize, alloc::vec::Vec<FunctionArg>, usize),
    ) -> alloc::vec::Vec<FunctionArg> {
        __0
    }
    #[allow(unused_variables)]
    fn __action7<'input>(
        input: &'input str,
        (_, ty, _): (usize, String, usize),
        (_, ident, _): (usize, String, usize),
    ) -> FunctionArg {
        FunctionArg {
            ty: ty,
            ident: ident,
        }
    }
    #[allow(unused_variables)]
    fn __action8<'input>(input: &'input str, (_, __0, _): (usize, &'input str, usize)) -> String {
        String::from(__0)
    }
    #[allow(unused_variables)]
    fn __action9<'input>(
        input: &'input str,
        __lookbehind: &usize,
        __lookahead: &usize,
    ) -> alloc::vec::Vec<FunctionArg> {
        ::alloc::vec::Vec::new()
    }
    #[allow(unused_variables)]
    fn __action10<'input>(
        input: &'input str,
        (_, v, _): (usize, alloc::vec::Vec<FunctionArg>, usize),
    ) -> alloc::vec::Vec<FunctionArg> {
        v
    }
    #[allow(unused_variables)]
    fn __action11<'input>(
        input: &'input str,
        (_, __0, _): (usize, FunctionArg, usize),
        (_, _, _): (usize, &'input str, usize),
    ) -> FunctionArg {
        __0
    }
    #[allow(unused_variables)]
    fn __action12<'input>(
        input: &'input str,
        (_, __0, _): (usize, FunctionArg, usize),
    ) -> alloc::vec::Vec<FunctionArg> {
        <[_]>::into_vec(box [__0])
    }
    #[allow(unused_variables)]
    fn __action13<'input>(
        input: &'input str,
        (_, v, _): (usize, alloc::vec::Vec<FunctionArg>, usize),
        (_, e, _): (usize, FunctionArg, usize),
    ) -> alloc::vec::Vec<FunctionArg> {
        {
            let mut v = v;
            v.push(e);
            v
        }
    }
    #[allow(unused_variables)]
    fn __action14<'input>(
        input: &'input str,
        __0: (usize, FunctionArg, usize),
        __1: (usize, &'input str, usize),
    ) -> alloc::vec::Vec<FunctionArg> {
        let __start0 = __0.0.clone();
        let __end0 = __1.2.clone();
        let __temp0 = __action11(input, __0, __1);
        let __temp0 = (__start0, __temp0, __end0);
        __action12(input, __temp0)
    }
    #[allow(unused_variables)]
    fn __action15<'input>(
        input: &'input str,
        __0: (usize, alloc::vec::Vec<FunctionArg>, usize),
        __1: (usize, FunctionArg, usize),
        __2: (usize, &'input str, usize),
    ) -> alloc::vec::Vec<FunctionArg> {
        let __start0 = __1.0.clone();
        let __end0 = __2.2.clone();
        let __temp0 = __action11(input, __1, __2);
        let __temp0 = (__start0, __temp0, __end0);
        __action13(input, __0, __temp0)
    }
    #[allow(unused_variables)]
    fn __action16<'input>(
        input: &'input str,
        __lookbehind: &usize,
        __lookahead: &usize,
    ) -> alloc::vec::Vec<FunctionArg> {
        let __start0 = __lookbehind.clone();
        let __end0 = __lookahead.clone();
        let __temp0 = __action9(input, &__start0, &__end0);
        let __temp0 = (__start0, __temp0, __end0);
        __action6(input, __temp0)
    }
    #[allow(unused_variables)]
    fn __action17<'input>(
        input: &'input str,
        __0: (usize, alloc::vec::Vec<FunctionArg>, usize),
    ) -> alloc::vec::Vec<FunctionArg> {
        let __start0 = __0.0.clone();
        let __end0 = __0.2.clone();
        let __temp0 = __action10(input, __0);
        let __temp0 = (__start0, __temp0, __end0);
        __action6(input, __temp0)
    }
    pub trait __ToTriple<'input> {
        fn to_triple(
            value: Self,
        ) -> Result<
            (usize, Token<'input>, usize),
            __lalrpop_util::ParseError<usize, Token<'input>, &'static str>,
        >;
    }
    impl<'input> __ToTriple<'input> for (usize, Token<'input>, usize) {
        fn to_triple(
            value: Self,
        ) -> Result<
            (usize, Token<'input>, usize),
            __lalrpop_util::ParseError<usize, Token<'input>, &'static str>,
        > {
            Ok(value)
        }
    }
    impl<'input> __ToTriple<'input> for Result<(usize, Token<'input>, usize), &'static str> {
        fn to_triple(
            value: Self,
        ) -> Result<
            (usize, Token<'input>, usize),
            __lalrpop_util::ParseError<usize, Token<'input>, &'static str>,
        > {
            match value {
                Ok(v) => Ok(v),
                Err(error) => Err(__lalrpop_util::ParseError::User { error }),
            }
        }
    }
}
fn main() {
    gram::FunctionArgsParser::new()
        .parse("Test test, Test test")
        .unwrap();
    gram::FunctionArgsParser::new()
        .parse("Test test Test test")
        .unwrap();
}
