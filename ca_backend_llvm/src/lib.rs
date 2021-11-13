use ca_ast::Program;
pub use inkwell;
use inkwell::{
    builder::Builder, context::Context, execution_engine::ExecutionEngine, module::Module,
    OptimizationLevel,
};

pub struct Compiler<'a> {
    pub module: Module<'a>,
    pub context: &'a Context,
    pub builder: Builder<'a>,
    pub execution_engine: ExecutionEngine<'a>,
    pub program: ca_ast::Program,
}

impl<'a> Compiler<'a> {
    pub fn new_compiler<'b>(p: Program, context: &'b Context) -> Compiler<'b> {
        let module = context.create_module("module");
        let execution_engine = module
            .create_jit_execution_engine(OptimizationLevel::None)
            .unwrap();
        let builder = context.create_builder();
        let compiler = Compiler {
            context,
            module,
            builder,
            execution_engine,
            program: p,
        };
        compiler
    }
}
