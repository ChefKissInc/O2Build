use std::slice::Iter;

use debug_tree::add_branch;

use crate::{ast::Node, token::Token};

pub fn parse_equality(_it: &mut Iter<Token>) -> Result<Node, Option<Token>> {
    add_branch!("parse_equality");
    Err(None)
}
