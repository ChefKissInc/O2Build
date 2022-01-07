use cranelift::prelude::*;
use cranelift_module::Module;

#[inline]
pub fn gen_integer_lit(
    generator: &mut super::FunctionGenerator,
    lit: &str,
) -> Result<Option<Value>, String> {
    Ok(Some(
        generator
            .builder
            .ins()
            .iconst(types::I64, lit.parse::<i64>().unwrap()),
    ))
}

#[inline]
pub fn gen_char_lit(
    generator: &mut super::FunctionGenerator,
    c: char,
) -> Result<Option<Value>, String> {
    Ok(Some(generator.builder.ins().iconst(types::I64, c as i64)))
}

#[inline]
pub fn gen_str_lit(
    generator: &mut super::FunctionGenerator,
    data: Box<[u8]>,
) -> Result<Option<Value>, String> {
    generator.data_ctx.define(data);

    let data = generator
        .module
        .declare_anonymous_data(false, false)
        .unwrap();
    generator
        .module
        .define_data(data, generator.data_ctx)
        .unwrap();

    let data_local = generator
        .module
        .declare_data_in_func(data, generator.builder.func);

    generator.data_ctx.clear();
    Ok(Some(generator.builder.ins().symbol_value(
        generator.module.target_config().pointer_type(),
        data_local,
    )))
}
