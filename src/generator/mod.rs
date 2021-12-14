use inkwell::{
    builder::Builder,
    context::Context,
    module::{Linkage, Module},
    passes::PassManager,
    types::BasicMetadataTypeEnum,
    values::{BasicMetadataValueEnum, BasicValue, FunctionValue, IntValue},
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
        linkage: Option<Linkage>,
    ) -> Result<FunctionValue<'ctx>, String> {
        let arg_types = std::iter::repeat(self.context.i64_type())
            .take(fn_proto.args.len())
            .map(|f| f.into())
            .collect::<Vec<BasicMetadataTypeEnum>>();
        let fn_type = self.context.i64_type().fn_type(arg_types.as_slice(), false);
        let fn_val = self.module.add_function(&fn_proto.symbol, fn_type, linkage);

        for (i, arg) in fn_val.get_param_iter().enumerate() {
            if let Node::FunctionArgument(arg_name) = &fn_proto.args[i] {
                arg.into_int_value().set_name(arg_name.as_str());
            } else {
                unreachable!()
            }
        }

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
            Expression::CharLiteral(c) => Ok(self.context.i64_type().const_int(*c as u64, false)),
            Expression::FunctionCall { name, args } => {
                match self.module.get_function(name.as_str()) {
                    Some(func) => {
                        let mut compiled_args = Vec::with_capacity(args.len());

                        for arg in args {
                            compiled_args.push(self.compile_expr(arg)?);
                        }

                        let argsv: Vec<BasicMetadataValueEnum> = compiled_args
                            .iter()
                            .by_ref()
                            .map(|&val| val.into())
                            .collect();

                        match self
                            .builder
                            .build_call(func, argsv.as_slice(), "tmp")
                            .try_as_basic_value()
                            .left()
                        {
                            Some(val) => Ok(val.into_int_value()),
                            None => Err(format!("Invalid call to function: {}", name)),
                        }
                    }
                    None => Err(format!("Function not found: {}", name)),
                }
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
            let function = self.compile_fn_proto(
                fn_proto,
                if fn_proto.public {
                    None
                } else {
                    Some(Linkage::Private)
                },
            )?;

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
                Node::ExternalFunction(fn_proto) => {
                    self.compile_fn_proto(fn_proto, Some(Linkage::External))?;
                }
                Node::StaticDecl => todo!(),
                _ => panic!("This shouldn't be here"),
            }
        }

        Ok(self.module.print_to_string().to_string())
    }
}
