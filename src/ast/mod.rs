use self::{
    definition::parse_definition,
    expression::Expression,
    function::{parse_abi, parse_function_definition},
};
use crate::{
    abi::Abi,
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
    ($next:expr, $($expected:pat_param)|+, $($ret:tt)*) => {
        match $next {
            Some($($expected)|+) => $($ret)*,
            Some(token) => Err(Some(token.clone())),
            None => Err(None),
        }
    };
}

#[derive(Debug, PartialEq)]
pub struct CompilationUnit {
    pub members: Vec<Node>,
}

#[derive(Debug, PartialEq)]
pub enum Node {
    Expression(Expression),
    FunctionArgument,
    StaticDecl,
    FunctionDefinition {
        public: bool,
        abi: Abi,
        symbol: String,
        args: Vec<Node>,
        body: Vec<Node>,
    },
    ExternalFunction {
        public: bool,
        abi: Abi,
        symbol: String,
        args: Vec<Node>,
    },
}

impl CompilationUnit {
    pub fn new(tokens: Vec<Token>) -> (Self, Vec<Option<Token>>) {
        let mut members = Vec::new();
        let mut errs = Vec::new();
        let mut it = tokens.iter();

        loop {
            let token = next_token!(it, break (Self { members }, errs));

            match token {
                Token::Keyword(_, Keyword::Public) => {
                    parse_definition(true, &mut it)
                        .map_or_else(|e| errs.push(e), |v| members.push(v))
                }
                Token::Keyword(_, Keyword::Function) => {
                    parse_definition(false, &mut it)
                        .map_or_else(|e| errs.push(e), |v| members.push(v))
                }
                Token::Keyword(_, Keyword::Abi) => {
                    parse_abi(&mut it)
                        .and_then(|abi| {
                            match_token!(it.next(), Token::Keyword(_, Keyword::Function), Ok(()))
                                .and_then(|_| {
                                    parse_function_definition(false, abi, &mut it)
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
