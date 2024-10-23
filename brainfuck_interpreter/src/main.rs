use std::fs;

use clap::Parser;

use brainfuck_interpreter::{args::Args, interpreter::Interpreter};

fn main() {
    let args = Args::parse();

    let file = fs::read_to_string(args.file).expect("Error reading file");

    // println!("{:?}", Interpreter::cache_brackets(file.as_bytes()));

    Interpreter::interpret_brainfuck(file.as_bytes());
}
