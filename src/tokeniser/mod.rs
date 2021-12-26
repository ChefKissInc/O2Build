use std::str::Chars;

use itertools::Itertools;

use self::{binary::*, ident::*, literals::*};
use crate::{
    next_token,
    token::{Token, TokenPos, TokenisationError},
};

pub mod binary;
pub mod ident;
pub mod literals;

#[macro_export]
macro_rules! incr_pos_by {
    ($pos:ident, $len:expr) => {{
        let pos_before = *$pos;
        $pos.column += $len;
        pos_before
    }};
}

#[inline]
pub fn skip_spaces(pos: &mut TokenPos, it: &mut Chars) {
    it.take_while_ref(|v| *v == ' ')
        .for_each(|_| pos.column += 1);
}

pub trait Tokeniser {
    fn tokenise(&self) -> (Vec<Token>, Vec<TokenisationError>);
}

impl Tokeniser for String {
    #[inline]
    fn tokenise(&self) -> (Vec<Token>, Vec<TokenisationError>) {
        let mut tokens = Vec::new();
        let mut errs = Vec::new();
        let mut it = self.trim().chars();
        let mut pos = TokenPos {
            row: 1,
            ..TokenPos::default()
        };

        loop {
            pos.column += 1;

            let c = next_token!(it, break (tokens, errs));

            match c {
                '\n' => {
                    pos.row += 1;
                    pos.column = 0;
                }
                ' ' => skip_spaces(&mut pos, &mut it),
                '0'..='9' => tokens.push(tokenise_digit(&mut pos, c, &mut it)),
                '"' => tokens.push(tokenise_str(&mut pos, &mut it)),
                '\'' => tokens.push(tokenise_char(&mut pos, &mut it)),
                'A'..='Z' | 'a'..='z' | '_' => {
                    tokens.push(tokenise_identifier(&mut pos, c, &mut it))
                }
                '(' => tokens.push(Token::LeftParen(pos)),
                ')' => tokens.push(Token::RightParen(pos)),
                ':' => tokens.push(Token::Colon(pos)),
                ',' => tokens.push(Token::Comma(pos)),
                '{' => tokens.push(Token::LeftBracket(pos)),
                '}' => tokens.push(Token::RightBracket(pos)),
                '-' => tokens.push(tokenise_minus(&mut pos, &mut it)),
                '+' => tokens.push(tokenise_plus(&mut pos, &mut it)),
                ';' => tokens.push(Token::Semicolon(pos)),
                '/' => {
                    if let Some(t) = tokenise_divide(&mut pos, &mut it) {
                        tokens.push(t)
                    }
                }
                '=' => tokens.push(tokenise_eq(&mut pos, &mut it)),
                '<' => tokens.push(tokenise_less(&mut pos, &mut it)),
                '>' => tokens.push(tokenise_greater(&mut pos, &mut it)),
                _ => errs.push(TokenisationError::UnknownToken(c)),
            }
        }
    }
}
