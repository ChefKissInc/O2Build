use cranelift::prelude::*;
use cranelift_module::{FuncId, Linkage, Module};

use crate::ast::{function::FunctionPrototype, typing::Type, Node};

impl super::Generator {
    pub fn gen_func_proto(
        &mut self,
        fn_proto: &FunctionPrototype,
        linkage: Linkage,
    ) -> Result<(FuncId, Signature), String> {
        let signature = Signature {
            params: fn_proto
                .args
                .iter()
                .map(|arg| {
                    if let Node::FunctionArgument(_, type_) = arg {
                        AbiParam::new(match type_ {
                            Type::Int => types::I64,
                            Type::Str => self.module.target_config().pointer_type(),
                            _ => unreachable!(),
                        })
                    } else {
                        unreachable!()
                    }
                })
                .collect(),
            returns: match fn_proto.ret_type {
                Type::Int => vec![AbiParam::new(types::I64)],
                Type::Str => {
                    vec![AbiParam::new(self.module.target_config().pointer_type())]
                }
                Type::Void => vec![],
            },
            call_conv: fn_proto.call_conv,
        };

        let id = match self
            .module
            .declare_function(&fn_proto.symbol, linkage, &signature)
        {
            Ok(v) => v,
            Err(e) => {
                return Err(format!(
                    "Failed to declare function '{:?}': {:?}",
                    fn_proto, e
                ));
            }
        };
        self.functions.insert(
            fn_proto.symbol.clone(),
            super::CompiledFunction {
                defined: false,
                id,
                param_count: fn_proto.args.len(),
            },
        );

        Ok((id, signature))
    }
}
