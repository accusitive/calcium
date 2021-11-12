use crate::lexer::Token;

/// Enum that represents all kinds of values that can be returned
/// from parser derivations.
///
/// This values has to be in a single enum, because LALR parsers
/// have a stack, and it's better for it to be heterogeneous.
#[derive(Debug, Clone)]
pub enum Value {
    None,
    Uninitialized,
    Stolen,
    Token(Token),
    Program(Box<Value>), // Valuelist
    ValueList(Vec<Value>),
    Function(String, Box<Value>, Box<Value>, Box<Value>), // Name, args, ty, body
    FunctionArg(String, Box<Value>),
    Ident(String),
    Statement(Box<Value>),
    LetStatement(Box<Value>, Box<Value>, Box<Value>), //ident, path, right
    ReturnStatement(Box<Value>),
    Expr(Box<Value>),

    LiteralExpr(String),
    PathExpr(Box<Value>),
    ArithExpr(Box<Value>, Op, Box<Value>),
    CallExpr(Box<Value>, Box<Value>), // Path, args
    Ty(Box<Value>),
    Infer,
}
#[derive(Debug, Clone, Copy)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}
impl Default for Value {
    fn default() -> Self {
        Self::Stolen
    }
}

impl Value {
    /// Required method, parser expects it to be defined.
    ///
    /// Constructor for `Value::Token(token)` variant.
    pub(crate) fn from_token(value: Token) -> Self {
        Self::Token(value)
    }
}

#[allow(non_snake_case)]
pub mod ValueList {
    use super::Value;

    pub(crate) fn from(value: Value) -> Vec<Value> {
        match value {
            Value::ValueList(v) => v,
            // Value::Number(out) => out,
            other => panic!("wrong type, expected Number, got {:?}", other),
        }
    }
}
#[allow(non_snake_case)]
pub mod Ident {
    use super::Value;

    pub(crate) fn from(value: Value) -> String {
        match value {
            Value::Ident(s) => s,
            // Value::Number(out) => out,
            other => panic!("wrong type, expected Number, got {:?}", other),
        }
    }
}
// macro_rules! impl_from {
//     ($i: ident) => {
//         pub mod $i {
//             use super::Value;

//         }
//     };
// }
