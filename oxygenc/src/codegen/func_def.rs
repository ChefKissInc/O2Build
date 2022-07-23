//! Copyright (c) ChefKiss Inc 2021-2022.
//! This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives license.

use cranelift::{
    codegen::{verifier::VerifierErrors, CodegenError},
    frontend::FunctionBuilder,
    prelude::*,
};
use cranelift_module::{FuncId, Linkage, Module, ModuleError};
use oxygen::ast::Node;

use super::func_codegen::FuncCodeGen;

impl super::CodeGen {
    pub fn gen_func(&mut self, func: &Node) -> Result<FuncId, String> {
        if let Node::FunctionDefinition(fn_proto, body) = func {
            let (function, signature) = self.gen_func_proto(
                fn_proto,
                if fn_proto.public {
                    Linkage::Export
                } else {
                    Linkage::Local
                },
            )?;
            self.ctx.func.signature = signature;

            let mut builder = FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_ctx);

            let entry = builder.create_block();
            builder.append_block_params_for_function_params(entry);
            builder.switch_to_block(entry);
            builder.seal_block(entry);

            self.functions.get_mut(&fn_proto.symbol).unwrap().defined = true;

            let mut generator = FuncCodeGen {
                builder,
                functions: &self.functions,
                module: &mut self.module,
                data_ctx: &mut self.data_ctx,
            };

            match generator.gen_expr(body)? {
                Some(v) => generator.builder.ins().return_(&[v]),
                None => generator.builder.ins().return_(&[]),
            };

            generator.builder.finalize();

            println!("{}", self.ctx.func.display());
            self.module
                .define_function(function, &mut self.ctx)
                .map_err(|e| {
                    if let ModuleError::Compilation(CodegenError::Verifier(VerifierErrors(errs))) =
                        e
                    {
                        format!("Compilation error: Verifier errors: {:#?}", errs)
                    } else {
                        e.to_string()
                    }
                })?;
            self.module.clear_context(&mut self.ctx);

            Ok(function)
        } else {
            unimplemented!()
        }
    }
}
