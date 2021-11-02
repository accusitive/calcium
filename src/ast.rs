#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub args: Vec<FunctionArg>,
}
impl Function {
    pub fn new<S: Into<String>>(name: S, args: Vec<FunctionArg>) -> Self {
        Self {
            name: name.into(),
            args,
        }
    }
}
#[derive(Debug)]

pub struct FunctionArg {
    pub ty: String,
    pub ident: String,
}
