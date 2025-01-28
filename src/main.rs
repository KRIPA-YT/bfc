use std::{env, fs};

#[derive(Debug)]
struct Loc {
    line: u32,
    char: u32,
}

#[derive(Debug)]
struct Span {
    beg: Loc,
    end: Loc,
}

#[derive(Debug)]
enum Op {
    Right,
    Left,
    Inc,
    Dec,
    Output,
    Input,
    JumpZero,
    JumpNonZero,
}

#[derive(Debug)]
struct Token {
    op: Op,
    count: u32,
    span: Span,
}

fn lex(source: String) -> Vec<Op> {
    source
        .chars()
        .map(|c| {
            if vec!['>', '<', '+', '-', '.', ',', '[', ']'].contains(&c) {
                Some(c)
            } else {
                None
            }
        })
        .flatten()
        .map(|t| match t {
            '>' => Op::Right,
            '<' => Op::Left,
            '+' => Op::Inc,
            '-' => Op::Dec,
            '.' => Op::Output,
            ',' => Op::Input,
            '[' => Op::JumpZero,
            ']' => Op::JumpNonZero,
            _ => {
                panic!("Invalid token, should never happen")
            }
        })
        .collect()
}

fn parse(source: Vec<Op>) -> Vec<Token> {
    todo!();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("File name not provided!");
    let contents = fs::read_to_string(file_path).expect("File not found!");
    let tokens = lex(contents);
    let ops = parse(tokens);
    dbg!(ops);
}
