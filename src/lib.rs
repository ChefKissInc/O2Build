#![allow(non_snake_case)]
#![feature(derive_default_enum)]

pub mod abi;
pub mod ast;
pub mod generator;
pub mod token;
pub mod tokeniser;

#[macro_export]
macro_rules! next_token {
    ($it:ident, $($err:tt)*) => {
        match $it.next() {
            Some(v) => v,
            None => $($err)*,
        }
    };
}
