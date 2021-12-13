use std::slice::Iter;

use debug_tree::add_branch;
use itertools::PeekingNext;

use super::expression::Expression;
use crate::token::Token;

pub fn parse_statement(_it: &mut Iter<Token>) -> Result<Expression, Option<Token>> {
    add_branch!("parse_statement");
    Err(None)
}

pub fn parse_block_expr(it: &mut Iter<Token>) -> Result<Expression, Option<Token>> {
    add_branch!("parse_block_expr");
    let mut ret = vec![];

    while it
        .peeking_next(|t| matches!(**t, Token::RightBracket(_)))
        .is_none()
    {
        ret.push(parse_statement(it)?)
    }

    Ok(Expression::Block(ret))
}
