use std::slice::Iter;

use crate::{ast::Node, token::Token};

pub fn parse_equality(_it: &mut Iter<Token>) -> Result<Node, Option<Token>> {
    Err(None)
}
