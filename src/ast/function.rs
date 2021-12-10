use std::slice::Iter;

use super::Node;
use crate::{abi::Abi, match_token, token::Token};

pub fn parse_args(it: &mut Iter<Token>) -> Result<Vec<Node>, Option<Token>> {
    let ret = vec![];

    loop {
        let token = it.next();

        // If no more arguments, return
        if match_token!(token, Token::RightParen(_), Ok(())).is_ok() {
            break Ok(ret);
        }
    }
}

pub fn parse_abi(it: &mut Iter<Token>) -> Result<Abi, Option<Token>> {
    let token = it.next();

    match_token!(
        token,
        Token::String(_, v),
        match v.as_str() {
            "UEFI" => Ok(Abi::Uefi),
            "SystemV64" => Ok(Abi::SystemV64),
            _ => Err(Some(token.unwrap().clone())),
        }
    )
}

pub fn parse_function_definition(
    public: bool,
    abi: Abi,
    it: &mut Iter<Token>,
) -> Result<Node, Option<Token>> {
    let symbol = match_token!(it.next(), Token::Identifier(_, v), Ok(v))?;
    match_token!(it.next(), Token::LeftParen(_), Ok(()))?;
    let args = parse_args(it)?;

    Ok(Node::FunctionDefinition {
        public,
        symbol: symbol.clone(),
        args,
        abi,
        body: vec![],
    })
}
