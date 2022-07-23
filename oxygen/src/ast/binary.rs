//! Copyright (c) ChefKiss Inc 2021-2022.
//! This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives license.

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
