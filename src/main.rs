use std::{cell::RefCell, collections::HashMap, path::Path};

use ca_ast::Program;
use ca_backend_llvm::{
    inkwell::{
        execution_engine::JitFunction,
        targets::{CodeModel, FileType, RelocMode, Target, TargetTriple},
        OptimizationLevel,
    },
    Compiler,
};
fn main() {
    let w = std::fs::read_to_string("./example.ca").unwrap();
    let p: Program = ca_parser::gram::ProgramParser::new().parse(&w).unwrap();
    println!("Program {:#?}", p);
    let context = ca_backend_llvm::inkwell::context::Context::create();
    let module = context.create_module("module");
    let execution_engine = module
        .create_jit_execution_engine(ca_backend_llvm::inkwell::OptimizationLevel::None)
        .unwrap();
    let builder = context.create_builder();
    let compiler = Compiler {
        context: &context,
        module: module,
        builder: builder,
        execution_engine: execution_engine,
        program: p,
        types: RefCell::new(HashMap::new()),
        function_arg_names: RefCell::new(HashMap::new()),
        current_function: Default::default(),
        depth: RefCell::new(0),
        local_variables: RefCell::new(Vec::new()),
        functions: RefCell::new(HashMap::new()),
    };
    compiler.pre_compile();
    compiler.compile_program();
    compiler.module.print_to_stderr();

    match compiler.module.verify() {
        Ok(_) => {
            let good_target = Target::from_name("x86-64").unwrap();
            let target_machine = good_target
                .create_target_machine(
                    &TargetTriple::create("x86_64-pc-linux-gnu"),
                    "x86-64",
                    "+avx2",
                    OptimizationLevel::Default,
                    RelocMode::Default,
                    CodeModel::Default,
                )
                .unwrap();
            target_machine
                .write_to_file(&compiler.module, FileType::Object, Path::new("./out.o"))
                .unwrap();
        }
        Err(e) => println!("Error could not validate {}", e),
    };

    // unsafe {
    //     let f: JitFunction<unsafe extern "C" fn(i32, i32) -> i32> =
    //         compiler.execution_engine.get_function("comp").unwrap();
    //     // println!("Result {:#?}", f.call(2, 4));
    //     assert_eq!(f.call(1, 100), 2);
    //     assert_eq!(f.call(100, 1), 4);
    // }
    unsafe {
        let entry: JitFunction<unsafe extern "C" fn() -> i32> =
            compiler.execution_engine.get_function("entry").unwrap();
        println!("Program exited with value: {}", entry.call());
    };
}
#[cfg(test)]
mod tests {
    use ca_backend_llvm::{
        inkwell::{
            context::Context,
            execution_engine::{JitFunction, UnsafeFunctionPointer},
        },
        Compiler,
    };
    fn framework<'a, F: UnsafeFunctionPointer>(
        source: &'a str,
        function_name: &'a str,
    ) -> JitFunction<F> {
        let program = ca_parser::gram::ProgramParser::new().parse(source).unwrap();
        let context = &Context::create();
        let compiler = Compiler::new_compiler(program, context);
        compiler.pre_compile();
        compiler.compile_program();
        let f: JitFunction<F> =
            unsafe { compiler.execution_engine.get_function(function_name) }.unwrap();
        f
    }
    #[test]
    fn test_add() {
        let w = include_str!("../tests/add.ca");
        println!("before frame");
        let add = framework::<unsafe extern "C" fn(i32, i32) -> i32>(w, "add");
        println!("after frame");

        unsafe {
            assert_eq!(add.call(2, 4), 2 + 4);
            assert_eq!(add.call(2, 4), 2 + 4);
        }
    }
}
