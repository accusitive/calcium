use std::path::PathBuf;

use ca_parser_bison::{
    lexer::Lexer,
    parser::{token_name, Parser},
};
use colored::Colorize;

fn main() {
    let mut path = PathBuf::new();
    path.push("examples");
    path.push("main");
    path.set_extension("ca");
    let source = &std::fs::read_to_string(path.clone()).unwrap();
    let lexer = Lexer::new(source);

    for token in lexer {
        if token.token_type == Lexer::YYEOF {
            break;
        }
        println!(
            "{:>15} := {:<10} @ {:>2}..{:>2}  ",
            token_name(token.token_type),
            token.token_value,
            token.loc.begin,
            token.loc.end
        );
    }

    let lexer = Lexer::new(source);

    let parser = Parser::new(lexer, "Cheese", source, path);

    let parsed = parser.do_parse();

    match parsed.0 {
        Some(_result) => {
            let (_value, _name, output) = parsed;
            println!("Output: {:#?}", output.as_ref().unwrap());
            // panic!();
            let program = ca_uir::to_program(&output.unwrap());
            // println!("program: {:#?}", program);
            let ctx = ca_backend_llvm::inkwell::context::Context::create();
            let mut compiler = ca_backend_llvm::Compiler::new_compiler(&ctx);
            compiler.compile_program(&program);
            compiler.module.print_to_stderr();
        }
        None => {
            println!("{}", "Compilation failed.".bold())
        }
    }
}
#[cfg(test)]
mod tests {
    use ca_parser_bison::value::Value;
    use paste::paste;

    use ca_uir::*;
    macro_rules! asdf {
        ($main: ident, $test_case: expr, $($i: ident),*) => {
            paste! {
                #[test]
                fn [<asdf_ $main>]() {
                    [<to_$main>]($test_case);
                }
                $(
                    #[test]
                    #[should_panic]
                    fn [<$main _asdf_ $i>]() {
                        [<to_$i>]($test_case);

                    }
                )*
            }
        };
    }
    macro_rules! test_case_program {
        () => {
            Box::new(Value::Program(Box::new(Value::ValueList(vec![]))))
        };
    }
    macro_rules! test_case_statement {
        () => {
            Box::new(Value::Statement(Box::new(Value::LetStatement(
                test_case_ident!(),
                test_case_ty!(),
                test_case_expr!(),
            ))))
        };
    }
    macro_rules! test_case_ident {
        () => {
            Box::new(Value::Ident("test_case".to_string()))
        };
    }
    macro_rules! test_case_ty {
        () => {
            Box::new(Value::Ty(Box::new(Value::Infer)))
        };
    }
    macro_rules! test_case_expr {
        () => {
            Box::new(Value::Expr(Box::new(Value::LiteralExpr("12".to_string()))))
        };
    }

    asdf!(
        program,
        &test_case_program!(),
        function,
        expression,
        statement,
        function_arg,
        ty,
        path,
        identifier,
        vec
    );
    asdf!(
        statement,
        &test_case_statement!(),
        function,
        expression,
        function_arg,
        ty,
        path,
        identifier,
        vec
    );
    asdf!(
        ty,
        &test_case_ty!(),
        function,
        expression,
        function_arg,
        path,
        identifier,
        vec
    );
    asdf!(
        expression,
        &test_case_expr!(),
        program,
        function,
        function_arg,
        ty,
        path,
        identifier,
        vec
    );
}
