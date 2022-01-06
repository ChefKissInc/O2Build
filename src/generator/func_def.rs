use cranelift::{
    codegen::{
        binemit::{NullStackMapSink, NullTrapSink},
        verifier::VerifierErrors,
        CodegenError,
    },
    frontend::FunctionBuilder,
    prelude::*,
};
use cranelift_module::{FuncId, Linkage, Module, ModuleError};

use super::expr::FunctionGenerator;
use crate::ast::Node;

impl super::Generator {
    pub fn gen_func(&mut self, func: &Node) -> Result<FuncId, String> {
        if let Node::FunctionDefinition(fn_proto, body) = func {
            let mut context = self.module.make_context();
            let (function, signature) = self.gen_func_proto(
                fn_proto,
                if fn_proto.public {
                    Linkage::Export
                } else {
                    Linkage::Local
                },
            )?;
            context.func.signature = signature;

            let mut builder = FunctionBuilder::new(&mut context.func, &mut self.builder_context);

            let entry = builder.create_block();
            builder.append_block_params_for_function_params(entry);
            builder.switch_to_block(entry);
            builder.seal_block(entry);

            self.functions.get_mut(&fn_proto.symbol).unwrap().defined = true;

            let mut generator = FunctionGenerator {
                builder,
                functions: &self.functions,
                module: &mut self.module,
            };

            match generator.gen_expr(body)? {
                Some(v) => generator.builder.ins().return_(&[v]),
                None => generator.builder.ins().return_(&[]),
            };

            generator.builder.finalize();

            println!("{}", context.func.display());
            match self.module.define_function(
                function,
                &mut context,
                &mut NullTrapSink {},
                &mut NullStackMapSink {},
            ) {
                Ok(_) => {}
                Err(e) => {
                    if let ModuleError::Compilation(CodegenError::Verifier(VerifierErrors(ce))) = e
                    {
                        return Err(format!(
                            "Failed to define function '{:?}': Compilation error: Verifier \
                             errors: {:?}",
                            fn_proto, ce
                        ));
                    } else {
                        return Err(format!("Failed to define function '{:?}': {}", fn_proto, e));
                    }
                }
            };
            self.module.clear_context(&mut context);

            Ok(function)
        } else {
            unimplemented!()
        }
    }
}
