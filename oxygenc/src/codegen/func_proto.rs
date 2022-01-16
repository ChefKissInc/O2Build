/*
 * Copyright (c) VisualDevelopment 2021-2021.
 * This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.
 */

use cranelift::prelude::{isa::CallConv, *};
use cranelift_module::{FuncId, Linkage, Module};
use oxygen::ast::{function::FunctionPrototype, typing::Type, Node};

impl super::CodeGen {
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
            call_conv: match fn_proto.call_conv {
                oxygen::abi::CallConv::C => self.module.isa().default_call_conv(),
                oxygen::abi::CallConv::SystemV => CallConv::SystemV,
                oxygen::abi::CallConv::UEFI => CallConv::WindowsFastcall,
            },
        };

        let id = self
            .module
            .declare_function(&fn_proto.symbol, linkage, &signature)
            .map_err(|e| e.to_string())?;

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
