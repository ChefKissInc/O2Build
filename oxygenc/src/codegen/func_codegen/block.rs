/*
 * Copyright (c) VisualDevelopment 2021-2021.
 * This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.
 */

use cranelift::prelude::*;
use oxygen::ast::expression::Expression;

#[inline]
pub fn gen_block_expr(
    generator: &mut super::FuncCodeGen,
    exprs: &[Expression],
) -> Result<Option<Value>, String> {
    for expr in exprs {
        generator.gen_expr(expr)?;
    }

    Ok(None)
}
