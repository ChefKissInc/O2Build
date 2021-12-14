use debug_tree::add_branch;

use self::{
    definition::parse_def,
    expression::Expression,
    function::{parse_callconv, parse_func_def, FunctionPrototype},
};
use crate::{
    abi::CallingConv,
    next_token,
    token::{Keyword, Token},
};

pub mod binary;
pub mod comparison;
pub mod definition;
pub mod expression;
pub mod function;
pub mod statement;

#[macro_export]
macro_rules! match_token {
    ($next:expr, $($expected:pat_param)|+, $($ret:tt)*) => {{
        add_branch!("match_token");
        match $next {
            Some($($expected)|+) => $($ret)*,
            Some(token) => Err(Some(token.clone())),
            None => Err(None),
        }
    }};
}

#[derive(Debug, PartialEq)]
pub struct SyntaxTree {
    pub members: Vec<Node>,
}

#[derive(Debug, PartialEq)]
pub enum Node {
    Expression(Expression),
    FunctionArgument(String),
    StaticDecl,
    FunctionDefinition(FunctionPrototype, Expression),
    ExternalFunction(FunctionPrototype),
}

impl SyntaxTree {
    pub fn new(tokens: Vec<Token>) -> (Self, Vec<Option<Token>>) {
        add_branch!("Program::new");
        let mut members = Vec::new();
        let mut errs = Vec::new();
        let mut it = tokens.iter();

        loop {
            let token = next_token!(it, break (Self { members }, errs));

            match token {
                Token::Keyword(_, Keyword::Public) => {
                    parse_def(true, false, &mut it)
                        .map_or_else(|e| errs.push(e), |v| members.push(v))
                }
                Token::Keyword(_, Keyword::Function) => {
                    parse_func_def(false, false, CallingConv::SystemV64, &mut it)
                        .map_or_else(|e| errs.push(e), |v| members.push(v))
                }
                Token::Keyword(_, Keyword::Extern) => {
                    parse_def(true, true, &mut it)
                        .map_or_else(|e| errs.push(e), |v| members.push(v))
                }
                Token::Keyword(_, Keyword::CallConv) => {
                    parse_callconv(&mut it)
                        .and_then(|abi| {
                            match_token!(it.next(), Token::Keyword(_, Keyword::Function), Ok(()))
                                .and_then(|_| {
                                    parse_func_def(false, false, abi, &mut it)
                                        .map(|v| members.push(v))
                                })
                        })
                        .unwrap_or_else(|e| errs.push(e));
                }
                _ => errs.push(Some(token.clone())),
            }
        }
    }
}
