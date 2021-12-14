use std::slice::Iter;

use debug_tree::add_branch;

use crate::{
    ast::binary::{BinaryOp, Precedence},
    match_token,
    token::Token,
};

#[derive(Debug, PartialEq)]
pub enum Expression {
    IntegerLiteral(String),
    StringLiteral(String),
    CharLiteral(char),
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
    Unary {
        op: super::binary::BinaryOp,
        expr: Box<Expression>,
    },
}

pub fn parse_expr(it: &mut Iter<Token>) -> Result<Expression, Option<Token>> {
    add_branch!("parse_expr");
    let left_expr = parse_left_expr(it)?;

    parse_right_expr(it, 0, left_expr)
}

pub fn parse_left_expr(it: &mut Iter<Token>) -> Result<Expression, Option<Token>> {
    add_branch!("parse_left_expr");
    let token = it.next();

    match token {
        Some(Token::Integer(_, t)) => Ok(Expression::IntegerLiteral(t.clone())),
        Some(Token::String(_, t)) => Ok(Expression::StringLiteral(t.clone())),
        Some(Token::Char(_, t)) => Ok(Expression::CharLiteral(*t)),
        Some(Token::Identifier(_, ident)) => {
            match_token!(it.next(), Token::LeftParen(_), Ok(()))?;

            let mut args = vec![];

            loop {
                args.push(parse_expr(it)?);

                match it.next() {
                    // If no more arguments, return
                    Some(Token::RightParen(_)) => {
                        break Ok(Expression::FunctionCall {
                            name: ident.clone(),
                            args,
                        })
                    }
                    Some(Token::Comma(_)) => continue,
                    Some(token) => break Err(Some(token.clone())),
                    None => break Err(None),
                }
            }
        }
        Some(Token::LeftParen(_)) => parse_parenthesised_expr(it),
        Some(Token::Plus(_)) => parse_unary_expr(it, BinaryOp::Addition),
        Some(Token::Minus(_)) => parse_unary_expr(it, BinaryOp::Subtraction),
        Some(token) => Err(Some(token.clone())),
        None => Err(None),
    }
}

pub fn parse_parenthesised_expr(it: &mut Iter<Token>) -> Result<Expression, Option<Token>> {
    add_branch!("parse_parenthesised_expr");
    let expr = Box::new(parse_expr(it)?);
    match_token!(it.next(), Token::RightParen(_), Ok(()))?;
    Ok(Expression::Parenthesised(expr))
}

pub fn parse_right_expr(
    it: &mut Iter<Token>,
    precedence: i8,
    mut left_expr: Expression,
) -> Result<Expression, Option<Token>> {
    add_branch!("parse_right_expr");

    loop {
        let op = match it.clone().next() {
            Some(Token::Plus(_)) => BinaryOp::Addition,
            Some(Token::Minus(_)) => BinaryOp::Subtraction,
            Some(Token::Times(_)) => BinaryOp::Multiplication,
            Some(Token::Divide(_)) => BinaryOp::Division,
            Some(_) => BinaryOp::None,
            None => break Err(None),
        };

        let curr_precedence = op.get_precedence();

        if curr_precedence < precedence {
            break Ok(left_expr);
        }

        it.next();
        add_branch!("it.next");
        let mut right_expr = parse_left_expr(it)?;

        if curr_precedence
            < match it.clone().next() {
                Some(Token::Plus(_)) => BinaryOp::Addition,
                Some(Token::Minus(_)) => BinaryOp::Subtraction,
                Some(Token::Times(_)) => BinaryOp::Multiplication,
                Some(Token::Divide(_)) => BinaryOp::Division,
                Some(_) => BinaryOp::None,
                None => break Err(None),
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

pub fn parse_unary_expr(it: &mut Iter<Token>, op: BinaryOp) -> Result<Expression, Option<Token>> {
    add_branch!("parse_unary_expr");
    let expr = Box::new(parse_left_expr(it)?);
    Ok(Expression::Unary { op, expr })
}
