use std::slice::Iter;

use itertools::PeekingNext;

use crate::{
    ast::binary::{BinaryOp, Precedence},
    match_token,
    token::Token,
};

#[derive(Debug, PartialEq)]
pub enum Expression {
    IntegerLiteral(String),
    StringLiteral(String),
    Parenthesised(Box<Expression>),
    Block(Vec<Expression>),
    FunctionCall {
        name: String,
        args: Vec<Expression>,
    },
    Binary {
        op: super::binary::BinaryOp,
        left_expr: Box<Expression>,
        right_expr: Box<Expression>,
    },
}

pub fn parse_expr(it: &mut Iter<Token>) -> Result<Expression, Option<Token>> {
    let left_expr = parse_left_expr(it)?;
    parse_right_expr(it, 0, left_expr)
}

pub fn parse_left_expr(it: &mut Iter<Token>) -> Result<Expression, Option<Token>> {
    let token = it.next();

    match token {
        Some(Token::Integer(_, t)) => Ok(Expression::IntegerLiteral(t.clone())),
        Some(Token::String(_, t)) => Ok(Expression::StringLiteral(t.clone())),
        Some(Token::LeftParen(_)) => parse_parenthesised_expr(it),
        Some(Token::Plus(_) | Token::Minus(_)) => todo!(),
        Some(token) => Err(Some(token.clone())),
        None => Err(None),
    }
}

pub fn parse_parenthesised_expr(it: &mut Iter<Token>) -> Result<Expression, Option<Token>> {
    let expr = Box::new(parse_expr(it)?);
    match_token!(it.next(), Token::RightParen(_), Ok(()))?;
    Ok(Expression::Parenthesised(expr))
}

pub fn parse_right_expr(
    it: &mut Iter<Token>,
    curr_precedence: i8,
    mut left_expr: Expression,
) -> Result<Expression, Option<Token>> {
    loop {
        let op = match it.peeking_next(|_| false) {
            Some(Token::Plus(_)) => BinaryOp::Addition,
            Some(Token::Minus(_)) => BinaryOp::Subtraction,
            Some(Token::Times(_)) => BinaryOp::Multiplication,
            Some(Token::Divide(_)) => BinaryOp::Division,
            Some(_) => BinaryOp::None,
            None => return Err(None),
        };

        let precedence = op.get_precedence();

        if precedence < curr_precedence {
            return Ok(left_expr);
        }

        it.next();
        let mut right_expr = parse_left_expr(it)?;

        if curr_precedence
            < match it.peeking_next(|_| false) {
                Some(Token::Plus(_)) => BinaryOp::Addition,
                Some(Token::Minus(_)) => BinaryOp::Subtraction,
                Some(Token::Times(_)) => BinaryOp::Multiplication,
                Some(Token::Divide(_)) => BinaryOp::Division,
                Some(_) => BinaryOp::None,
                None => return Err(None),
            }
            .get_precedence()
        {
            right_expr = parse_right_expr(it, curr_precedence + 1, right_expr)?;
        }

        left_expr = Expression::Binary {
            op,
            left_expr: Box::new(left_expr),
            right_expr: Box::new(right_expr),
        };
    }
}
