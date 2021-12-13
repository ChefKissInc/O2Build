use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    passes::PassManager,
    types::BasicMetadataTypeEnum,
    values::{FunctionValue, IntValue},
};

use crate::ast::{
    binary::BinaryOp, expression::Expression, function::FunctionPrototype, Node, SyntaxTree,
};

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
            .i64_type()
            .fn_type(&Vec::<BasicMetadataTypeEnum>::new(), false);
        let fn_val = self.module.add_function(&fn_proto.symbol, fn_type, None);
        Ok(fn_val)
    }

    fn compile_expr(&self, expr: &Expression) -> Result<IntValue<'ctx>, String> {
        match expr {
            Expression::IntegerLiteral(lit) => {
                Ok(self
                    .context
                    .i64_type()
                    .const_int(lit.parse().unwrap(), false))
            }
            Expression::Binary {
                op,
                left_expr,
                right_expr,
            } => {
                let lhs = self.compile_expr(left_expr)?;
                let rhs = self.compile_expr(right_expr)?;

                match op {
                    BinaryOp::Addition => Ok(self.builder.build_int_add(lhs, rhs, "tmpadd")),
                    BinaryOp::Subtraction => Ok(self.builder.build_int_sub(lhs, rhs, "tmpsub")),
                    BinaryOp::Multiplication => Ok(self.builder.build_int_mul(lhs, rhs, "tmpmul")),
                    BinaryOp::Division => Ok(self.builder.build_int_signed_div(lhs, rhs, "tmpdiv")),
                    _ => unimplemented!(),
                }
            }
            _ => unimplemented!(),
        }
    }

    fn compile_fn(&self, func: &Node) -> Result<FunctionValue<'ctx>, String> {
        if let Node::FunctionDefinition(fn_proto, body) = func {
            let function = self.compile_fn_proto(fn_proto)?;

            let entry = self.context.append_basic_block(function, "entry");
            self.builder.position_at_end(entry);

            let body = self.compile_expr(body)?;
            self.builder.build_return(Some(&body));

            if function.verify(true) {
                self.fpm.run_on(&function);

                Ok(function)
            } else {
                Err("Failed to verify function!".to_string())
            }
        } else {
            unimplemented!()
        }
    }

    pub fn compile_program(&self, program: &SyntaxTree) -> Result<String, String> {
        for member in &program.members {
            match member {
                Node::FunctionDefinition(_, _) => {
                    self.compile_fn(member)?;
                }
                Node::StaticDecl => todo!(),
                _ => panic!("This shouldn't be here"),
            }
        }

        Ok(self.module.print_to_string().to_string())
    }
}
