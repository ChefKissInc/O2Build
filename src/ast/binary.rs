use debug_tree::add_branch;

#[derive(Debug, PartialEq)]
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

pub trait Precedence {
    fn get_precedence(&self) -> i8;
}

impl Precedence for BinaryOp {
    fn get_precedence(&self) -> i8 {
        add_branch!("get_precedence");
        match *self {
            Self::Assignment => 0,
            Self::Equal
            | Self::NotEqual
            | Self::LessThan
            | Self::GreaterThan
            | Self::LessOrEqualTo
            | Self::GreaterOrEqualTo => 10,
            Self::Addition | Self::Subtraction => 20,
            Self::Multiplication | Self::Division => 30,
            Self::None => -1,
        }
    }
}
