mod parser;
mod tree;
use parser::parser::parse;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("File name not provided!");
    let contents = fs::read_to_string(file_path).expect("File not found!");

    dbg!(parse(contents));
}
