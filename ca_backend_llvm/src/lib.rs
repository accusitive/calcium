use ca_uir::{Expression, Function, Identifier, Path, Program, Statement, Struct, Ty};
use debug_cell::RefCell;
pub use inkwell;
use inkwell::{
    builder::Builder,
    context::Context,
    execution_engine::ExecutionEngine,
    module::{Linkage, Module},
    targets::{
        CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine, TargetTriple,
    },
    types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum, StructType},
    values::{BasicMetadataValueEnum, BasicValue, BasicValueEnum, FunctionValue},
    IntPredicate, OptimizationLevel,
};
use std::{collections::HashMap, fmt::{Formatter, Pointer}, path::Path as StdPath};

pub struct Compiler<'a> {
    pub module: Module<'a>,
    pub context: &'a Context,
    pub builder: Builder<'a>,
    pub execution_engine: ExecutionEngine<'a>,
    pub optimization_level: OptimizationLevel,
    /// FIFO vector to keep track of what prefix to use for item names.
    pub prefixes: RefCell<Vec<String>>,
    pub locals: RefCell<Vec<Local<'a>>>,
    pub structs: RefCell<HashMap<String, StructType<'a>>>,
    pub depth: RefCell<i32>,
    pub struct_fields: RefCell<HashMap<&'a Identifier, u32>>,
    pub main_function: RefCell<Option<FunctionValue<'a>>>,
    pub target: String,
}
#[derive(Debug, Clone)]
pub struct Local<'a> {
    depth: i32,
    name: String,
    value: BasicValueEnum<'a>,
}

