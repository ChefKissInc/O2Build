/*
 * Copyright (c) VisualDevelopment 2021-2021.
 * This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.
 */

use cranelift::prelude::*;
use cranelift_module::Module;

#[inline]
pub fn gen_integer_lit(
    codegen: &mut super::FuncCodeGen,
    val: i64,
) -> Result<Option<Value>, String> {
    Ok(Some(codegen.builder.ins().iconst(types::I64, val)))
}

#[inline]
pub fn gen_char_lit(codegen: &mut super::FuncCodeGen, c: char) -> Result<Option<Value>, String> {
    Ok(Some(codegen.builder.ins().iconst(types::I64, c as i64)))
}

#[inline]
pub fn gen_str_lit(
    codegen: &mut super::FuncCodeGen,
    data: Box<[u8]>,
) -> Result<Option<Value>, String> {
    codegen.data_ctx.define(data);

    let data = codegen.module.declare_anonymous_data(false, false).unwrap();
    codegen.module.define_data(data, codegen.data_ctx).unwrap();

    let data_local = codegen
        .module
        .declare_data_in_func(data, codegen.builder.func);

    codegen.data_ctx.clear();
    Ok(Some(codegen.builder.ins().symbol_value(
        codegen.module.target_config().pointer_type(),
        data_local,
    )))
}
