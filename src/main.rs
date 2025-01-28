use std::collections::HashMap;
use std::{env, fs};

#[derive(Debug, Clone, Copy)]
struct Loc {
    line: usize,
    char: usize,
}

#[derive(Debug, Clone, Copy)]
struct Span {
    beg: Loc,
    end: Loc,
}

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug, Clone, Copy)]
struct Token {
    op: Op,
    count: usize,
    span: Span,
}

fn lex(source: String) -> Vec<Token> {
    let op_map = HashMap::from([
        ('>', Op::Right),
        ('<', Op::Left),
        ('+', Op::Inc),
        ('-', Op::Dec),
        ('.', Op::Output),
        (',', Op::Input),
        ('[', Op::JumpZero),
        (']', Op::JumpNonZero),
    ]);

    let mut tokens = Vec::new();
    for (li, l) in source.split('\n').enumerate() {
        for (ci, c) in l.chars().enumerate() {
            if let Some(t) = op_map.get(&c) {
                tokens.push(Token {
                    op: *t,
                    count: 1,
                    span: Span {
                        beg: Loc { line: li, char: ci },
                        end: Loc { line: li, char: ci },
                    },
                })
            }
        }
    }
    tokens
}

fn collapse(source: Vec<Token>) -> Vec<Token> {
    let mut collapsed = Vec::new();
    let collapsable = vec![Op::Right, Op::Left, Op::Inc, Op::Dec, Op::Output, Op::Input];
    let mut count: usize = 1;
    let mut beg: Loc = source
        .get(0)
        .map_or(Loc { line: 0, char: 0 }, |t| t.span.beg);
    for (i, mut t) in source.clone().into_iter().enumerate() {
        if collapsable.contains(&t.op)
            && ({
                let t_next = source.get(i + 1);
                t_next.map_or(false, |t_next| t_next.op == t.op)
            })
        {
            count += 1;
        } else {
            t.count = count;
            t.span = Span {
                beg: beg.clone(),
                end: t.span.end,
            };
            collapsed.push(t);
            beg = source
                .get(i + 1)
                .map_or(Loc { line: 0, char: 0 }, |t| t.span.beg);
            dbg!(beg);
            count = 1;
        }
    }
    collapsed
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("File name not provided!");
    let contents = fs::read_to_string(file_path).expect("File not found!");
    let tokens = lex(contents);
    let ops = collapse(tokens);
    dbg!(ops);
}
