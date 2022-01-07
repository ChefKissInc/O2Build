#![allow(non_snake_case)]

pub mod ast;
pub mod generator;
pub mod token;

#[macro_export]
macro_rules! next_token {
    ($it:ident, $($err:tt)*) => {
        match $it.next() {
            Some(v) => v,
            None => $($err)*,
        }
    };
}
