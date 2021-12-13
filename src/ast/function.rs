use std::slice::Iter;

use debug_tree::add_branch;

use super::{expression::parse_expr, statement::parse_block_expr, Node};
use crate::{abi::Abi, match_token, token::Token};

#[derive(Debug, PartialEq)]
pub struct FunctionPrototype {
    pub public: bool,
    pub abi: Abi,
    pub symbol: String,
    pub args: Vec<Node>,
}

pub fn parse_args(it: &mut Iter<Token>) -> Result<Vec<Node>, Option<Token>> {
    add_branch!("parse_args");
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
    add_branch!("parse_abi");
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
    add_branch!("parse_function_definition");
    let symbol = match_token!(it.next(), Token::Identifier(_, v), Ok(v))?;
    match_token!(it.next(), Token::LeftParen(_), Ok(()))?;
    let args = parse_args(it)?;

    let body = match it.next() {
        Some(Token::FatArrow(_)) => {
            let ret = parse_expr(it);
            match_token!(it.next(), Token::Semicolon(_), Ok(()))?;
            ret
        }
        Some(Token::LeftBracket(_)) => parse_block_expr(it),
        Some(token) => Err(Some(token.clone())),
        None => Err(None),
    }?;

    Ok(Node::FunctionDefinition(
        FunctionPrototype {
            public,
            symbol: symbol.clone(),
            args,
            abi,
        },
        body,
    ))
}
