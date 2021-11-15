use std::path::PathBuf;

use ca_backend_llvm::{inkwell::context::Context, Compiler};
use ca_parser_bison::{lexer::Lexer, parser::Parser};
use clap::{App, Arg};
#[derive(Debug)]
pub enum OutputType {
    LLVMIR,
    BINARY,
    OBJECT,
}
#[derive(Debug)]
pub struct DriverConfig {
    file: PathBuf,
    type_check: bool,
    output_ty: OutputType,
    only_lex: bool,
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
                    .possible_values(&["llvm-ir", "object", "bin"])
                    .takes_value(true)
                    .max_values(1),
            )
            .get_matches();

        let typecheck = match matches.values_of("experimental") {
            Some(mut values) => {
                println!("Values {:#?}", values);

                values.find(|v| v == &"typecheck").is_some()
            }
            None => false,
        };

        let config = DriverConfig {
            file: PathBuf::from(matches.value_of("file").unwrap()),
            type_check: typecheck,
            output_ty: OutputType::BINARY,
            only_lex: matches.is_present("lex"),
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
                let compiler = Compiler::new_compiler(&ctx);
                compiler.compile_program(&program);
                match compiler.module.verify() {
                    Ok(_) => match self.config.output_ty {
                        OutputType::LLVMIR => {
                            // Just printing for now, not much use in writing the IR to disk
                            let ir = compiler.module.print_to_string();
                            println!("{}", ir)
                        }
                        OutputType::BINARY => {
                            compiler.write_object_file(&PathBuf::from("build/out.o"));
                            std::process::Command::new("clang")
                                .arg("std.c")
                                .arg("./build/out.o")
                                .arg("-o")
                                .arg(self.config.file.file_stem().unwrap())
                                .spawn()
                                .unwrap()
                                .wait()
                                .unwrap();
                            std::fs::remove_file("./build/out.o").unwrap();
                        }
                        OutputType::OBJECT => {
                            compiler.write_object_file(&PathBuf::from("build/out.o"));
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
                "{:>15} := {:<10} @ {:>2}..{:>2}  ",
                ca_parser_bison::parser::token_name(token.token_type),
                token.token_value,
                token.loc.begin,
                token.loc.end
            );
        }
    }
}
