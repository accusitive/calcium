use std::path::PathBuf;

use ca_backend_llvm::inkwell::execution_engine::JitFunction;
use ca_parser_bison::{
    lexer::Lexer,
    parser::{token_name, Parser},
};
use colored::Colorize;

fn main() {
    let mut path = PathBuf::new();
    path.push("examples");
    path.push("main");
    path.set_extension("ca");
    let source = &std::fs::read_to_string(path.clone()).unwrap();
    let lexer = Lexer::new(source);

    for token in lexer {
        if token.token_type == Lexer::YYEOF {
            break;
        }
        println!(
            "{:>15} := {:<10} @ {:>2}..{:>2}  ",
            token_name(token.token_type),
            token.token_value,
            token.loc.begin,
            token.loc.end
        );
    }

    let lexer = Lexer::new(source);

    let parser = Parser::new(lexer, "Cheese", source, path);

    let parsed = parser.do_parse();

    match parsed.0 {
        Some(_result) => {
            let (_value, _name, output) = parsed;
            println!("Output: {:#?}", output.as_ref().unwrap());
            let program = ca_uir::to_program(&output.unwrap());
            let ctx = ca_backend_llvm::inkwell::context::Context::create();
            let compiler = ca_backend_llvm::Compiler::new_compiler(&ctx);
            compiler.compile_program(&program);
            compiler.module.print_to_stderr();
            if let Err(e) = compiler.module.verify() {
                println!("LLVM ERROR: {}", e.to_string());
            } else {
                compiler.write_object_file(&PathBuf::from("./out.o"));

                unsafe {
                    macro_rules! get_f {
                        ($f: expr) => {
                            compiler.execution_engine.get_function($f).unwrap()
                        };
                    }
                    let main: JitFunction<unsafe extern "C" fn() -> TwoNumbers> =
                        get_f!("cheese__main");
                    let get_first: JitFunction<unsafe extern "C" fn(TwoNumbers) -> i32> =
                        get_f!("cheese__first");
                    let get_second: JitFunction<unsafe extern "C" fn(TwoNumbers) -> i64> =
                        get_f!("cheese__second");

                    let tn = main.call();
                    let first = get_first.call(tn.clone());
                    let second = get_second.call(tn);
                    println!("First: {}\nSecond: {}", first, second);
                }
            }
        }
        None => {
            println!("{}", "Compilation failed.".bold())
        }
    }
}
#[test]
fn test_add() {
    let source = include_str!("../examples/tests/add.ca");
    let lexer = Lexer::new(source);
    let parser = Parser::new(lexer, "Cheese", source, PathBuf::new());
    let value = parser.do_parse().2.unwrap();
    let program = ca_uir::to_program(&value);

    let ctx = ca_backend_llvm::inkwell::context::Context::create();
    let compiler = ca_backend_llvm::Compiler::new_compiler(&ctx);
    compiler.compile_program(&program);

    unsafe {
        let add: JitFunction<unsafe extern "C" fn(i32, i32) -> i32> = compiler
            .execution_engine
            .get_function("cheese__add")
            .unwrap();
        assert_eq!(add.call(1, 2), 1 + 2);
        assert_eq!(add.call(-1, 2), -1 + 2);
    }
}
#[derive(Debug, Clone)]
#[repr(C)]
struct TwoNumbers {
    first: i32,
    second: i32,
}
