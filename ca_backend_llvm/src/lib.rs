use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
    default,
    rc::Rc,
};

use ca_uir::{Expression, Function, Identifier, Path, Program, Statement, Struct, StructField, Ty};
pub use inkwell;
use inkwell::{
    builder::Builder,
    context::Context,
    execution_engine::ExecutionEngine,
    module::Module,
    types::{BasicType, BasicTypeEnum, StructType},
    values::{BasicValue, BasicValueEnum},
    OptimizationLevel,
};

pub struct Compiler<'a> {
    pub module: Module<'a>,
    pub context: &'a Context,
    pub builder: Builder<'a>,
    pub execution_engine: ExecutionEngine<'a>,
    /// FIFO vector to keep track of what prefix to use for item names.
    pub prefixes: RefCell<Vec<String>>,
    pub locals: RefCell<Vec<Local<'a>>>,
    pub structs: RefCell<HashMap<String, StructType<'a>>>,
    pub depth: RefCell<i32>,
}
pub struct Local<'a> {
    depth: i32,
    name: String,
    value: BasicValueEnum<'a>,
}

impl<'a> Compiler<'a> {
    pub fn new_compiler<'b>(context: &'b Context) -> Compiler<'b> {
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
            prefixes: RefCell::new(vec!["cheese".to_string()]),
            structs: RefCell::new(HashMap::new()),
            locals: RefCell::new(Vec::new()),
            depth: RefCell::new(0),
        };
        compiler
    }
    pub fn compile_program(&self, p: &'a Program) {
        for item in &p.items {
            match item {
                ca_uir::Item::Function(f) => self.compile_function(f),
                ca_uir::Item::Struct(s) => self.compile_struct(s),
                ca_uir::Item::Import(i) => {
                    self.prefixes.borrow_mut().push(i.ident.0.clone());
                    self.compile_program(&i.prog);
                    self.prefixes.borrow_mut().pop();
                }
            }
        }
    }
    pub fn compile_function(&self, f: &'a Function) {
        let ty = self.compile_ty(&f.return_ty);
        let args = f
            .args
            .iter()
            .map(|a| self.compile_ty(&a.ty))
            .collect::<Vec<_>>();
        let fnty = ty.fn_type(&args, false);
        let func = self.module.add_function(
            &format!("{}__{}", self.prefixes.borrow().last().unwrap(), f.name),
            fnty,
            None,
        );
        let entry = self.context.append_basic_block(func, "entry");
        self.builder.position_at_end(entry);
        self.compile_expression(&f.body);
    }
    pub fn compile_expression(&self, e: &'a Expression) -> Option<BasicValueEnum<'a>> {
        match e {
            Expression::Call(_, _) => todo!(),
            Expression::Arith(left, op, right) => {
                let l = self.compile_expression(left).unwrap();
                let r = self.compile_expression(right).unwrap();
                Some(
                    self.builder
                        .build_int_add(l.into_int_value(), r.into_int_value(), "add")
                        .as_basic_value_enum(),
                )
            }
            Expression::Literal(num) => Some(inkwell::values::BasicValueEnum::IntValue(
                self.context
                    .i32_type()
                    .const_int((*num).try_into().unwrap(), false),
            )),
            Expression::Block(stmts) => {
                {
                    let mut d = self.depth.borrow_mut();
                    *d += 1;
                }

                // borrow_mut.push(Local{
                //     depth: *d,
                //     variables: HashMap::new()
                // });
                stmts.iter().for_each(|s| self.compile_statement(s));

                // borrow_mut.pop();
                let mut borrow_mut = self.locals.borrow_mut();
                let mut d = self.depth.borrow_mut();

                *d -= 1;
                let mut to_remove = vec![];
                for (local, index) in (*borrow_mut).iter().zip(0..) {
                    if local.depth < *d {
                        to_remove.push(index);
                    }
                }
                for index in to_remove {
                    borrow_mut.remove(index);
                }
                None
            }
            Expression::Path(p) => {
                let borrow = self.locals.borrow();
                let var = borrow
                    .iter()
                    .find(|l| l.name == Self::path_to_s(p))
                    .expect("Invald var reference.");
                Some(var.value)
            }
        }
    }
    pub fn compile_statement(&self, s: &'a Statement) {
        match s {
            Statement::Let(name, ty, value) => {
                let mut borrow_mut = self.locals.borrow_mut();
                borrow_mut.push(Local {
                    depth: *self.depth.borrow(),
                    name: name.to_string(),
                    value: self.compile_expression(value).unwrap(),
                })
            }
            Statement::Return(e) => {
                let result = self.compile_expression(e).unwrap();
                self.builder.build_return(Some(&result));
            }
        }
    }
    pub fn compile_struct(&self, s: &'a Struct) {
        let field_types = s
            .fields
            .iter()
            .map(|f| {
                // self.structs.insert(s.name, v)
                self.compile_ty(&f.ty)
            })
            .collect::<Vec<_>>();

        let mut borrow_mut = self.structs.borrow_mut();
        let s = borrow_mut
            .entry(format!("{}", s.name))
            .or_insert(self.context.struct_type(&field_types, false));
        if s.is_opaque() {
            s.set_body(&field_types, false);
            // println!("Was opaque!");
        }

        // println!("s {:#?}", self.structs);
    }
    // pub fn compile_struct_field(&self, s: &StructField) {

    // }
    pub fn compile_ty(&self, t: &'a Ty) -> BasicTypeEnum<'a> {
        match t {
            Ty::Named(path) => {
                let s = self
                    .structs
                    .borrow_mut()
                    .entry(Self::path_to_s(path))
                    .or_insert(self.context.opaque_struct_type(&Self::path_to_s(path)));

                self.structs
                    .borrow_mut()
                    .get(&Self::path_to_s(path))
                    .unwrap()
                    .as_basic_type_enum()
            }
            Ty::Int32 => inkwell::types::BasicTypeEnum::IntType(self.context.i32_type()),
            Ty::Infer => todo!(),
        }
    }
    pub fn path_to_s(p: &Path) -> String {
        p.parts
            .iter()
            .map(|p| p.0.clone())
            .collect::<Vec<_>>()
            .join("__")
    }
}
