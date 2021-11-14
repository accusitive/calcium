use ca_uir::{Expression, Function, Identifier, Path, Program, Statement, Struct, StructField, Ty};
use debug_cell::RefCell;
pub use inkwell;
use inkwell::{
    builder::Builder,
    context::Context,
    execution_engine::ExecutionEngine,
    module::Module,
    types::{BasicType, BasicTypeEnum, FunctionType, StructType},
    values::{BasicValue, BasicValueEnum, FunctionValue, IntValue},
    OptimizationLevel,
};
use std::{
    borrow::{Borrow, BorrowMut},
    cell::Ref,
    collections::HashMap,
    default,
    rc::Rc,
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
    pub struct_fields: RefCell<HashMap<&'a Identifier, u32>>,
}
#[derive(Debug, Clone)]
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
            struct_fields: RefCell::new(HashMap::new()),
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
        let func_name = format!("{}__{}", self.prefixes.borrow().last().unwrap(), f.name);

        let func = match self.module.get_function(&func_name) {
            Some(stub) => {
                println!("Found stub function. Fixing");
                let func = self.module.add_function(&(func_name + ""), fnty, None);
                stub.replace_all_uses_with(func);
                unsafe { stub.delete() };
                func
            }
            None => self.module.add_function(&func_name, fnty, None),
        };

        let entry = self.context.append_basic_block(func, "entry");
        self.builder.position_at_end(entry);

        for (arg, i) in f.args.iter().zip(0..) {
            let depth_borrow = {
                let d = self.depth.borrow();
                *d
            };
            let mut mref = self.locals.borrow_mut();
            mref.push(Local {
                depth: depth_borrow,
                name: arg.name.to_string(),
                value: func.get_nth_param(i).unwrap(),
            });
    
        }

        
        
        self.compile_expression(&f.body);

        let mut d = self.depth.borrow_mut();
        {
            let mut borrow = self.locals.borrow_mut();
            let (_drained, remaining): (Vec<&Local>, _) =
                borrow.iter().partition(|l| l.depth >= *d);
            let remaining_owned: Vec<Local> = remaining
                .clone()
                .into_iter()
                .map(|l| l.to_owned())
                .collect();
            *borrow = remaining_owned;
        }
        *d -= 1;
    }
    pub fn get_or_stub_function(&self, name: &str) -> FunctionValue<'a> {
        match self.module.get_function(name) {
            Some(func) => func,
            None => {
                let ty = self.context.i32_type().fn_type(&[], false);
                self.module.add_function(name, ty, None)
            }
        }
    }
    pub fn compile_expression(&self, e: &'a Expression) -> Option<BasicValueEnum<'a>> {
        match e {
            Expression::Call(function_name, args) => {
                // let func = self.get_or_stub_function(&Self::path_to_s(function_name), ty);
                // let func = self
                //     .module
                //     .get_function(&Self::path_to_s(function_name))
                //     .expect("Unknown function");
                let func = self.get_or_stub_function(&Self::path_to_s(function_name));
                let a = args
                    .iter()
                    .map(|a| self.compile_expression(a).unwrap())
                    .collect::<Vec<_>>();

                Some(
                    self.builder
                        .build_call(func, &a, "call")
                        .try_as_basic_value()
                        .expect_left("Failed to unwrap left."),
                )
            }
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
                let compiled = stmts
                    .iter()
                    .map(|s| self.compile_statement(s))
                    .collect::<Vec<_>>();
                // println!("Everything is done borrowing. {:?}", self.locals);
                // borrow_mut.pop();
                // let mut borrow_mut = ;
                let mut d = self.depth.borrow_mut();
                {
                    let mut borrow = self.locals.borrow_mut();
                    let (_drained, remaining): (Vec<&Local>, _) =
                        borrow.iter().partition(|l| l.depth >= *d);
                    let remaining_owned: Vec<Local> = remaining
                        .clone()
                        .into_iter()
                        .map(|l| l.to_owned())
                        .collect();
                    *borrow = remaining_owned;
                }
                *d -= 1;
                // let mut to_remove = vec![];
                // for (local, index) in (*self.locals.borrow_mut()).iter().zip(0..) {
                //     if local.depth < *d {
                //         to_remove.push(index);
                //     }
                // }
                // for index in to_remove {
                //     self.locals.borrow_mut().remove(index);
                // }
                None
            }
            Expression::Path(p) => {
                let borrow = { self.locals.borrow() };
                let var = borrow
                    .iter()
                    .find(|l| l.name == Self::path_to_s(p))
                    .expect(&format!("Variable `{}` not found.", Self::path_to_s(p)));
                Some(var.value)
            }
            Expression::New(p, args) => {
                let p = self
                    .structs
                    .borrow()
                    .get(&Self::path_to_s(p))
                    .expect("Garbage")
                    .as_basic_type_enum();
                let memory = self.builder.build_malloc(p, "malloc.for.new").unwrap();
                println!("memory is {:#?}", memory);
                let values = args
                    .iter()
                    .map(|e| self.compile_expression(e).unwrap())
                    .collect::<Vec<_>>();
                let val = p.into_struct_type().const_named_struct(&values);
                self.builder.build_store(memory, val);

                let l = self.builder.build_load(memory, "l");
                // let second_field = unsafe { self.builder.build_struct_gep(memory, 1, "second") };
                // let second_field = self.builder.build_load(second_field, "second_field");

                Some(l)
            }
            Expression::FieldExpr(e, i) => {
                // path
                // self.locals.borrow().iter().find(|l| l.name )
                match &**e {
                    Expression::Path(_) => {}
                    _ => panic!("Field expr must be on a path."),
                }
                let compiled_expression = self.compile_expression(e).unwrap();
                println!("Compiled expression: {:#?}", compiled_expression);
                let compiled_expression_struct_type =
                    compiled_expression.get_type().into_struct_type();
                let compiled_expression_struct_value = compiled_expression.into_struct_value();
                let compiled_expression_struct_ptr = self
                    .builder
                    .build_malloc(compiled_expression_struct_type, "temp")
                    .unwrap();
                self.builder.build_store(
                    compiled_expression_struct_ptr,
                    compiled_expression_struct_value,
                );
                let borrow = self.struct_fields.borrow();
                println!("Borrow {:?}", *borrow);
                let index = borrow.get(i).unwrap();
                let accessed_field_ptr = unsafe {
                    self.builder.build_struct_gep(
                        compiled_expression_struct_ptr,
                        *index,
                        "fieldexpr",
                    )
                };
                let accessed_field = self.builder.build_load(accessed_field_ptr, "z");
                Some(accessed_field)
            }
        }
    }
    pub fn compile_statement(&self, s: &'a Statement) {
        match s {
            Statement::Let(name, ty, value) => {
                // let mut borrow_mut = self.locals.borrow_mut();
                let depth_borrow = {
                    let d = self.depth.borrow();
                    *d
                };
                let e = self.compile_expression(value);
                if e.is_some() {
                    let mut mref = self.locals.borrow_mut();
                    mref.push(Local {
                        depth: depth_borrow,
                        name: name.to_string(),
                        value: e.unwrap(),
                    });
                }
            }
            Statement::Return(e) => {
                let result = self.compile_expression(e).unwrap();
                self.builder.build_return(Some(&result));
            }
            Statement::Expr(e) => {
                self.compile_expression(e);
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
            {
                let mut b = self.struct_fields.borrow_mut();
                for (arg, index) in s.fields.iter().zip(0..) {
                    b.insert(&arg.name, index);
                    
                }
            }
        let mut borrow_mut = self.structs.borrow_mut();
        let s = borrow_mut
            .entry(format!("{}", s.name))
            .or_insert(self.context.struct_type(&field_types, true));
        if s.is_opaque() {
            s.set_body(&field_types, true);
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
            Ty::Pointer(p) => inkwell::types::BasicTypeEnum::PointerType(
                self.compile_ty(p).ptr_type(inkwell::AddressSpace::Generic),
            ),
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
