use std::slice::Iter;

use cranelift::prelude::isa::CallConv;
use debug_tree::add_branch;

use super::{expression::parse_expr, statement::parse_block_expr, typing::Type, Node};
use crate::{match_token, token::Token};

#[derive(Debug, PartialEq)]
pub struct FunctionPrototype {
    pub public: bool,
    pub call_conv: CallConv,
    pub symbol: String,
    pub args: Vec<Node>,
    pub ret_type: Type,
}

pub fn parse_callconv(it: &mut Iter<Token>) -> Result<CallConv, Option<Token>> {
    add_branch!("parse_callconv");
    let token = it.next();

    match_token!(
        token,
        Token::String(_, v),
        match v.as_str() {
            "C" | "SystemV64" => Ok(CallConv::SystemV),
            "UEFI" => Ok(CallConv::WindowsFastcall),
            _ => Err(Some(token.unwrap().clone())),
        }
    )
}

fn parse_args(it: &mut Iter<Token>) -> Result<Vec<Node>, Option<Token>> {
    add_branch!("parse_args");
    let mut ret = vec![];

    loop {
        match it.next() {
            // If no more arguments, return
            Some(Token::RightParen(_)) => break Ok(ret),
            Some(Token::Identifier(_, ident)) => {
                match_token!(it.next(), Token::Colon(_), Ok(()))?;
                ret.push(Node::FunctionArgument(ident.clone(), parse_type(it)?));

                match it.next() {
                    // If no more arguments, return
                    Some(Token::RightParen(_)) => break Ok(ret),
                    Some(Token::Comma(_)) => {}
                    Some(token) => break Err(Some(token.clone())),
                    None => break Err(None),
                }
            }
            Some(token) => break Err(Some(token.clone())),
            None => break Err(None),
        }
    }
}

fn parse_type(it: &mut Iter<Token>) -> Result<Type, Option<Token>> {
    let token = it.next();

    match_token!(
        token,
        Token::Identifier(_, v),
        match v.as_str() {
            "Void" => Ok(Type::Void),
            "Int" => Ok(Type::Int),
            "Str" => Ok(Type::Str),
            _ => Err(Some(token.unwrap().clone())),
        }
    )
}

pub fn parse_func_def(
    public: bool,
    external: bool,
    call_conv: CallConv,
    it: &mut Iter<Token>,
) -> Result<Node, Option<Token>> {
    add_branch!("parse_func_def");
    let symbol = match_token!(it.next(), Token::Identifier(_, v), Ok(v))?.clone();
    match_token!(it.next(), Token::LeftParen(_), Ok(()))?;
    let args = parse_args(it)?;

    let ret_type = if match_token!(it.clone().next(), Token::Arrow(_), Ok(())).is_ok() {
        it.next();
        parse_type(it)?
    } else {
        Type::Void
    };

    if external {
        match_token!(
            it.next(),
            Token::Semicolon(_),
            Ok(Node::ExternalFunction(FunctionPrototype {
                public,
                symbol,
                args,
                call_conv,
                ret_type,
            }))
        )
    } else {
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
                symbol,
                args,
                call_conv,
                ret_type,
            },
            body,
        ))
    }
}
