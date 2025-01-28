mod tree;
use std::iter::Peekable;
use std::ops::Deref;
use tree::tree::Tree;

use crate::tree::tree::Node;
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
enum Token {
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
struct Op {
    token: Token,
    count: usize,
    span: Span,
}

fn lex(source: String) -> Vec<Op> {
    let op_map = HashMap::from([
        ('>', Token::Right),
        ('<', Token::Left),
        ('+', Token::Inc),
        ('-', Token::Dec),
        ('.', Token::Output),
        (',', Token::Input),
        ('[', Token::JumpZero),
        (']', Token::JumpNonZero),
    ]);

    let mut tokens = Vec::new();
    for (li, l) in source.split('\n').enumerate() {
        for (ci, c) in l.chars().enumerate() {
            if let Some(t) = op_map.get(&c) {
                tokens.push(Op {
                    token: t.clone(),
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

fn collapse(source: Vec<Op>) -> Vec<Op> {
    let mut collapsed = Vec::new();
    let collapsable = vec![
        Token::Right,
        Token::Left,
        Token::Inc,
        Token::Dec,
        Token::Output,
        Token::Input,
    ];
    let mut count: usize = 1;
    let mut beg: Loc = source
        .get(0)
        .map_or(Loc { line: 0, char: 0 }, |t| t.span.beg);
    for (i, mut t) in source.clone().into_iter().enumerate() {
        if collapsable.contains(&t.token)
            && ({
                let t_next = source.get(i + 1);
                t_next.map_or(false, |t_next| t_next.token == t.token)
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
            count = 1;
        }
    }
    collapsed
}

fn parse_tree<T>(source: &mut Peekable<T>) -> Tree<Op>
where
    T: Iterator<Item = Op>,
{
    let mut tree = Tree::new();
    while let Some(t) = source.next() {
        tree.push(Node::Leaf { value: t.clone() });
        if t.token == Token::JumpZero {
            tree.push(Node::Internal {
                children: parse_tree(source),
            });
        }
        if source
            .peek()
            .map_or(false, |n| (*n).token == Token::JumpNonZero)
        {
            return tree;
        }
    }
    tree
}

fn into_tree(source: Vec<Op>) -> Tree<Op> {
    parse_tree(&mut source.into_iter().peekable())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("File name not provided!");
    let contents = fs::read_to_string(file_path).expect("File not found!");
    let tokens = lex(contents);
    let ops = collapse(tokens);
    let tree = into_tree(ops);
    dbg!(tree);
}
