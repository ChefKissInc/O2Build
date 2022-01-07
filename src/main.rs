#![allow(non_snake_case)]

use std::{fs, io::BufWriter};

use cranelift::prelude::*;
use debug_tree::add_branch;
use Oxygen::{ast::SyntaxTree, generator::Generator, token::tokenise};

fn main() {
    (|| {
        let args: Vec<String> = std::env::args().collect();
        assert_eq!(args.len(), 2, "You must provide a path to an oxygen file");
        println!("Infusing: {}", args[1]);

        let contents = fs::read_to_string(&args[1]).expect("Failed to read file");
        add_branch!("Module {}", args[1]);
        // defer_print!();

        // Tokenise text
        let (tokens, errs) = tokenise(contents);
        println!("{:?}\n", tokens);
        println!("Errors: {:?}\n", errs);

        // Parse AST
        let (program, errs) = SyntaxTree::new(tokens);
        println!("{:#?}\n", program);
        println!("Errors: {:?}\n", errs);

        let mut builder = settings::builder();
        builder.set("is_pic", "true").unwrap();
        let target = isa::lookup_by_name("x86_64-apple-darwin")
            .unwrap()
            .finish(settings::Flags::new(builder));

        let mut compiler = Generator::new(target, &args[1]);
        compiler.gen_program(&program)?;
        let buffer = BufWriter::new(fs::File::create("Build/out.o").unwrap());
        compiler
            .module
            .finish()
            .object
            .write_stream(buffer)
            .unwrap();
        Ok(())
    })()
    .map_err(|e: String| {
        println!("Error: {}", e);
    })
    .unwrap();
}
