pub fn compile_stmt(&self, stmt: &Stmt) -> BasicValueEnum<'a> {
    match stmt {
        Stmt::LetStmt { name, ty: _, value } => {
            let compiled_expr = self.compile_expr(value);
            self.local_variables.borrow_mut().push(LocalVariable {
                value: compiled_expr,
                depth: *self.depth.borrow(),
                name: name.to_string(),
            });
            compiled_expr
        }
        Stmt::ExprStmt(e) => {
            let compiled_expr = self.compile_expr(e);
            compiled_expr
        }
        Stmt::Return(r) => {
            println!("Building Expr {:#?}", r);

            let compile_expr = self.compile_expr(r);
            println!("Building return {:#?}", compile_expr);
            self.builder.build_return(Some(&compile_expr));
            BasicValueEnum::StructValue(self.unit_ty().const_named_struct(&[]))
        }
    }
}
