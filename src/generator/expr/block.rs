use cranelift::prelude::*;

use crate::ast::expression::Expression;

#[inline]
pub fn gen_block_expr(
    fgen: &mut super::FunctionGenerator,
    exprs: &[Expression],
) -> Result<Option<Value>, String> {
    for expr in exprs {
        fgen.gen_expr(expr)?;
    }

    Ok(None)
}
