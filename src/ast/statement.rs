use std::slice::Iter;

use debug_tree::add_branch;

use super::expression::Expression;
use crate::{ast::expression::parse_expr, token::Token};

pub fn parse_block_expr(it: &mut Iter<Token>) -> Result<Expression, Option<Token>> {
    add_branch!("parse_block_expr");
    let mut ret = vec![];

    match it.clone().next() {
        // If no expressions, return
        Some(Token::RightBracket(_)) => {
            it.next();
            return Ok(Expression::Block(ret));
        }
        Some(_) => {}
        None => return Err(None),
    }

    loop {
        ret.push(parse_expr(it)?);

        match it.next() {
            Some(Token::Semicolon(_)) => {
                match it.clone().next() {
                    // If no more expressions, return
                    Some(Token::RightBracket(_)) => {
                        it.next();
                        break Ok(Expression::Block(ret));
                    }
                    Some(_) => continue,
                    None => break Err(None),
                }
            }
            Some(token) => break Err(Some(token.clone())),
            None => break Err(None),
        }
    }
}
