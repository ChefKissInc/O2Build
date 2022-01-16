/*
 * Copyright (c) VisualDevelopment 2021-2021.
 * This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.
 */

use std::collections::HashMap;

use cranelift::prelude::*;
use cranelift_module::{DataContext, FuncId, Linkage, Module};
use cranelift_object::{ObjectBuilder, ObjectModule};
use oxygen::ast::{Node, SyntaxTree};

mod func_codegen;
mod func_def;
mod func_proto;

pub struct CompiledFunction {
    pub defined: bool,
    pub id: FuncId,
    pub param_count: usize,
}

pub struct CodeGen {
    builder_ctx: FunctionBuilderContext,
    functions: HashMap<String, CompiledFunction>,
    ctx: codegen::Context,
    data_ctx: DataContext,
    pub module: ObjectModule,
}

impl CodeGen {
    pub fn new(isa: Box<dyn isa::TargetIsa>, name: &str) -> Self {
        let module = ObjectModule::new(
            ObjectBuilder::new(isa, name, cranelift_module::default_libcall_names()).unwrap(),
        );

        Self {
            builder_ctx: FunctionBuilderContext::new(),
            functions: HashMap::new(),
            ctx: module.make_context(),
            data_ctx: DataContext::new(),
            module,
        }
    }

    pub fn gen_module(&mut self, syntax_tree: &SyntaxTree) -> Result<(), String> {
        for member in &syntax_tree.members {
            match member {
                Node::FunctionDefinition(_, _) => {
                    self.gen_func(member)?;
                }
                Node::ExternalFunction(fn_proto) => {
                    self.gen_func_proto(fn_proto, Linkage::Import)?;
                }
                Node::StaticDecl => todo!(),
                _ => panic!("Bug: {:?} shouldn't be in the syntax tree root", member),
            }
        }

        Ok(())
    }
}
