use std::fmt::{Display, Write};

use ca_parser_bison::value::{Op, Value};

pub fn to_program(v: &Value) -> Program {
    match v {
        Value::Program(list) => Program {
            items: to_vec(list).iter().map(|v| to_item(v)).collect(),
        },
        _ => todo!(),
    }
}
pub fn to_item(v: &Value) -> Item {
    match v {
        Value::Item(i) => match &**i {
            Value::Function(_, _, _, _, _, _) => Item::Function(to_function(i)),
            Value::Struct(_, _) => Item::Struct(to_struct(i)),
            Value::Import(_, _) => Item::Import(to_import(i)),
            _ => todo!(),
        },
        _ => todo!(),
    }
}
pub fn to_function(v: &Value) -> Function {
    // TODO: How can the parser not return a Value::None with no function args?
    match v {
        Value::Function(name, params, ty, body, is_extern, is_varargs) => Function {
            name: Identifier(name.to_string()),
            args: to_vec(params)
                .iter()
                .filter(|f| match &**f {
                    Value::None => false,
                    _ => true,
                })
                .map(|a| to_function_arg(a))
                .collect(),
            return_ty: to_ty(ty),
            body: body.as_ref().map(|e| to_expression(e)),
            is_extern: *is_extern,
            is_varargs: *is_varargs,
        },

        _ => todo!(),
    }
}
pub fn to_struct(v: &Value) -> Struct {
    match v {
        Value::Struct(name, fields) => Struct {
            name: to_identifier(name),
            fields: to_vec(fields).iter().map(|v| to_struct_field(v)).collect(),
        },
        _ => todo!(),
    }
}
pub fn to_struct_field(v: &Value) -> StructField {
    match v {
        Value::StructField(name, ty) => StructField {
            name: to_identifier(name),
            ty: to_ty(ty),
        },
        _ => todo!(),
    }
}
pub fn to_import(v: &Value) -> Import {
    match v {
        Value::Import(ident, prog) => Import {
            ident: to_identifier(ident),
            prog: to_program(prog),
        },
        _ => todo!(),
    }
}
pub fn to_expression(v: &Value) -> Expression {
    match v {
        Value::Expr(e) => match &**e {
            Value::CallExpr(func, values) => Expression::Call(
                to_path(&func),
                to_vec(&values).iter().map(|a| to_expression(a)).collect(),
            ),
            #[rustfmt::skip]
            Value::LiteralExpr(l) => {
                // let to_ty = to_ty(t);
                let l = to_literal(l);
                // Converting to u64 should be fine for everything except u128.
                Expression::Literal(l)
            }

            Value::PathExpr(p) => Expression::Path(to_path(p)),
            Value::ArithExpr(left, op, right) => Expression::Arith(
                Box::new(to_expression(left)),
                *op,
                Box::new(to_expression(right)),
            ),
            Value::BlockExpr(stmts) => {
                Expression::Block(to_vec(stmts).iter().map(|s| to_statement(s)).collect())
            }
            Value::NewExpr(p, a) => Expression::New(
                to_path(&p),
                to_vec(&a).iter().map(|a| to_expression(a)).collect(),
            ),
            Value::FieldExpr(e, i) => {
                Expression::FieldExpr(Box::new(to_expression(e)), to_identifier(i))
            }
            _ => todo!(),
        },
        Value::BlockExpr(stmts) => {
            Expression::Block(to_vec(stmts).iter().map(|s| to_statement(s)).collect())
        }

        _ => todo!(),
    }
}
pub fn to_literal(v: &Value) -> Literal {
    /*
    match to_ty {
                    Ty::Named(_) => panic!("Cannot have a literal type as a named."),
                    Ty::Infer => panic!("This should never happen"),
                    Ty::Int32  => Expression::Literal(l.parse::<i32>() .expect("Failed to parser integer literal.").try_into().unwrap(), to_ty),
                    Ty::Int64  => Expression::Literal(l.parse::<i64>() .expect("Failed to parser integer literal.").try_into().unwrap(), to_ty),
                    Ty::Int128 => Expression::Literal(l.parse::<i128>().expect("Failed to parser integer literal.").try_into().unwrap(), to_ty),
                    Ty::UInt32 => Expression::Literal(l.parse::<u128>().expect("Failed to parser integer literal.").try_into().unwrap(), to_ty),
                    Ty::UInt64 => Expression::Literal(l.parse::<u64>() .expect("Failed to parser integer literal."), to_ty),
                    Ty::Pointer(_) => panic!("Cannot have a literal type as a pointer."),
                } */
    match v {
        Value::StringLiteral(s) => Literal::String(s.to_string()),
        Value::IntegerLiteral(i, ty) => {
            let to_ty = to_ty(ty);
            match to_ty {
                Ty::Int32 => Literal::Number(
                    i.parse::<i32>()
                        .expect("failed tp parse number")
                        .try_into()
                        .unwrap(),
                    to_ty,
                ),
                Ty::Int64 => Literal::Number(
                    i.parse::<i64>()
                        .expect("failed tp parse number")
                        .try_into()
                        .unwrap(),
                    to_ty,
                ),
                Ty::Int128 => Literal::Number(
                    i.parse::<i128>()
                        .expect("failed tp parse number")
                        .try_into()
                        .unwrap(),
                    to_ty,
                ),
                Ty::UInt32 => Literal::Number(
                    i.parse::<u32>()
                        .expect("failed tp parse number")
                        .try_into()
                        .unwrap(),
                    to_ty,
                ),
                Ty::UInt64 => Literal::Number(
                    i.parse::<u64>()
                        .expect("failed tp parse number")
                        .try_into()
                        .unwrap(),
                    to_ty,
                ),
                _ => todo!(),
            }
        }
        _ => todo!(),
    }
}
pub fn to_statement(v: &Value) -> Statement {
    match v {
        Value::Statement(s) => match &**s {
            Value::LetStatement(bind, ty, expr) => {
                Statement::Let(to_identifier(&bind), to_ty(&ty), to_expression(&expr))
            }
            Value::ReturnStatement(val) => Statement::Return(to_expression(val)),
            Value::ExprStatement(e) => Statement::Expr(to_expression(e)),
            _ => todo!(),
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
            Value::ValueList(_segments) => Ty::Named(to_path(&t)),
            Value::Int8 => Ty::Int8,
            Value::Int32 => Ty::Int32,
            Value::Int64 => Ty::Int64,
            Value::Int128 => Ty::Int128,
            Value::UInt32 => Ty::UInt32,
            Value::UInt64 => Ty::UInt64,
            Value::PointerTy(t) => Ty::Pointer(Box::new(to_ty(t))),
            Value::ArrayTy(ty, len) => Ty::ArrayTy(
                Box::new(to_ty(ty)),
                to_literal(len).get_integer_value().try_into().unwrap(),
            ),
            // Value::
            _ => todo!(),
        },
        _ => todo!(),
    }
}
pub fn to_path(v: &Value) -> Path {
    match v {
        Value::Path(p) => {
            let segments = to_vec(p).iter().map(|seg| to_identifier(seg)).collect();
            Path { parts: segments }
        }
        Value::ValueList(vl) => {
            let segments = vl.iter().map(|seg| to_identifier(seg)).collect();
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

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Identifier(pub String);
impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('`')?;
        f.write_str(self.0.as_str())?;
        f.write_char('`')?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct Program {
    pub items: Vec<Item>,
}
#[derive(Debug)]
pub enum Item {
    Function(Function),
    Struct(Struct),
    Import(Import),
}
#[derive(Debug)]
pub struct Struct {
    pub name: Identifier,
    pub fields: Vec<StructField>,
}
#[derive(Debug)]
pub struct StructField {
    pub name: Identifier,
    pub ty: Ty,
}
#[derive(Debug)]
pub struct Function {
    pub name: Identifier,
    pub args: Vec<FunctionArg>,
    pub return_ty: Ty,
    pub body: Option<Expression>,
    pub is_extern: bool,
    pub is_varargs: bool,
}
#[derive(Debug)]
pub struct FunctionArg {
    pub name: Identifier,
    pub ty: Ty,
}
#[derive(Debug)]
pub struct Import {
    pub ident: Identifier,
    pub prog: Program,
}
#[derive(Debug)]
pub struct ValueList<V> {
    pub content: Vec<V>,
}
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Path {
    pub parts: Vec<PathSegment>,
}

type PathSegment = Identifier;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ty {
    Named(Path),
    Infer,
    Int8,
    Int32,
    Int64,
    Int128,
    UInt32,
    UInt64,

    Pointer(Box<Self>),
    ArrayTy(Box<Self>, u32),
}
impl Display for Ty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ty::Named(p) => f.write_str(
                &p.parts
                    .iter()
                    .map(|p| p.0.clone())
                    .collect::<Vec<_>>()
                    .join("__"),
            ),
            Ty::Infer => f.write_char('_'),
            Ty::Int8 => f.write_str("i8"),
            Ty::Int32 => f.write_str("i32"),
            Ty::Int64 => f.write_str("i64"),
            Ty::Int128 => f.write_str("i128"),
            Ty::UInt32 => f.write_str("u32"),
            Ty::UInt64 => f.write_str("u64"),
            Ty::Pointer(inner) => f.write_fmt(format_args!("&{}", inner)),
            Ty::ArrayTy(ty, len) => f.write_fmt(format_args!("({}; {})", ty, len)),
        }
    }
}
impl Ty {
    /// Returns `true` if the ty is [`Infer`].
    ///
    /// [`Infer`]: Ty::Infer
    pub fn is_infer(&self) -> bool {
        matches!(self, Self::Infer)
    }

