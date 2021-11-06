#![feature(drain_filter)]
use std::{cell::RefCell, collections::HashMap, sync::Arc};

use ca_ast::{Expr, Function, FunctionArg, Item, Program, Stmt, Struct, Ty};
pub use inkwell;
use inkwell::{OptimizationLevel, builder::Builder, context::{Context, ContextRef}, execution_engine::ExecutionEngine, module::Module, types::{BasicType, BasicTypeEnum, StructType}, values::{BasicValueEnum, FunctionValue}};
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

pub struct LocalVariable<'a> {
    value: BasicValueEnum<'a>,
    depth: u32,
    name: String,
}
impl<'a> Compiler<'a> {
    pub fn new_compiler<'b> (p: Program, context: &'b Context) -> Compiler<'b>{
        // let context = Context::create();
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
        // compiler.pre_compile();
        // compiler.compile_program();
        // compiler.module.print_to_stderr();
        compiler
    }
    pub fn compile_expr(&self, e: &Expr) -> BasicValueEnum<'a> {
        match e {
            Expr::LiteralExpr(l) => match l {
                ca_ast::Literal::String(_) => todo!(),
                ca_ast::Literal::Char(_) => todo!(),
                ca_ast::Literal::Integer(i) => inkwell::values::BasicValueEnum::IntValue(
                    self.context
                        .i32_type()
                        .const_int((*i).try_into().unwrap(), false),
                ),
            },

            Expr::Block(s) => {
                *self.depth.borrow_mut() += 1;
                let compiled = s.iter().map(|s| self.compile_stmt(s)).collect::<Vec<_>>();
                for stmt in s {
                    self.compile_stmt(stmt);
                }
                *self.depth.borrow_mut() -= 1;
                self.local_variables
                    .borrow_mut()
                    .drain_filter(|lv| lv.depth >= *self.depth.borrow())
                    .for_each(drop);
                *compiled.last().unwrap_or(&inkwell::values::BasicValueEnum::StructValue(self.unit_ty().const_named_struct(&[])))
                
            }
            Expr::BinOp(l, op, r) => {
                let left = self.compile_expr(l);
                let right = self.compile_expr(r);
                let result = match op {
                    ca_ast::Op::LShift => todo!(),
                    ca_ast::Op::RShift => todo!(),
                    ca_ast::Op::Mul => todo!(),
                    ca_ast::Op::Div => todo!(),
                    ca_ast::Op::Sub => self.builder.build_int_sub(
                        left.into_int_value(),
                        right.into_int_value(),
                        "sub",
                    ),
                    ca_ast::Op::Add => self.builder.build_int_add(
                        left.into_int_value(),
                        right.into_int_value(),
                        "add",
                    ),
                    ca_ast::Op::Xor => todo!(),
                    ca_ast::Op::And => todo!(),
                    ca_ast::Op::Or => todo!(),
                    ca_ast::Op::Lt => self.builder.build_int_compare(
                        inkwell::IntPredicate::SLT,
                        left.into_int_value(),
                        right.into_int_value(),
                        "slt",
                    ),

                    ca_ast::Op::Gt => self.builder.build_int_compare(
                        inkwell::IntPredicate::SGT,
                        left.into_int_value(),
                        right.into_int_value(),
                        "sgt",
                    ),
                };
                inkwell::values::BasicValueEnum::IntValue(result)
            }
            Expr::Ident(i) => {
                let borrow = self.local_variables.borrow();
                let lv = borrow.iter().find(|v| &v.name == i);
                match lv {
                    Some(v) => v.value,
                    None => {
                        let borrow_mut = self.function_arg_names.borrow_mut();
                        let current_function_value = self.current_function.borrow().unwrap();
                        let current_function_name = current_function_value.get_name().to_str().unwrap();
                        let index = borrow_mut.get(current_function_name).unwrap().get(i).expect(&format!(
                            "No local variable or function parameter `{}` found.",
                            i
                        ));
                        let fa = self
                            .current_function
                            .borrow()
                            .unwrap()
                            .get_nth_param(*index)
                            .unwrap();
                        fa
                    }
                }
            }
            Expr::UnaryOp(_, _) => todo!(),
            Expr::Call(name, args) => {
                let borrow = self.functions.borrow();
                let f = borrow
                    .get(name)
                    .expect(&format!("Function `{}` not found.", name));
                let args = args
                    .iter()
                    .map(|e| self.compile_expr(e))
                    .collect::<Vec<_>>();
                self.builder
                    .build_call(*f, &args, "call()")
                    .try_as_basic_value()
                    .left()
                    .unwrap()
            }
            Expr::If(predicate, then, elze) => {
                let cmp = self.compile_expr(predicate);
                let function = self.current_function.borrow().unwrap();
                let entry = function.get_last_basic_block().unwrap();

                let then_block = self.context.append_basic_block(function, "then");
                let elze_block = self.context.append_basic_block(function, "elze");
                let cont = self.context.append_basic_block(function, "cont");


                self.builder.position_at_end(then_block);
                let thenv = self.compile_expr(then);
                println!("ThenV {:#?}", then);
                self.builder.build_unconditional_branch(cont);

                self.builder.position_at_end(elze_block);
                let elzev = self.compile_expr(elze);
                println!("elzev {:#?}", elze);

                self.builder.build_unconditional_branch(cont);

                self.builder.position_at_end(entry);

                self.builder.build_conditional_branch(cmp.into_int_value(), then_block, elze_block);
                
                self.builder.position_at_end(cont);

                let pn = self.builder.build_phi(self.context.i32_type(), "pn");
                pn.add_incoming(&[(&thenv, then_block)]);
                pn.add_incoming(&[(&elzev, elze_block)]);

                pn.as_basic_value()
                // BasicValueEnum::StructValue(self.unit_ty().const_named_struct(&[]))
            }
            _ => todo!(),
        }
    }
    pub fn compile_stmt(&self, stmt: &Stmt) -> BasicValueEnum<'a> {
        match stmt {
            Stmt::LetStmt { name, ty: _, value } => {
                let compiled_expr = self.compile_expr(value);
                self.local_variables.borrow_mut().push(LocalVariable {
                    value: compiled_expr,
                    depth: *self.depth.borrow(),
                    name: name.to_string(),
                });
                compiled_expr
            }
            Stmt::ExprStmt(e) => {
                let compiled_expr = self.compile_expr(e);
                compiled_expr
            }
            Stmt::Return(r) => {
                println!("Building Expr {:#?}", r);

                let compile_expr = self.compile_expr(r);
                println!("Building return {:#?}", compile_expr);
                self.builder.build_return(Some(&compile_expr));
                BasicValueEnum::StructValue(self.unit_ty().const_named_struct(&[]))
            }
        }
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
        // self.current_function.replace(Some(func));
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
            Ty::Bool => inkwell::types::BasicTypeEnum::IntType(self.context.bool_type())
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
