#![allow(non_snake_case)]

use std::fs;

use debug_tree::add_branch;
use O2Build::{ast::SyntaxTree, generator::Generator, tokeniser::Tokeniser};

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    assert_eq!(args.len(), 2, "You must provide a path to an oxygen file");
    println!("Infusing: {}", args[1]);

    let contents = fs::read_to_string(&args[1]).expect("Failed to read file");
    add_branch!("Module {}", args[1]);
    // defer_print!();

    // Tokenise text
    let (tokens, errs) = contents.tokenise();
    println!("{:?}\n", tokens);
    println!("Errors: {:?}\n", errs);

    // Parse AST
    let (program, errs) = SyntaxTree::new(tokens);
    println!("{:#?}\n", program);
    println!("Errors: {:?}\n", errs);

    let mut compiler = Generator::default();
    compiler.compile_program(&program)?;
    Ok(())
}