    /// Returns `true` if the ty is [`Named`].
    ///
    /// [`Named`]: Ty::Named
    pub fn is_named(&self) -> bool {
        matches!(self, Self::Named(..))
    }

    pub fn as_named(&self) -> Option<&Path> {
        if let Self::Named(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the ty is [`Int8`].
    ///
    /// [`Int8`]: Ty::Int8
    pub fn is_int8(&self) -> bool {
        matches!(self, Self::Int8)
    }

    /// Returns `true` if the ty is [`Int32`].
    ///
    /// [`Int32`]: Ty::Int32
    pub fn is_int32(&self) -> bool {
        matches!(self, Self::Int32)
    }

    /// Returns `true` if the ty is [`Int64`].
    ///
    /// [`Int64`]: Ty::Int64
    pub fn is_int64(&self) -> bool {
        matches!(self, Self::Int64)
    }

    /// Returns `true` if the ty is [`Int128`].
    ///
    /// [`Int128`]: Ty::Int128
    pub fn is_int128(&self) -> bool {
        matches!(self, Self::Int128)
    }

    /// Returns `true` if the ty is [`UInt32`].
    ///
    /// [`UInt32`]: Ty::UInt32
    pub fn is_uint32(&self) -> bool {
        matches!(self, Self::UInt32)
    }

    /// Returns `true` if the ty is [`UInt64`].
    ///
    /// [`UInt64`]: Ty::UInt64
    pub fn is_uint64(&self) -> bool {
        matches!(self, Self::UInt64)
    }
}
#[derive(Debug)]
pub enum Expression {
    Call(Path, Vec<Expression>),
    Arith(Box<Expression>, Op, Box<Expression>),
    Literal(Literal),
    Block(Vec<Statement>),
    Path(Path),
    New(Path, Vec<Expression>),
    FieldExpr(Box<Expression>, Identifier),
}
#[derive(Debug)]
pub enum Literal {
    Number(i128, Ty),
    String(String),
}
#[derive(Debug)]
pub enum Statement {
    Let(Identifier, Ty, Expression),
    Return(Expression),
    Expr(Expression),
}

impl Literal {
    fn get_integer_value(&self) -> i128 {
        match self {
            Literal::Number(n, _) => *n,
            Literal::String(_) => panic!("Invalid type."),
        }
    }
}
