use std::collections::HashMap;

use cranelift::prelude::*;
use cranelift_module::{DataContext, FuncId, Linkage, Module};
use cranelift_object::{ObjectBuilder, ObjectModule};

use crate::ast::{Node, SyntaxTree};

mod expr;
mod func_def;
mod func_proto;

pub struct CompiledFunction {
    pub defined: bool,
    pub id: FuncId,
    pub param_count: usize,
}

pub struct Generator {
    builder_context: FunctionBuilderContext,
    functions: HashMap<String, CompiledFunction>,
    ctx: codegen::Context,
    data_ctx: DataContext,
    module: ObjectModule,
}

impl Default for Generator {
    fn default() -> Self {
        let module = ObjectModule::new(
            ObjectBuilder::new(
                isa::lookup_by_name("x86_64-apple-darwin")
                    .unwrap()
                    .finish(settings::Flags::new(settings::builder())),
                "Idk",
                cranelift_module::default_libcall_names(),
            )
            .unwrap(),
        );
        Self {
            builder_context: FunctionBuilderContext::new(),
            functions: HashMap::new(),
            ctx: module.make_context(),
            data_ctx: DataContext::new(),
            module,
        }
    }
}

impl Generator {
    pub fn compile_program(&mut self, program: &SyntaxTree) -> Result<(), String> {
        for member in &program.members {
            match member {
                Node::FunctionDefinition(_, _) => {
                    self.gen_func(member)?;
                }
                Node::ExternalFunction(fn_proto) => {
                    self.gen_func_proto(fn_proto, Linkage::Import)?;
                }
                Node::StaticDecl => todo!(),
                _ => panic!("This shouldn't be here"),
            }
        }

        Ok(())
    }
}
