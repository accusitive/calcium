mod expr;
mod stmt;

use std::{cell::RefCell, collections::HashMap};

use ca_ast::{Expr, Function, FunctionArg, Item, Program, Struct, Ty};
pub use inkwell;
use inkwell::{
    builder::Builder,
    context::Context,
    execution_engine::ExecutionEngine,
    module::Module,
    types::{BasicType, BasicTypeEnum, StructType},
    values::{BasicValueEnum, FunctionValue},
    OptimizationLevel,
};
pub struct Compiler<'a> {
    pub module: Module<'a>,
    pub context: &'a Context,
    pub builder: Builder<'a>,

    pub execution_engine: ExecutionEngine<'a>,
    pub program: ca_ast::Program,
    pub types: RefCell<HashMap<String, BasicTypeEnum<'a>>>,
    pub function_arg_names: RefCell<HashMap<String, HashMap<String, u32>>>,
    pub current_function: RefCell<Option<FunctionValue<'a>>>,
    pub local_variables: RefCell<Vec<LocalVariable<'a>>>,
    pub depth: RefCell<u32>,
    pub functions: RefCell<HashMap<String, FunctionValue<'a>>>,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LocalVariable<'a> {
    value: BasicValueEnum<'a>,
    depth: u32,
    name: String,
}
impl<'a> Compiler<'a> {
    pub fn new_compiler<'b>(p: Program, context: &'b Context) -> Compiler<'b> {
        let module = context.create_module("module");
        let execution_engine = module
            .create_jit_execution_engine(OptimizationLevel::None)
            .unwrap();
        let builder = context.create_builder();
        let compiler = Compiler {
            context: context,
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
        compiler
    }

    pub fn compile_item(&self, item: &Item) {
        match item {
            Item::Function(f) => self.compile_function(f),
            Item::Struct(s) => self.compile_struct(s),
        }
    }
    fn get_function_ty(&self, f: &Function) -> FunctionValue<'a> {
        let ty = f
            .args
            .iter()
            .map(|a| self.compile_function_arg(a))
            .collect::<Vec<_>>();

        f.args.iter().zip(0..f.args.len()).for_each(|(fa, i)| {
            self.function_arg_names
                .borrow_mut()
                .entry(f.name.clone())
                .or_default()
                .entry(fa.ident.clone())
                .or_insert(i.try_into().unwrap());
        });
        let ret = self.compile_ty(&f.ty);
        let fnty = ret.fn_type(&ty, false);
        match self.module.get_function(&f.name) {
            Some(f) => f,
            None => {
                let func = self.module.add_function(&f.name, fnty, None);
                func
            }
        }
    }
    pub fn compile_function(&self, f: &Function) {
        let func = self.get_function_ty(f);
        let entry = self.context.append_basic_block(func, "entry");
        *self.current_function.borrow_mut() = Some(func);
        self.builder.position_at_end(entry);
        if let Expr::Block(b) = &f.body {
            for stmt in b {
                self.compile_stmt(stmt);
            }
        }
    }
    pub fn compile_function_arg(&self, fa: &FunctionArg) -> BasicTypeEnum<'a> {
        self.compile_ty(&fa.ty)
    }
    pub fn compile_ty(&self, ty: &Ty) -> BasicTypeEnum<'a> {
        match ty {
            Ty::Named(s) => {
                let v =
                    inkwell::types::BasicTypeEnum::StructType(self.context.opaque_struct_type(&s));
                self.types.borrow_mut().entry(s.clone()).or_insert(v);
                v
            }
            Ty::Int32 => inkwell::types::BasicTypeEnum::IntType(self.context.i32_type()),
            Ty::Unit => inkwell::types::BasicTypeEnum::StructType(self.unit_ty()),
            Ty::Bool => inkwell::types::BasicTypeEnum::IntType(self.context.bool_type()),
        }
    }

    pub fn compile_struct(&self, f: &Struct) {
        let fields = f
            .fields
            .iter()
            .map(|f| self.compile_ty(&f.ty))
            .collect::<Vec<_>>();

        self.types
            .borrow_mut()
            .entry(f.name.clone())
            .or_insert(inkwell::types::BasicTypeEnum::StructType(
                self.context.struct_type(&fields, false),
            ))
            .into_struct_type();

        self.types
            .borrow_mut()
            .entry(f.name.clone())
            .and_modify(|f| {
                f.into_struct_type().set_body(&fields, false);
            });
    }
    pub fn compile_program(&self) -> Option<i32> {
        for item in &self.program.items {
            self.compile_item(item);
        }
        None
    }
    pub fn unit_ty(&self) -> StructType<'a> {
        self.types
            .borrow_mut()
            .entry("()".to_string())
            .or_insert(inkwell::types::BasicTypeEnum::StructType(
                self.context.struct_type(&[], false),
            ))
            .into_struct_type()
    }
    pub fn pre_compile(&self) {
        for item in &self.program.items {
            if let Item::Function(f) = item {
                self.functions
                    .borrow_mut()
                    .insert(f.name.clone(), self.get_function_ty(f));
            }
        }
    }
}
