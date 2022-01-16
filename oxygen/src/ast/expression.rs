/*
 * Copyright (c) VisualDevelopment 2021-2021.
 * This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.
 */

use super::binary::BinaryOp;

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    IntegerLiteral(String),
    StringLiteral(String),
    CharLiteral(char),
    Parenthesised(Box<Expression>),
    Block(Vec<Expression>),
    FunctionCall {
        name: String,
        args: Vec<Expression>,
    },
    Binary {
        op: BinaryOp,
        left_expr: Box<Expression>,
        right_expr: Box<Expression>,
    },
    Unary {
        op: BinaryOp,
        expr: Box<Expression>,
    },
}
