use std::{cell::RefCell, collections::HashMap};

use ca_uir::{Function, Item, Literal, Program, Statement, Struct, Ty};

#[allow(dead_code)]
pub struct TypeChecker {
    functions: RefCell<HashMap<String, Function>>,
    structs: RefCell<HashMap<String, Struct>>,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            functions: RefCell::new(HashMap::new()),
            structs: RefCell::new(HashMap::new()),
        }
    }
    pub fn check_program(&self, p: &Program) {
        for item in &p.items {
            self.check_item(item);
        }
    }
    pub fn check_item(&self, i: &Item) {
        match i {
            Item::Function(f) => self.check_function(f),
            Item::Struct(s) => self.check_struct(s),
            Item::Import(p) => self.check_program(&p.prog),
        }
    }

    pub fn check_function(&self, f: &Function) {
        if f.is_extern {
            return;
        }
        match f.body.as_ref().unwrap() {
            ca_uir::Expression::Block(stmts) => {
                for stmt in stmts {
                    self.check_statement(stmt)
                }
            }
            _ => panic!(),
        }
    }
    pub fn check_statement(&self, s: &Statement) {
        match s {
            Statement::Let(_name, expected_ty, right) => {
                let right_ty = self.get_expr_ty(right);
                if !expected_ty.is_infer() && &right_ty != expected_ty {
                    panic!("Let statement's right hand type is {}, but the right hand evaluates to {}.", right_ty, expected_ty);
                }
            }
            Statement::Return(_) => {}
            Statement::Expr(_e) => {}
        }
    }
    pub fn check_struct(&self, s: &Struct) {
        for field in &s.fields {
            match field.ty {
                Ty::Infer => panic!("Field {} on struct {} cannot be Infer.", field.name, s.name),
                _ => {}
            }
        }
    }

    pub fn get_expr_ty(&self, e: &ca_uir::Expression) -> Ty {
        match e {
            ca_uir::Expression::Call(_, _) => todo!(),
            ca_uir::Expression::Arith(_, _, _) => todo!(),
            ca_uir::Expression::Literal(lit) => self.get_lit_ty(lit),
            ca_uir::Expression::Block(_) => todo!(),
            ca_uir::Expression::Path(_) => todo!(),
            ca_uir::Expression::New(_, _) => todo!(),
            ca_uir::Expression::FieldExpr(_, _) => todo!(),
        }
    }
    pub fn get_lit_ty(&self, l: &Literal) -> Ty {
        match l {
            Literal::Number(_val, ty) => ty.to_owned(),
            Literal::String(_) => Ty::Pointer(Box::new(Ty::Int8)),
        }
    }
}
