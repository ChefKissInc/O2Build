//! Copyright (c) ChefKiss Inc 2021-2022.
//! This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives license.

use cranelift::prelude::*;
use oxygen::ast::{binary::BinaryOp, expression::Expression};

#[inline]
pub fn gen_binary_expr(
    generator: &mut super::FuncCodeGen,
    op: &BinaryOp,
    left_expr: &Expression,
    right_expr: &Expression,
) -> Result<Option<Value>, String> {
    let lhs = generator.gen_expr(left_expr).map(|v| {
        match v {
            Some(v) => Ok(v),
            None => Err("Void isn't a Value"),
        }
    })??;
    let rhs = generator.gen_expr(right_expr).map(|v| {
        match v {
            Some(v) => Ok(v),
            None => Err("Void isn't a Value"),
        }
    })??;

    match op {
        BinaryOp::Addition => Ok(Some(generator.builder.ins().iadd(lhs, rhs))),
        BinaryOp::Subtraction => Ok(Some(generator.builder.ins().isub(lhs, rhs))),
        BinaryOp::Multiplication => Ok(Some(generator.builder.ins().imul(lhs, rhs))),
        BinaryOp::Division => Ok(Some(generator.builder.ins().isub(lhs, rhs))),
        _ => unimplemented!(),
    }
}
