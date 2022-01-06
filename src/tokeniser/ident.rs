use std::str::Chars;

use itertools::Itertools;

use crate::{
    incr_pos_by,
    token::{Keyword, Token, TokenPos},
};

#[inline]
pub fn tokenise_identifier(pos: &mut TokenPos, c: char, it: &mut Chars) -> Token {
    let ident = c.to_string()
        + &it
            .take_while_ref(|v| matches!(*v, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_'))
            .collect::<String>();

    match ident.as_str() {
        "public" => Token::Keyword(incr_pos_by!(pos, ident.len() - 1), Keyword::Public),
        "extern" => Token::Keyword(incr_pos_by!(pos, ident.len() - 1), Keyword::Extern),
        "callconv" => Token::Keyword(incr_pos_by!(pos, ident.len() - 1), Keyword::CallConv),
        "func" => Token::Keyword(incr_pos_by!(pos, ident.len() - 1), Keyword::Function),
        "let" => Token::Keyword(incr_pos_by!(pos, ident.len() - 1), Keyword::Variable),
        "mutable" => Token::Keyword(incr_pos_by!(pos, ident.len() - 1), Keyword::Mutable),
        _ => Token::Identifier(incr_pos_by!(pos, ident.len() - 1), ident),
    }
}
