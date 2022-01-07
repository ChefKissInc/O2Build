use cranelift::prelude::*;

use crate::ast::expression::Expression;

#[inline]
pub fn gen_block_expr(
    generator: &mut super::FunctionGenerator,
    exprs: &[Expression],
) -> Result<Option<Value>, String> {
    for expr in exprs {
        generator.gen_expr(expr)?;
    }

    Ok(None)
}
