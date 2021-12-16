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
        "Public" => Token::Keyword(incr_pos_by!(pos, ident.len() - 1), Keyword::Public),
        "Extern" => Token::Keyword(incr_pos_by!(pos, ident.len() - 1), Keyword::Extern),
        "CallConv" => Token::Keyword(incr_pos_by!(pos, ident.len() - 1), Keyword::CallConv),
        "Func" => Token::Keyword(incr_pos_by!(pos, ident.len() - 1), Keyword::Function),
        "Let" => Token::Keyword(incr_pos_by!(pos, ident.len() - 1), Keyword::Variable),
        "Mutable" => Token::Keyword(incr_pos_by!(pos, ident.len() - 1), Keyword::Mutable),
        _ => Token::Identifier(incr_pos_by!(pos, ident.len() - 1), ident),
    }
}
