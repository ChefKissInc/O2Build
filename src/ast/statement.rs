use std::slice::Iter;

use super::expression::{parse_expr, Expression};
use crate::{match_token, token::Token};

// pub fn parse_block_expr(it: &mut Iter<Token>) -> Result<Expression, Option<Token>> {
//     let mut ret = vec![];
//     loop {
//         let expr = Box::new(parse_expr(it)?);
//         match_token!(it.next(), Token::Semicolon(_), Ok(())).;
//     }
// }
