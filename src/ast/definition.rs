use std::slice::Iter;

use debug_tree::add_branch;

use super::function::{parse_abi, parse_func_def};
use crate::{
    abi::Abi,
    ast::Node,
    match_token, next_token,
    token::{Keyword, Token},
};

pub fn parse_definition(public: bool, external: bool, it: &mut Iter<Token>) -> Result<Node, Option<Token>> {
    add_branch!("parse_definition");
    let token = next_token!(it, return Err(None));

    match token {
        Token::Keyword(_, Keyword::Function) => {
            parse_func_def(public, external, Abi::SystemV64, it)
        }
        Token::Keyword(_, Keyword::Abi) => {
            let abi = parse_abi(it)?;
            match_token!(it.next(), Token::Keyword(_, Keyword::Function), Ok(()))?;

            parse_func_def(public, external, abi, it)
        }
        _ => Err(Some(token.clone())),
    }
}
