mod interpreter;
mod parser;
mod tree;
use interpreter::interpreter::interpret;
use parser::parser::{parse, Error};
use std::{env, fs};

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("File name not provided!");
    let contents = fs::read_to_string(file_path).expect("File not found!");
    let ast = parse(contents)?;
    let mut mem = Vec::new();
    let output = interpret(&ast, &mut mem, &mut 0);
    dbg!(output);
    dbg!(mem);
    Ok(())
}
