use crate::Compiler;

impl<'a> Compiler<'a> {
    pub fn compile_expr(&self, e: &Expr) -> BasicValueEnum<'a> {
        match e {
            Expr::LiteralExpr(l) => match l {
                ca_ast::Literal::String(_) => todo!(),
                ca_ast::Literal::Char(_) => todo!(),
                ca_ast::Literal::Integer(i) => inkwell::values::BasicValueEnum::IntValue(
                    self.context
                        .i32_type()
                        .const_int((*i).try_into().unwrap(), false),
                ),
            },

            Expr::Block(s) => {
                *self.depth.borrow_mut() += 1;
                let compiled = s.iter().map(|s| self.compile_stmt(s)).collect::<Vec<_>>();
                for stmt in s {
                    self.compile_stmt(stmt);
                }
                *self.depth.borrow_mut() -= 1;
                // Remove all local variables that was inside block.

                // Block here to narrow borrow scope
                {
                    let mut borrow = self.local_variables.borrow_mut();
                    let (_drained, remaining): (Vec<&LocalVariable>, Vec<&LocalVariable>) = borrow
                        .iter()
                        .partition(|lv| lv.depth >= *self.depth.borrow());

                    let remaining_owned: Vec<LocalVariable> = remaining
                        .clone()
                        .into_iter()
                        .map(|l| l.to_owned())
                        .collect();
                    *borrow = remaining_owned;
                }
                *compiled
                    .last()
                    .unwrap_or(&inkwell::values::BasicValueEnum::StructValue(
                        self.unit_ty().const_named_struct(&[]),
                    ))
            }
            Expr::BinOp(l, op, r) => {
                let left = self.compile_expr(l);
                let right = self.compile_expr(r);
                let result = match op {
                    ca_ast::Op::LShift => todo!(),
                    ca_ast::Op::RShift => todo!(),
                    ca_ast::Op::Mul => todo!(),
                    ca_ast::Op::Div => todo!(),
                    ca_ast::Op::Sub => self.builder.build_int_sub(
                        left.into_int_value(),
                        right.into_int_value(),
                        "sub",
                    ),
                    ca_ast::Op::Add => self.builder.build_int_add(
                        left.into_int_value(),
                        right.into_int_value(),
                        "add",
                    ),
                    ca_ast::Op::Xor => todo!(),
                    ca_ast::Op::And => todo!(),
                    ca_ast::Op::Or => todo!(),
                    ca_ast::Op::Lt => self.builder.build_int_compare(
                        inkwell::IntPredicate::SLT,
                        left.into_int_value(),
                        right.into_int_value(),
                        "slt",
                    ),

                    ca_ast::Op::Gt => self.builder.build_int_compare(
                        inkwell::IntPredicate::SGT,
                        left.into_int_value(),
                        right.into_int_value(),
                        "sgt",
                    ),
                };
                inkwell::values::BasicValueEnum::IntValue(result)
            }
            Expr::Ident(i) => {
                let borrow = self.local_variables.borrow();
                let lv = borrow.iter().find(|v| &v.name == i);
                match lv {
                    Some(v) => v.value,
                    None => {
                        let borrow_mut = self.function_arg_names.borrow_mut();
                        let current_function_value = self.current_function.borrow().unwrap();
                        let current_function_name =
                            current_function_value.get_name().to_str().unwrap();
                        let index = borrow_mut
                            .get(current_function_name)
                            .unwrap()
                            .get(i)
                            .expect(&format!(
                                "No local variable or function parameter `{}` found.",
                                i
                            ));
                        let fa = self
                            .current_function
                            .borrow()
                            .unwrap()
                            .get_nth_param(*index)
                            .unwrap();
                        fa
                    }
                }
            }
            Expr::UnaryOp(_, _) => todo!(),
            Expr::Call(name, args) => {
                let borrow = self.functions.borrow();
                let f = borrow
                    .get(name)
                    .expect(&format!("Function `{}` not found.", name));
                let args = args
                    .iter()
                    .map(|e| self.compile_expr(e))
                    .collect::<Vec<_>>();
                self.builder
                    .build_call(*f, &args, "call()")
                    .try_as_basic_value()
                    .left()
                    .unwrap()
            }
            Expr::If(predicate, then, elze) => {
                let cmp = self.compile_expr(predicate);
                let function = self.current_function.borrow().unwrap();
                let entry = function.get_last_basic_block().unwrap();

                let then_block = self.context.append_basic_block(function, "then");
                let elze_block = self.context.append_basic_block(function, "elze");
                let cont = self.context.append_basic_block(function, "cont");

                self.builder.position_at_end(then_block);
                let thenv = self.compile_expr(then);
                println!("ThenV {:#?}", then);
                self.builder.build_unconditional_branch(cont);

                self.builder.position_at_end(elze_block);
                let elzev = self.compile_expr(elze);
                println!("elzev {:#?}", elze);

                self.builder.build_unconditional_branch(cont);

                self.builder.position_at_end(entry);

                self.builder
                    .build_conditional_branch(cmp.into_int_value(), then_block, elze_block);

                self.builder.position_at_end(cont);

                let pn = self.builder.build_phi(self.context.i32_type(), "pn");
                pn.add_incoming(&[(&thenv, then_block)]);
                pn.add_incoming(&[(&elzev, elze_block)]);

                pn.as_basic_value()
            }
        }
    }
}
