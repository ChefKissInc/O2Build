/*
 * Copyright (c) VisualDevelopment 2021-2021.
 * This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.
 */

use self::{expression::Expression, function::FunctionPrototype, typing::Type};

pub mod binary;
pub mod expression;
pub mod function;
pub mod typing;

#[derive(Debug, PartialEq)]
pub struct SyntaxTree {
    pub members: Vec<Node>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Expression(Expression),
    FunctionArgument(String, Type),
    StaticDecl,
    FunctionDefinition(FunctionPrototype, Expression),
    ExternalFunction(FunctionPrototype),
}
