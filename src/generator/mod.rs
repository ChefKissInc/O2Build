use inkwell::{
    builder::Builder,
    context::Context,
    module::{Linkage, Module},
    passes::PassManager,
    targets::TargetData,
    types::BasicMetadataTypeEnum,
    values::{BasicMetadataValueEnum, BasicValue, BasicValueEnum, FunctionValue},
    AddressSpace,
};

use crate::ast::{
    binary::BinaryOp, expression::Expression, function::FunctionPrototype, typing::Type, Node,
    SyntaxTree,
};

pub struct Compiler<'a, 'ctx> {
    pub context: &'ctx Context,
    pub builder: &'a Builder<'ctx>,
    pub fpm: &'a PassManager<FunctionValue<'ctx>>,
    pub module: &'a Module<'ctx>,
    pub target_data: TargetData,
}

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn new(
        context: &'ctx Context,
        builder: &'a Builder<'ctx>,
        fpm: &'a PassManager<FunctionValue<'ctx>>,
        module: &'a Module<'ctx>,
        target_data: TargetData,
    ) -> Self {
        Self {
            context,
            builder,
            fpm,
            module,
            target_data,
        }
    }

    fn compile_fn_proto(
        &self,
        fn_proto: &FunctionPrototype,
        linkage: Option<Linkage>,
    ) -> Result<FunctionValue<'ctx>, String> {
        let arg_types = fn_proto
            .args
            .iter()
            .map(|arg| {
                if let Node::FunctionArgument(_, type_) = arg {
                    match type_ {
                        Type::Int => self.context.i64_type().into(),
                        Type::Str => {
                            self.context
                                .i8_type()
                                .ptr_type(AddressSpace::Generic)
                                .into()
                        }
                        _ => unreachable!(),
                    }
                } else {
                    unreachable!()
                }
            })
            .collect::<Vec<BasicMetadataTypeEnum>>();
        let fn_type = match fn_proto.ret_type {
            Type::Void => {
                self.context
                    .void_type()
                    .fn_type(arg_types.as_slice(), false)
            }
            Type::Int => self.context.i64_type().fn_type(arg_types.as_slice(), false),
            Type::Str => {
                self.context
                    .i8_type()
                    .ptr_type(AddressSpace::Generic)
                    .fn_type(arg_types.as_slice(), false)
            }
        };
        let fn_val = self.module.add_function(&fn_proto.symbol, fn_type, linkage);
        fn_val.set_call_conventions(fn_proto.call_conv as u32);

        for (i, arg) in fn_val.get_param_iter().enumerate() {
            if let Node::FunctionArgument(arg_name, ty_) = &fn_proto.args[i] {
                match ty_ {
                    Type::Int => arg.into_int_value().set_name(arg_name.as_str()),
                    Type::Str => arg.into_pointer_value().set_name(arg_name.as_str()),
                    _ => unreachable!(),
                }
            } else {
                unreachable!()
            }
        }

        Ok(fn_val)
    }

    fn compile_expr(&self, expr: &Expression) -> Result<Option<BasicValueEnum<'ctx>>, String> {
        match expr {
            Expression::IntegerLiteral(lit) => {
                Ok(Some(
                    self.context
                        .i64_type()
                        .const_int(lit.parse().unwrap(), false)
                        .into(),
                ))
            }
            Expression::CharLiteral(c) => {
                Ok(Some(
                    self.context.i8_type().const_int(*c as u64, false).into(),
                ))
            }
            Expression::StringLiteral(s) => {
                Ok(Some(
                    self.builder
                        .build_global_string_ptr(s.as_str(), "str")
                        .as_basic_value_enum(),
                ))
            }
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
                            .map(|&val| val.unwrap().into())
                            .collect();

                        Ok(self
                            .builder
                            .build_call(func, argsv.as_slice(), "tmp")
                            .try_as_basic_value()
                            .left())
                    }
                    None => Err(format!("Function not found: {}", name)),
                }
            }
            Expression::Block(exprs) => {
                for expr in exprs {
                    self.compile_expr(expr)?;
                }
                Ok(None)
            }
            Expression::Binary {
                op,
                left_expr,
                right_expr,
            } => {
                let lhs = self.compile_expr(left_expr).map(|v| {
                    match v {
                        Some(v) => Ok(v),
                        None => Err("Void isn't a BasicValue"),
                    }
                })??;
                let rhs = self.compile_expr(right_expr).map(|v| {
                    match v {
                        Some(v) => Ok(v),
                        None => Err("Void isn't a BasicValue"),
                    }
                })??;

                match op {
                    BinaryOp::Addition => {
                        Ok(Some(
                            self.builder
                                .build_int_add(lhs.into_int_value(), rhs.into_int_value(), "tmpadd")
                                .as_basic_value_enum(),
                        ))
                    }
                    BinaryOp::Subtraction => {
                        Ok(Some(
                            self.builder
                                .build_int_sub(lhs.into_int_value(), rhs.into_int_value(), "tmpsub")
                                .as_basic_value_enum(),
                        ))
                    }
                    BinaryOp::Multiplication => {
                        Ok(Some(
                            self.builder
                                .build_int_mul(lhs.into_int_value(), rhs.into_int_value(), "tmpmul")
                                .as_basic_value_enum(),
                        ))
                    }
                    BinaryOp::Division => {
                        Ok(Some(
                            self.builder
                                .build_int_signed_div(
                                    lhs.into_int_value(),
                                    rhs.into_int_value(),
                                    "tmpdiv",
                                )
                                .as_basic_value_enum(),
                        ))
                    }
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

            match self.compile_expr(body)? {
                Some(v) => self.builder.build_return(Some(&v)),
                None => self.builder.build_return(None),
            };

            if function.verify(true) {
                self.fpm.run_on(&function);

                Ok(function)
            } else {
                Err(format!("Failed to verify function {}!", fn_proto.symbol))
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
