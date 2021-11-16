fn main() {
    let driver = ca_driver::Driver::new();
    driver.run();
}
#[test]
fn test_add() {
    use ca_parser_bison::{lexer::Lexer, parser::Parser};
    use std::path::PathBuf;
    let source = include_str!("../examples/tests/add.ca");
    let lexer = Lexer::new(source);
    let parser = Parser::new(lexer, "Cheese", source, PathBuf::new());
    let value = parser.do_parse().2.unwrap();
    let program = ca_uir::to_program(&value);

    let ctx = ca_backend_llvm::inkwell::context::Context::create();
    let compiler = ca_backend_llvm::Compiler::new_compiler(&ctx, ca_backend_llvm::inkwell::OptimizationLevel::Aggressive);
    compiler.compile_program(&program);

    unsafe {
        let add: ca_backend_llvm::inkwell::execution_engine::JitFunction<
            unsafe extern "C" fn(i32, i32) -> i32,
        > = compiler.execution_engine.get_function("add").unwrap();
        assert_eq!(add.call(1, 2), 1 + 2);
        assert_eq!(add.call(-1, 2), -1 + 2);
    }
}
