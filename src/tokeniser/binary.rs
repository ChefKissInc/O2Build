use std::str::Chars;

use itertools::{Itertools, PeekingNext};

use crate::{
    incr_pos_by,
    token::{Token, TokenPos},
};

#[inline]
pub fn tokenise_minus(pos: &mut TokenPos, it: &mut Chars) -> Token {
    match it.peeking_next(|c| matches!(*c, '>' | '=')) {
        Some('>') => Token::Arrow(incr_pos_by!(pos, 1)),
        Some('=') => Token::MinusEquals(incr_pos_by!(pos, 1)),
        _ => Token::Minus(*pos),
    }
}

#[inline]
pub fn tokenise_plus(pos: &mut TokenPos, it: &mut Chars) -> Token {
    match it.peeking_next(|c| *c == '=') {
        Some('=') => Token::PlusEquals(incr_pos_by!(pos, 1)),
        _ => Token::Plus(*pos),
    }
}

#[inline]
pub fn tokenise_divide(pos: &mut TokenPos, it: &mut Chars) -> Option<Token> {
    match it.peeking_next(|c| *c == '/') {
        Some('/') => {
            it.take_while_ref(|v| *v != '\n')
                .for_each(|_| pos.column += 1);
            pos.column += 1;
            None
        }
        _ => Some(Token::Divide(*pos)),
    }
}

#[inline]
pub fn tokenise_eq(pos: &mut TokenPos, it: &mut Chars) -> Token {
    match it.peeking_next(|c| matches!(*c, '=' | '>')) {
        Some('=') => Token::EqEq(incr_pos_by!(pos, 1)),
        Some('>') => Token::FatArrow(incr_pos_by!(pos, 1)),
        _ => Token::Eq(*pos),
    }
}

#[inline]
pub fn tokenise_less(pos: &mut TokenPos, it: &mut Chars) -> Token {
    match it.peeking_next(|c| *c == '=') {
        Some('=') => Token::LessOrEqual(incr_pos_by!(pos, 1)),
        _ => Token::Less(*pos),
    }
}

#[inline]
pub fn tokenise_greater(pos: &mut TokenPos, it: &mut Chars) -> Token {
    match it.peeking_next(|c| *c == '=') {
        Some('=') => Token::GreaterOrEqual(incr_pos_by!(pos, 1)),
        _ => Token::Greater(*pos),
    }
}
