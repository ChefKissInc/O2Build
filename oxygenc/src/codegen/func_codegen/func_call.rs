use cranelift::prelude::*;
use cranelift_module::Module;
use Oxygen::ast::expression::Expression;

#[inline]
pub fn gen_func_call(
    generator: &mut super::FuncCodeGen,
    name: &str,
    args: &[Expression],
) -> Result<Option<Value>, String> {
    match generator.functions.get(name) {
        Some(func) => {
            let mut compiled_args = Vec::with_capacity(args.len());

            for arg in args {
                compiled_args.push(match generator.gen_expr(arg)? {
                    Some(v) => v,
                    None => return Err("Void isn't a Value".to_string()),
                });
            }

            let local_func = generator
                .module
                .declare_func_in_func(func.id, generator.builder.func);
            let call = generator.builder.ins().call(local_func, &compiled_args);

            Ok(generator.builder.inst_results(call).get(0).copied())
        }
        None => Err(format!("Function not found: {}", name)),
    }
}
