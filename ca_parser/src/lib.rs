use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub gram);

#[test]
pub fn struct_field() {
    gram::ItemParser::new()
        .parse("struct kek { a: Int}")
        .unwrap();
}
#[test]
pub fn function_param() {
    gram::FunctionArgParser::new().parse("'a: Int").unwrap();
}
