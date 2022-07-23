//! Copyright (c) ChefKiss Inc 2021-2022.
//! This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives license.

use std::{fs, io::BufWriter};

use clap::Parser;
use codegen::CodeGen;
use codespan_reporting::{
    diagnostic::{Diagnostic, Label},
    files::SimpleFile,
    term::{
        self,
        termcolor::{ColorChoice, StandardStream},
    },
};
use oxygen::grammar;

use crate::opts::Options;

mod codegen;
mod opts;

fn main() -> anyhow::Result<()> {
    let options = Options::parse();

    let contents = std::fs::read_to_string(&options.src).unwrap();

    match grammar::syntax_tree::parse(&contents) {
        Ok(tree) => {
            println!("{:?}", tree);
            let mut codegen = CodeGen::new(options.to_target_isa(), options.src.to_str().unwrap());
            codegen.gen_module(&tree).unwrap();

            // Generate object file
            let buffer = BufWriter::new(fs::File::create(options.out).unwrap());
            codegen.module.finish().object.write_stream(buffer).unwrap();
        }
        Err(e) => {
            let file = SimpleFile::new(options.src.to_str().unwrap(), contents);
            let start = e.location.offset;
            let diagnostic = Diagnostic::error()
                .with_message("Parse error")
                .with_labels(vec![
                    Label::primary((), start..start).with_message("Unexpected token")
                ])
                .with_notes(vec![format!("Expected {}", e.expected)]);

            term::emit(
                &mut StandardStream::stderr(ColorChoice::Always),
                &term::Config::default(),
                &file,
                &diagnostic,
            )?;
        }
    }

    Ok(())
}
