mod ast;
mod tokenizer;
mod util;

use ast::Ast;
use std::{fs::File, io::Write};
use tokenizer::Tokenizer;
use util::read_file;

fn main() {
    let characters = read_file("program.txt").expect("failed to read program");

    let mut tokenizer = Tokenizer::new(characters);
    if let Err(e) = tokenizer.tokenize() {
        println!("{tokenizer:#?}");
        println!("{e}");
        return;
    }

    let mut ast = Ast::new(tokenizer);
    if let Err(e) = ast.build() {
        println!("{ast:#?}");
        println!("{e}");
        return;
    }

    let mut file = File::create("ast.txt").expect("failed to create file");
    write!(file, "{ast:#?}").expect("failed to write to file");
}
