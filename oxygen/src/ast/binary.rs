/*
 * Copyright (c) VisualDevelopment 2021-2021.
 * This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.
 */

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOp {
    Assignment,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessOrEqualTo,
    GreaterOrEqualTo,
    Division,
    Multiplication,
    Addition,
    Subtraction,
    None,
}
