use cranelift::prelude::*;
use cranelift_module::Module;

use crate::ast::expression::Expression;

#[inline]
pub fn gen_func_call(
    fgen: &mut super::FunctionGenerator,
    name: &str,
    args: &[Expression],
) -> Result<Option<Value>, String> {
    match fgen.functions.get(name) {
        Some(func) => {
            let mut compiled_args = Vec::with_capacity(args.len());

            for arg in args {
                compiled_args.push(match fgen.gen_expr(arg)? {
                    Some(v) => v,
                    None => return Err("Void isn't a Value".to_string()),
                });
            }

            let local_func = fgen.module.declare_func_in_func(func.id, fgen.builder.func);
            let call = fgen.builder.ins().call(local_func, &compiled_args);

            Ok(fgen.builder.inst_results(call).get(0).copied())
        }
        None => Err(format!("Function not found: {}", name)),
    }
}
