use lalrpop_util::lalrpop_mod;
mod ast;
lalrpop_mod!(pub gram);
#[test]
fn sensible() {
    assert!(gram::FunctionParser::new().parse("add (Int a)").is_ok());
    assert!(gram::FunctionParser::new().parse("add (Int a,)").is_ok());
    assert!(gram::FunctionParser::new().parse("add (Int a,Int a,)").is_ok());
    assert!(gram::FunctionParser::new().parse("add (Int a,Int b)").is_ok());

    assert!(gram::FunctionParser::new().parse("add (,Int a,)").is_err());

    assert!(gram::FunctionParser::new().parse("add() ").is_ok());
    assert!(gram::FunctionParser::new().parse("add () ").is_ok());
    assert!(gram::FunctionParser::new().parse("add  ()").is_ok());
    assert!(gram::FunctionParser::new().parse("add()  ").is_ok());
    assert!(gram::FunctionParser::new().parse("add  ()  ").is_ok());
    assert!(gram::FunctionParser::new().parse("  add () ").is_ok());

}
fn main() {
    // gram::FunctionParser::new().parse("add()").unwrap();
    //  gram::FunctionParser::new().parse("add () ").unwrap();
    gram::FunctionArgsParser::new().parse("Test test, A b, c D").unwrap();
    // gram::FunctionArgsParser::new().parse("Test test Test test").unwrap();

    // println!("Fun {:#?}", fun);
    //    assert!(gram::TermParser::new().parse("22").is_ok());
    //    assert!(gram::TermParser::new().parse("(22)").is_ok());
    //    assert!(gram::TermParser::new().parse("((((22))))").is_ok());
    //    assert!(gram::TermParser::new().parse("((22)").is_err());
}
