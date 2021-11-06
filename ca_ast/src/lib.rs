pub type BExpr = Box<Expr>;
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Program {
    pub items: Vec<Item>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Item {
    Function(Function),
    Struct(Struct),
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Struct {
    pub name: String,
    pub fields: Vec<StructField>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct StructField {
    pub name: String,
    pub ty: Ty,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Function {
    pub name: String,
    pub args: Vec<FunctionArg>,
    pub body: Expr,
    pub ty: Ty,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]

pub struct FunctionArg {
    pub ty: Ty,
    pub ident: String,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Ty {
    Named(String),
    Int32,
    Bool,
    Unit,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Stmt {
    LetStmt {
        name: String,
        ty: Option<Ty>,
        value: Expr,
    },
    ExprStmt(Expr),
    Return(Box<Expr>),
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Expr {
    LiteralExpr(Literal),
    Block(Vec<Stmt>),
    BinOp(Box<Expr>, Op, Box<Expr>),
    Ident(String),
    UnaryOp(UnaryOp, Box<Expr>),
    Call(String, Vec<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Op {
    LShift,
    RShift,
    Mul,
    Div,
    Sub,
    Add,
    Xor,
    And,
    Or,
    Lt,
    Gt,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum UnaryOp {
    Sub,
    Invert,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Literal {
    String(String),
    Char(char),
    Integer(i64),
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Attribute {
    Short(String),
    Long(String, String),
    Array(String, Vec<String>),
}
impl Expr {
    pub fn assume_ident(&self) -> String {
        match self {
            Expr::Ident(s) => s.to_string(),
            _ => panic!("Assumption was wrong."),
        }
    }
}
