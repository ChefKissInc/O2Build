#![allow(non_snake_case)]
#![feature(path_file_prefix)]

use std::path::Path;

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

pub fn get_config(file_name: &str) -> Option<&str> {
    Path::new(file_name).file_prefix()?.to_str()
}
