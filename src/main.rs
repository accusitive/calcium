use ca_parser_bison::{lexer::Lexer, parser::{Parser, token_name}};
use colored::Colorize;

// use std::path::Path;

// use ca_ast::Program;
// use ca_backend_llvm::{
//     inkwell::{
//         execution_engine::JitFunction,
//         targets::{CodeModel, FileType, RelocMode, Target, TargetTriple},
//         OptimizationLevel,
//     },
//     Compiler,
// };
// fn main() {
//     let w = std::fs::read_to_string("./example.ca").unwrap();
//     let p: Program = ca_parser::gram::ProgramParser::new().parse(&w).unwrap();
//     println!("Program {:#?}", p);
//     let context = ca_backend_llvm::inkwell::context::Context::create();
//     let compiler = Compiler::new_compiler(p, &context);
//     compiler.pre_compile();
//     compiler.compile_program();
//     compiler.module.print_to_stderr();

//     match compiler.module.verify() {
//         Ok(_) => {
//             let good_target = Target::from_name("x86-64").unwrap();
//             let target_machine = good_target
//                 .create_target_machine(
//                     &TargetTriple::create("x86_64-pc-linux-gnu"),
//                     "x86-64",
//                     "+avx2",
//                     OptimizationLevel::Default,
//                     RelocMode::Default,
//                     CodeModel::Default,
//                 )
//                 .unwrap();
//             target_machine
//                 .write_to_file(&compiler.module, FileType::Object, Path::new("./out.o"))
//                 .unwrap();
//         }
//         Err(e) => println!("Error could not validate {}", e),
//     };

//     // unsafe {
//     //     let f: JitFunction<unsafe extern "C" fn(i32, i32) -> i32> =
//     //         compiler.execution_engine.get_function("comp").unwrap();
//     //     // println!("Result {:#?}", f.call(2, 4));
//     //     assert_eq!(f.call(1, 100), 2);
//     //     assert_eq!(f.call(100, 1), 4);
//     // }
//     unsafe {
//         let entry: JitFunction<unsafe extern "C" fn() -> i32> =
//             compiler.execution_engine.get_function("entry").unwrap();
//         println!("Program exited with value: {}", entry.call());
//     };
// }
// #[cfg(test)]
// mod tests {
//     use ca_backend_llvm::{
//         inkwell::{
//             context::Context,
//             execution_engine::{JitFunction, UnsafeFunctionPointer},
//             targets::{InitializationConfig, Target},
//         },
//         Compiler,
//     };
//     fn framework<'a, F: UnsafeFunctionPointer>(
//         source: &'a str,
//         function_name: &'a str,
//     ) -> JitFunction<F> {
//         let program = ca_parser::gram::ProgramParser::new().parse(source).unwrap();
//         let context = &Context::create();
//         let compiler = Compiler::new_compiler(program, context);
//         compiler.pre_compile();
//         compiler.compile_program();
//         compiler.module.print_to_stderr();
//         match compiler.module.verify() {
//             Ok(_) => println!("Validated fine"),
//             Err(e) => eprintln!("Could not validate {}", e),
//         }
//         Target::initialize_all(&InitializationConfig::default());
//         // compiler.execution_engine.add_module(&compiler.module).unwrap();
//         let f: JitFunction<F> =
//             unsafe { compiler.execution_engine.get_function(function_name) }.unwrap();
//         f
//     }
//     #[test]
//     fn test_add() {
//         let w = include_str!("../tests/add.ca");
//         let add = framework::<unsafe extern "C" fn(u32, u32) -> u32>(w, "add");

//         unsafe {
//             assert_eq!(add.call(2, 4), 2 + 4);
//             assert_eq!(add.call(2, 4), 2 + 4);
//             assert_ne!(add.call(2, 4), 8);
//         }
//         std::mem::forget(add);
//     }
//     fn fib(num: i32) -> i32 {
//         if num < 2 {
//             num
//         } else {
//             fib(num - 1) + fib(num - 2)
//         }
//     }
//     #[test]
//     fn test_fib() {
//         let w = include_str!("../tests/fib.ca");
//         let jit_fib = framework::<unsafe extern "C" fn(i32) -> i32>(w, "fib");
//         unsafe {
//             assert_eq!(jit_fib.call(2), fib(2));
//             assert_eq!(jit_fib.call(4), fib(4));
//             assert_eq!(jit_fib.call(12), fib(12));
//             for i in 0..10 {
//                 assert_eq!(jit_fib.call(i), fib(i));
//             }
//         }
//         std::mem::forget(jit_fib);
//     }
// }

fn main() {
    let source = "fn add(a: i32 b: i32), fn add(a: i32, b: i32)";

    let mut src = source.to_string();

    let lexer = Lexer::new(source);
    for token in lexer {
        if token.token_type == ca_parser_bison::lexer::Lexer::YYEOF {
            break;
        }
        let s = token.token_value.to_string();
        let s2 = match token.token_type {
            Lexer::tIDENTIFIER => s.underline(),
            Lexer::tFN => s.cyan(),
            Lexer::tLPAREN | Lexer::tRPAREN | Lexer::tRBRACK | Lexer::tLBRACK => s.yellow(),
            Lexer::tCOLON => s.bright_purple(),
            _ => s.white(),
        };
        // print!("{}{}", token.spaces_before, s2);
        println!("{} {}", token_name(token.token_type), s);
    }
    println!("");

    // println!("{}", src);

    let lexer = Lexer::new(source);

    let parser = Parser::new(lexer, "Cheese", source);

    let parsed = parser.do_parse();

    match parsed.0 {
        Some(_result) => {
            let (_value, _name, output) = parsed;
            println!("Output: {:#?}", output.unwrap());
        }
        None => {
            println!("{}", "Compilation failed.".bold())
        },
    }
}
