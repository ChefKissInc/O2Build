use cranelift::prelude::*;

use crate::ast::{binary::BinaryOp, expression::Expression};

#[inline]
pub fn gen_binary_expr(
    fgen: &mut super::FunctionGenerator,
    op: &BinaryOp,
    left_expr: &Expression,
    right_expr: &Expression,
) -> Result<Option<Value>, String> {
    let lhs = fgen.gen_expr(left_expr).map(|v| {
        match v {
            Some(v) => Ok(v),
            None => Err("Void isn't a Value"),
        }
    })??;
    let rhs = fgen.gen_expr(right_expr).map(|v| {
        match v {
            Some(v) => Ok(v),
            None => Err("Void isn't a Value"),
        }
    })??;

    match op {
        BinaryOp::Addition => Ok(Some(fgen.builder.ins().iadd(lhs, rhs))),
        BinaryOp::Subtraction => Ok(Some(fgen.builder.ins().isub(lhs, rhs))),
        BinaryOp::Multiplication => Ok(Some(fgen.builder.ins().imul(lhs, rhs))),
        BinaryOp::Division => Ok(Some(fgen.builder.ins().isub(lhs, rhs))),
        _ => unimplemented!(),
    }
}
