/*
 * Copyright (c) VisualDevelopment 2021-2021.
 * This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.
 */

use std::collections::HashMap;

use cranelift::{frontend::FunctionBuilder, prelude::*};
use cranelift_module::DataContext;
use cranelift_object::ObjectModule;
use oxygen::ast::expression::Expression;

use self::{binary::gen_binary_expr, block::gen_block_expr, func_call::gen_func_call, literals::*};
use super::CompiledFunction;

mod binary;
mod block;
mod func_call;
mod literals;

pub struct FuncCodeGen<'a> {
    pub builder: FunctionBuilder<'a>,
    pub functions: &'a HashMap<String, CompiledFunction>,
    pub module: &'a mut ObjectModule,
    pub data_ctx: &'a mut DataContext,
}

impl<'a> FuncCodeGen<'a> {
    #[inline]
    pub fn gen_expr(&mut self, expr: &Expression) -> Result<Option<Value>, String> {
        match expr {
            Expression::IntegerLiteral(lit) => gen_integer_lit(self, lit.parse().unwrap()),
            Expression::CharLiteral(c) => gen_char_lit(self, *c),
            Expression::StringLiteral(s) => {
                gen_str_lit(
                    self,
                    (s.clone() + "\0").as_bytes().to_vec().into_boxed_slice(),
                )
            }
            Expression::FunctionCall { name, args } => gen_func_call(self, name, args),
            Expression::Block(exprs) => gen_block_expr(self, exprs),
            Expression::Binary {
                op,
                left_expr,
                right_expr,
            } => gen_binary_expr(self, op, left_expr, right_expr),
            _ => unimplemented!(),
        }
    }
}