impl<'a> Compiler<'a> {
    pub fn new_compiler<'b>(
        context: &'b Context,
        optimization_level: OptimizationLevel,
        target: String,
    ) -> Compiler<'b> {
        let module = context.create_module("module");
        let execution_engine = module
            .create_jit_execution_engine(optimization_level)
            .unwrap();
        let builder = context.create_builder();
        let compiler = Compiler {
            context,
            module,
            builder,
            execution_engine,
            prefixes: RefCell::new(vec![]),
            structs: RefCell::new(HashMap::new()),
            locals: RefCell::new(Vec::new()),
            depth: RefCell::new(0),
            struct_fields: RefCell::new(HashMap::new()),
            main_function: RefCell::new(None),
            optimization_level,
            target: target,
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
                    println!("STARTING TO SUBCOMPILE {}", i.ident);
                    self.compile_program(&i.prog);
                    self.prefixes.borrow_mut().pop();
                }
            }
        }
    }
    fn ty_to_meta_ty(&self, t: BasicTypeEnum<'a>) -> BasicMetadataTypeEnum<'a> {
        match t {
            BasicTypeEnum::ArrayType(a) => BasicMetadataTypeEnum::ArrayType(a),
            BasicTypeEnum::FloatType(a) => BasicMetadataTypeEnum::FloatType(a),
            BasicTypeEnum::IntType(a) => BasicMetadataTypeEnum::IntType(a),
            BasicTypeEnum::PointerType(a) => BasicMetadataTypeEnum::PointerType(a),
            BasicTypeEnum::StructType(a) => BasicMetadataTypeEnum::StructType(a),
            BasicTypeEnum::VectorType(a) => BasicMetadataTypeEnum::VectorType(a),
        }
    }
    fn val_to_meta_val(&self, t: BasicValueEnum<'a>) -> BasicMetadataValueEnum<'a> {
        match t {
            BasicValueEnum::ArrayValue(a) => BasicMetadataValueEnum::ArrayValue(a),
            BasicValueEnum::FloatValue(a) => BasicMetadataValueEnum::FloatValue(a),
            BasicValueEnum::IntValue(a) => BasicMetadataValueEnum::IntValue(a),
            BasicValueEnum::PointerValue(a) => BasicMetadataValueEnum::PointerValue(a),
            BasicValueEnum::StructValue(a) => BasicMetadataValueEnum::StructValue(a),
            BasicValueEnum::VectorValue(a) => BasicMetadataValueEnum::VectorValue(a),
        }
    }
    pub fn compile_function(&self, f: &'a Function) {
        let ty = self.compile_ty(&f.return_ty);
        let args = f
            .args
            .iter()
            .map(|a| self.ty_to_meta_ty(self.compile_ty(&a.ty)))
            .collect::<Vec<_>>();
        // BasicMetadataTypeEnum
        let fnty = ty.fn_type(&args, f.is_varargs);
        // let func_name = format!("{}__{}", self.prefixes.borrow().last().unwrap(), f.name);
        let func_name = match (self.prefixes.borrow().last(), f.is_extern) {
            (_, ext) if ext => f.name.0.to_string(),
            (Some(p), _) => format!("{}__{}", p, f.name.0),
            (None, _) => f.name.0.to_string(),
        };

        let linkage = if f.is_extern {
            Some(Linkage::External)
        } else {
            None
        };
        let func = match self.module.get_function(&func_name) {
            Some(stub) => {
                let func = self.module.add_function(&(func_name + ""), fnty, linkage);
                stub.replace_all_uses_with(func);
                unsafe { stub.delete() };
                func
            }
            None => self.module.add_function(&func_name, fnty, linkage),
        };
        if &f.name.0 == "main" {
            let mut borrow = self.main_function.borrow_mut();
            if borrow.is_some() {
                panic!("Cannot define {} more than once.", f.name)
            } else {
                *borrow = Some(func);
            }
        }
        match &f.body {
            Some(body) => {
                let entry = self
                    .context
                    .append_basic_block(func, &format!("entry.{}", f.name));
                self.builder.position_at_end(entry);

                for (arg, i) in f.args.iter().zip(0..) {
                    let depth_borrow = {
                        let d = self.depth.borrow();
                        *d
                    };
                    let mut mref = self.locals.borrow_mut();
                    mref.push(Local {
                        depth: depth_borrow,
                        name: arg.name.0.to_string(),
                        value: func.get_nth_param(i).unwrap(),
                    });
                }

                self.compile_expression(body);
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
            None => {}
        }
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
                let func = self.get_or_stub_function(&Self::path_to_s(function_name));
                let a = args
                    .iter()
                    .map(|a| self.val_to_meta_val(self.compile_expression(a).unwrap()))
                    .collect::<Vec<_>>();

                Some(
                    self.builder
                        .build_call(func, &a, "call")
                        .try_as_basic_value()
                        .expect_left("Failed to unwrap left."),
                )
            }
            Expression::Arith(left, op, right) => {
                let l = self.compile_expression(left).unwrap().into_int_value();
                let r = self.compile_expression(right).unwrap().into_int_value();
                Some(match op {
                    ca_uir::Op::Add => self
                        .builder
                        .build_int_add(l, r, "add")
                        .as_basic_value_enum(),
                    ca_uir::Op::Sub => self
                        .builder
                        .build_int_sub(l, r, "add")
                        .as_basic_value_enum(),
                    ca_uir::Op::Mul => self
                        .builder
                        .build_int_mul(l, r, "add")
                        .as_basic_value_enum(),
                    ca_uir::Op::Div => self
                        .builder
                        .build_int_signed_div(l, r, "add")
                        .as_basic_value_enum(),
                    ca_uir::Op::Less | ca_uir::Op::Greater => panic!("Not possible."),
                })
            }
            Expression::Literal(lit) => Some(match lit {
                ca_uir::Literal::Number(n, ty) => {
                    let t = self.compile_ty(ty).into_int_type();
                    match ty {
                        Ty::Int32 => t
                            .const_int((*n).try_into().unwrap(), false)
                            .as_basic_value_enum(),
                        Ty::Int64 => t
                            .const_int((*n).try_into().unwrap(), false)
                            .as_basic_value_enum(),
                        Ty::UInt32 => t
                            .const_int((*n).try_into().unwrap(), false)
                            .as_basic_value_enum(),
                        Ty::UInt64 => t
                            .const_int((*n).try_into().unwrap(), false)
                            .as_basic_value_enum(),
                        _ => panic!("Bad literal ty"),
                    }
                }
                ca_uir::Literal::String(s) => {
                    let s = self.builder.build_global_string_ptr(&s, "global_string");

                    s.as_basic_value_enum()
                }
                ca_uir::Literal::Boolean(b) => self
                    .context
                    .bool_type()
                    .const_int(*b as u64, false)
                    .as_basic_value_enum(),
            }),
            Expression::Block(stmts) => {
                {
                    let mut d = self.depth.borrow_mut();
                    *d += 1;
                }

                stmts.iter().for_each(|s| self.compile_statement(s));

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
                let borrow = self.structs.borrow();
                let p = borrow.get(&Self::path_to_s(p)).expect("Garbage");

                let memory = self
                    .builder
                    .build_malloc(p.as_basic_type_enum(), "malloc.for.new")
                    .expect("Failed to malloc in constructor");

                let values = args
                    .iter()
                    .map(|e| self.compile_expression(e).unwrap())
                    .collect::<Vec<_>>();

                // Iterate over each parameter
                for (value, index) in values.iter().zip(0..) {
                    if index > p.count_fields() {
                        panic!(
                            "Cannot supply {} values to struct {}.",
                            values.len(),
                            p.count_fields()
                        )
                    }
                    let hmm = self.builder.build_struct_gep(memory, index, "gep").unwrap();
                    self.builder.build_store(hmm, *value);
                }

                let l = self.builder.build_load(memory, "l");
                Some(l)
            }
            Expression::FieldAccess(e, i) => {
                let compiled_expression = self.compile_expression(e).unwrap();

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

                let index = borrow.get(i).unwrap();
                let accessed_field_ptr = self
                    .builder
                    .build_struct_gep(compiled_expression_struct_ptr, *index, "fieldexpr")
                    .unwrap();
                let accessed_field = self.builder.build_load(accessed_field_ptr, "z");
                Some(accessed_field)
            }
            Expression::Logic(left, op, right) => {
                // TODO: Not only int and float math.
                let o = match op {
                    ca_uir::Op::Add | ca_uir::Op::Sub | ca_uir::Op::Mul | ca_uir::Op::Div => {
                        todo!()
                    }
                    ca_uir::Op::Less => IntPredicate::SLT,
                    ca_uir::Op::Greater => IntPredicate::SGT,
                    // _ => IntPredicate::EQ
                };
                let lhs = self.compile_expression(left).unwrap().into_int_value();
                let rhs = self.compile_expression(right).unwrap().into_int_value();
                Some(BasicValueEnum::IntValue(self.builder.build_int_compare(
                    o,
                    lhs,
                    rhs,
                    "logic.cmp",
                )))
            }
        }
    }
    pub fn compile_statement(&self, s: &'a Statement) {
        match s {
            Statement::Let(name, _ty, value) => {
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
                        name: name.0.to_string(),
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
            Statement::If(condition, body, elze) => {
                let cmp = self.compile_expression(condition).unwrap();
                let insert = self.builder.get_insert_block().unwrap();
                println!("IF ON {:#?}", insert);
                
                if insert.get_previous_basic_block().is_some()  && insert.get_previous_basic_block().unwrap().get_terminator().is_none() {
                    let prev = insert.get_previous_basic_block().unwrap();
                    println!("Yep {:?} -> {:?}", prev.get_name(), insert.get_name());

                    self.builder.position_at_end(prev);
                    self.builder.build_unconditional_branch(insert);
                    self.builder.position_at_end(insert);
                }
                // let insert_name = insert.get_name();
                let insert_name = "insert".to_string();
                // println!("INSERT IS {:?}", );
                let thenbb = self.context.insert_basic_block_after(
                    insert,
                    &format!("{}/then", insert_name.as_str()),
                );
                
                
                let contbb = self.context.insert_basic_block_after(
                    insert,
                    &format!("{}/cont", insert_name.as_str()),
                );
                if elze.is_some() {
                    let elzebb = self.context.insert_basic_block_after(
                        insert,
                        &format!("{}/else", insert_name.as_str()),
                    );
                    self.builder
                        .build_conditional_branch(cmp.into_int_value(), thenbb, elzebb);
                        self.builder.position_at_end(elzebb);
                        let e = elze.as_ref().unwrap();
                        self.compile_expression(&e);
                        self.builder.build_unconditional_branch(contbb);

                } else {
                    self.builder
                        .build_conditional_branch(cmp.into_int_value(), thenbb, contbb);
                }

                // Then block
                // let mut does_else_terminatea = false;
                // let does_then_terminate;

                self.builder.position_at_end(thenbb);
                self.compile_expression(body);
                // does_then_terminate = thenbb.get_terminator().is_some();
                println!(
                    "then {:#?} {}",
                    thenbb.get_terminator(),
                    thenbb.get_name().to_str().unwrap()
                );
                if thenbb.get_terminator().is_none() {
                    self.builder.build_unconditional_branch(contbb);
                }
                match elze {
                    Some(e) => {
                       

                        // if elzebb.get_terminator().is_none() {
                        // }
                    }
                    None => {
                        unsafe {  };
                    }
                }
                // Cont
                self.builder.position_at_end(contbb);

                // if does_then_terminate && does_else_terminatea {
                // self.builder.build_unreachable();
                // }
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
            .entry(format!("{}", s.name.0))
            .or_insert(self.context.struct_type(&field_types, true));

        if s.is_opaque() {
            s.set_body(&field_types, true);
        }
    }
    pub fn compile_ty(&self, t: &'a Ty) -> BasicTypeEnum<'a> {
        match t {
            Ty::Named(path) => {
                let mut structs_mut = self.structs.borrow_mut();
                let s = structs_mut
                    .entry(Self::path_to_s(path))
                    .or_insert(self.context.opaque_struct_type(&Self::path_to_s(path)));
                s.as_basic_type_enum()
            }
            Ty::Int32 => inkwell::types::BasicTypeEnum::IntType(self.context.i32_type()),
            Ty::Pointer(p) => inkwell::types::BasicTypeEnum::PointerType(
                self.compile_ty(p).ptr_type(inkwell::AddressSpace::Generic),
            ),
            Ty::Infer => todo!(),
            Ty::Int64 => {
                inkwell::types::BasicTypeEnum::IntType(self.context.custom_width_int_type(64))
            }
            Ty::Bool => {
                inkwell::types::BasicTypeEnum::IntType(self.context.custom_width_int_type(1))
            }
            Ty::UInt32 => {
                inkwell::types::BasicTypeEnum::IntType(self.context.custom_width_int_type(32))
            }
            Ty::UInt64 => {
                inkwell::types::BasicTypeEnum::IntType(self.context.custom_width_int_type(64))
            }
            Ty::ArrayTy(ty, len) => {
                inkwell::types::BasicTypeEnum::ArrayType(self.compile_ty(ty).array_type(*len))
            }
            Ty::Int8 => inkwell::types::BasicTypeEnum::IntType(self.context.i8_type()),
        }
    }
    pub fn path_to_s(p: &Path) -> String {
        p.parts
            .iter()
            .map(|p| p.0.clone())
            .collect::<Vec<_>>()
            .join("__")
    }
    pub fn write_object_file(&self, p: &StdPath) {
        self.get_target_machine()
            .write_to_file(&self.module, FileType::Object, &p)
            .unwrap();
    }
    pub fn write_assembly_file(&self, p: &StdPath) {
        self.get_target_machine()
            .write_to_file(&self.module, FileType::Assembly, &p)
            .unwrap();
    }
    fn get_target_machine(&self) -> TargetMachine {
        Target::initialize_all(&InitializationConfig {
            ..Default::default()
        });
        let good_target = Target::from_name(&self.target).unwrap();
        let target_machine = good_target
            .create_target_machine(
                &TargetTriple::create(&format!("{}-pc-linux-gnu", self.target.replace("-", "_"))),
                "x86-64",
                "+avx2",
                self.optimization_level,
                RelocMode::PIC,
                CodeModel::Default,
            )
            .unwrap();
        target_machine
    }
}
