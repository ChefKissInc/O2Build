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
