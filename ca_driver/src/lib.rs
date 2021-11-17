use std::path::PathBuf;

use ca_backend_llvm::{
    inkwell::{
        context::Context, execution_engine::JitFunction, memory_buffer::MemoryBuffer,
        module::Module, OptimizationLevel,
    },
    Compiler,
};
use ca_parser_bison::{lexer::Lexer, parser::Parser};
use clap::{App, Arg};
#[derive(Debug)]
pub enum OutputType {
    LlvmIR,
    Binary,
    Object,
    Assembly,
    Jit,
}
#[derive(Debug)]
pub struct DriverConfig {
    file: PathBuf,
    type_check: bool,
    output_ty: OutputType,
    only_lex: bool,
    optimization: OptimizationLevel,
    target: String,
    graph: u64,
}
pub struct Driver {
    config: DriverConfig,
}
impl Driver {
    pub fn new() -> Driver {
        let matches = App::new("Calcium Compiler")
            .version("0.0")
            .author("Sir George Lewis the 3rd")
            .about("Compiled calcium source code to native code")
            .arg(Arg::with_name("file").required(true).index(1))
            .arg(Arg::with_name("lex").short("l").help("Lex and exit."))
            .arg(
                Arg::with_name("experimental")
                    .short("X")
                    .takes_value(true)
                    .multiple(true)
                    .help("typecheck"),
            )
            .arg(
                Arg::with_name("outputtype")
                    .help("Type of file for the compiler to output")
                    .short("t")
                    .possible_values(&["llvm-ir", "object", "bin", "asm", "jit"])
                    .takes_value(true)
                    .max_values(1),
            )
            .arg(Arg::with_name("target").long("target").takes_value(true))
            .arg(Arg::with_name("optimization").short("o").takes_value(true))
            .arg(
                Arg::with_name("control flow graph")
                    .short("c")
                    .multiple(true)
                    .max_values(2)
                    .takes_value(false),
            )
            .get_matches();

        let typecheck = match matches.values_of("experimental") {
            Some(mut values) => {
                println!("Values {:#?}", values);

                values.find(|v| v == &"typecheck").is_some()
            }
            None => false,
        };
        let opti: i32 = matches
            .value_of("optimization")
            .unwrap_or("0")
            .parse()
            .unwrap();
        let opt = match opti {
            0 => OptimizationLevel::None,
            1 => OptimizationLevel::Aggressive,
            _ => OptimizationLevel::Default,
        };
        let target = matches.value_of("target").unwrap_or("x86-64");

        let output_value = matches.value_of("outputtype").unwrap_or("jit");
        let output = match output_value {
            "bin" => OutputType::Binary,
            "llvm-ir" => OutputType::LlvmIR,
            "object" => OutputType::Object,
            "asm" => OutputType::Assembly,
            "jit" => OutputType::Jit,
            _ => panic!("Invalid output {}.", output_value),
        };

        let config = DriverConfig {
            file: PathBuf::from(matches.value_of("file").unwrap()),
            type_check: typecheck,
            output_ty: output,
            only_lex: matches.is_present("lex"),
            optimization: opt,
            target: target.to_string(),
            graph: matches.occurrences_of("control flow graph"),
        };
        Driver { config }
    }
    pub fn run(&self) {
        let source =
            std::fs::read_to_string(&self.config.file).expect("Main file couldn't be found");
        let lexer = Lexer::new(&source);
        if self.config.only_lex {
            Self::print_lexer(lexer);
            return;
        }
        let parser = Parser::new(
            lexer,
            self.config.file.file_stem().unwrap().to_str().unwrap(),
            &source,
            self.config.file.clone(),
        );
        let (_status, _name, program) = parser.do_parse();

        match program {
            None => {
                println!("Failed to parse.");
                return;
            }
            Some(program) => {
                let program = ca_uir::to_program(&program);
                let ctx = Context::create();
                let compiler = Compiler::new_compiler(
                    &ctx,
                    self.config.optimization,
                    self.config.target.to_string(),
                );
                if self.config.type_check {
                    let tc = ca_tc::TypeChecker::new(&program);
                    tc.check_program(&program);
                }
                compiler.compile_program(&program);

                let memory_buffer =
                    MemoryBuffer::create_from_file(&PathBuf::from("./std.ll")).unwrap();
                let std_module = compiler
                    .context
                    .create_module_from_ir(memory_buffer)
                    .unwrap();
                compiler.module.link_in_module(std_module).unwrap();
                match self.config.graph {
                    1 => compiler
                        .main_function
                        .borrow()
                        .unwrap()
                        .view_function_cfg_only(),
                    2 => compiler.main_function.borrow().unwrap().view_function_cfg(),

                    0 | _ => {}
                }
                match compiler.module.verify() {
                    Ok(_) => match self.config.output_ty {
                        OutputType::LlvmIR => {
                            // Just printing for now, not much use in writing the IR to disk
                            let ir = compiler.module.print_to_string();
                            let mut path = PathBuf::from(self.config.file.file_stem().unwrap());
                            path.set_extension("ll");
                            std::fs::write(path, ir.to_string()).unwrap();
                            println!("{}", ir.to_string())
                        }
                        OutputType::Binary => {
                            compiler.write_object_file(&PathBuf::from("build/out.o"));
                            std::process::Command::new("clang")
                                .arg("./build/out.o")
                                .arg("-o")
                                .arg(self.config.file.file_stem().unwrap())
                                .spawn()
                                .unwrap()
                                .wait()
                                .unwrap();
                            std::fs::remove_file("./build/out.o").unwrap();
                        }
                        OutputType::Object => {
                            compiler.write_object_file(&PathBuf::from("build/out.o"));
                        }
                        OutputType::Assembly => {
                            compiler.write_assembly_file(&PathBuf::from("build/out.s"))
                        }
                        OutputType::Jit => {
                            let main: JitFunction<
                                unsafe extern "C" fn(i32, *const *const u8) -> i32,
                            > = unsafe { compiler.execution_engine.get_function("main").unwrap() };
                            unsafe {
                                main.call(1, ["program".as_ptr()].as_ptr());
                            }
                        }
                    },
                    Err(e) => {
                        compiler.module.print_to_stderr();
                        eprintln!("LLVM Error during compilation: {}", e);
                    }
                }
            }
        }
    }
    fn print_lexer(lexer: Lexer) {
        for token in lexer {
            if token.token_type == Lexer::YYEOF {
                break;
            }
            println!(
                "{:>15} := {:<20} @ {:>2}..{:>2}  ",
                ca_parser_bison::parser::token_name(token.token_type),
                token.token_value.escape_default().collect::<String>(),
                token.loc.begin,
                token.loc.end
            );
        }
    }
}
