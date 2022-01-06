use cranelift::{codegen::ir::InstBuilder, prelude::*};
use cranelift_module::Module;

#[inline]
pub fn gen_integer_lit(
    fgen: &mut super::FunctionGenerator,
    lit: &str,
) -> Result<Option<Value>, String> {
    Ok(Some(
        fgen.builder
            .ins()
            .iconst(types::I64, lit.parse::<i64>().unwrap()),
    ))
}

#[inline]
pub fn gen_char_lit(
    fgen: &mut super::FunctionGenerator,
    c: &char,
) -> Result<Option<Value>, String> {
    Ok(Some(fgen.builder.ins().iconst(types::I64, *c as i64)))
}

#[inline]
pub fn gen_str_lit(fgen: &mut super::FunctionGenerator, _s: &str) -> Result<Option<Value>, String> {
    let data = fgen.module.declare_anonymous_data(false, false).unwrap();
    let data_local = fgen.module.declare_data_in_func(data, fgen.builder.func);
    Ok(Some(fgen.builder.ins().symbol_value(
        fgen.module.target_config().pointer_type(),
        data_local,
    )))
}
