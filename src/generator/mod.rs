use inkwell::{
    builder::Builder, context::Context, module::Module, passes::PassManager,
    types::BasicMetadataTypeEnum, values::FunctionValue,
};

use crate::ast::{function::FunctionPrototype, Node, Program};

pub struct Compiler<'a, 'ctx> {
    pub context: &'ctx Context,
    pub builder: &'a Builder<'ctx>,
    pub fpm: &'a PassManager<FunctionValue<'ctx>>,
    pub module: &'a Module<'ctx>,
}

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn new(
        context: &'ctx Context,
        builder: &'a Builder<'ctx>,
        fpm: &'a PassManager<FunctionValue<'ctx>>,
        module: &'a Module<'ctx>,
    ) -> Self {
        Self {
            context,
            builder,
            fpm,
            module,
        }
    }

    fn compile_fn_proto(
        &self,
        fn_proto: &FunctionPrototype,
    ) -> Result<FunctionValue<'ctx>, String> {
        // TODO: Args
        let fn_type = self
            .context
            .f64_type()
            .fn_type(&Vec::<BasicMetadataTypeEnum>::new(), false);
        let fn_val = self.module.add_function(&fn_proto.symbol, fn_type, None);
        Ok(fn_val)
    }

    fn compile_fn(&self, func: &Node) -> Result<FunctionValue<'ctx>, String> {
        if let Node::FunctionDefinition(fn_proto, _body) = func {
            let function = self.compile_fn_proto(fn_proto)?;

            if function.verify(true) {
                self.fpm.run_on(&function);
            }

            Ok(function)
        } else {
            unimplemented!()
        }
    }

    pub fn compile_program(&self, program: &Program) -> String {
        for member in &program.members {
            match member {
                Node::FunctionDefinition(_, _) => {
                    println!("{:?}", self.compile_fn(member))
                }
                Node::StaticDecl => todo!(),
                _ => panic!("This shouldn't be here"),
            }
        }

        self.module.print_to_string().to_string()
    }
}
