use std::ops::Deref;

use ca_parser_bison::value::Value;

pub fn build_ast(v: Value) {}
pub fn to_program(v: &Value) -> Program {
    match v {
        Value::Program(list) => Program {
            functions: to_vec(list).iter().map(|v| to_function(v)).collect(),
        },
        _ => todo!(),
    }
}
pub fn to_function(v: &Value) -> Function {
    match v {
        Value::Function(name, params, ty, body) => Function {
            name: Identifier(name.to_string()),
            args: to_vec(params).iter().map(|a| to_function_arg(a)).collect(),
        },
        _ => todo!(),
    }
}
pub fn to_function_arg(v: &Value) -> FunctionArg {
    match v {
        Value::FunctionArg(name, ty) => FunctionArg {
            name: Identifier(name.to_string()),
            ty: to_ty(ty),
        },
        _ => todo!(),
    }
}
pub fn to_ty(v: &Value) -> Ty {
    match v {
        Value::Ty(t) => match &**t {
            Value::Infer => Ty::Infer,
            Value::PathExpr(_segments) => Ty::Named(to_path(&t)),
            _ => todo!(),
        },
        _ => todo!(),
    }
}
pub fn to_path(v: &Value) -> Path {
    match v {
        Value::PathExpr(p) => {
            let segments = to_vec(p).iter().map(|seg| to_identifier(seg)).collect();
            Path { parts: segments }
        }
        _ => todo!(),
    }
}
pub fn to_identifier(v: &Value) -> Identifier {
    match v {
        Value::Ident(s) => Identifier(s.to_string()),
        _ => todo!(),
    }
}
pub fn to_vec(v: &Value) -> Vec<Value> {
    match v {
        Value::ValueList(v) => v.to_vec(),
        _ => todo!(),
    }
}

#[derive(Debug)]
pub struct Identifier(String);
#[derive(Debug)]

pub struct Program {
    functions: Vec<Function>,
}
#[derive(Debug)]

pub struct Function {
    name: Identifier,
    args: Vec<FunctionArg>,
}
#[derive(Debug)]

pub struct FunctionArg {
    name: Identifier,
    ty: Ty,
}
#[derive(Debug)]

pub struct ValueList<V> {
    content: Vec<V>,
}
#[derive(Debug)]

pub struct Path {
    parts: Vec<PathSegment>,
}

type PathSegment = Identifier;
#[derive(Debug)]

pub enum Ty {
    Named(Path),
    Infer,
}
