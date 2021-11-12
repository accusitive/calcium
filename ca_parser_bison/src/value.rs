use crate::lexer::Token;

/// Enum that represents all kinds of values that can be returned
/// from parser derivations.
///
/// This values has to be in a single enum, because LALR parsers
/// have a stack, and it's better for it to be heterogeneous.
#[derive(Debug, Clone)]
pub enum Value {
    /// Required variant, parser expects it to be defined
    None,
    /// Required variant, parser expects it to be defined
    Uninitialized,
    /// Required variant, parser expects it to be defined
    Stolen,

    /// Required variant, parser expects it to be defined.
    /// Represents a token that is returned from a Lexer
    Token(Token),

    /// Represents a number
    // Number(i32),
    Program(Vec<Value>),
    ValueList(Vec<Value>),
    Function(String, Vec<Value>, Box<Value>),
    FunctionArg(Box<Value>, Box<Value>),
    Ident(String),
}
#[derive(Debug, Clone)]
pub struct XFunction {
    pub name: String,
    pub args: Vec<FunctionArg>,
}
#[derive(Debug, Clone)]
pub struct FunctionArg {
    pub name: String,
    pub ty: String,
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
