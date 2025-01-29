pub mod parser {
    use itertools::Itertools;
    use std::{cmp::Ordering, iter::Peekable};

    pub use crate::tree::tree::{Node, Tree};

    #[derive(Debug, Clone, PartialEq)]
    pub enum Loc {
        Single { pos: Pos },
        Span { beg: Pos, end: Pos },
    }

    impl Loc {
        pub fn new(beg: Pos, end: Pos) -> Option<Self> {
            match std::cmp::Ord::cmp(&beg, &end) {
                Ordering::Less => Some(Self::Span { beg, end }),
                Ordering::Equal => {
                    Some(Self::Single { pos: beg }) // Doesn't matter which one since they're equal anyway
                }
                Ordering::Greater => None,
            }
        }

        pub fn new_min(beg: Pos, end: Pos) -> Self {
            match std::cmp::Ord::cmp(&beg, &end) {
                Ordering::Less => Self::Span { beg, end },
                Ordering::Equal | Ordering::Greater => Self::Single { pos: beg },
            }
        }

        pub fn into_beg_or_pos(self) -> Pos {
            match self {
                Self::Single { pos } | Self::Span { beg: pos, end: _ } => pos,
            }
        }

        pub fn into_end_or_pos(self) -> Pos {
            match self {
                Self::Single { pos } | Self::Span { beg: _, end: pos } => pos,
            }
        }

        pub fn get_beg_or_pos(&self) -> &Pos {
            match self {
                Self::Single { pos } | Self::Span { beg: pos, end: _ } => pos,
            }
        }

        pub fn get_end_or_pos(&self) -> &Pos {
            match self {
                Self::Single { pos } | Self::Span { beg: _, end: pos } => pos,
            }
        }
    }

    #[derive(Debug, PartialOrd, Eq, PartialEq, Clone)]
    pub struct Pos {
        line: usize,
        char: usize,
    }

    impl Ord for Pos {
        fn cmp(&self, other: &Self) -> Ordering {
            match std::cmp::Ord::cmp(&self.line, &other.line) {
                Ordering::Equal => std::cmp::Ord::cmp(&self.char, &other.char),
                ord => ord,
            }
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum Token {
        Right,
        Left,
        Inc,
        Dec,
        Output,
        Input,
        JumpZero,
        JumpNonZero,
    }

    impl Token {
        fn new(c: char) -> Option<Token> {
            match c {
                '>' => Some(Token::Right),
                '<' => Some(Token::Left),
                '+' => Some(Token::Inc),
                '-' => Some(Token::Dec),
                '.' => Some(Token::Output),
                ',' => Some(Token::Input),
                '[' => Some(Token::JumpZero),
                ']' => Some(Token::JumpNonZero),
                _ => None,
            }
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Op {
        token: Token,
        count: usize,
        loc: Loc,
    }

    fn lex(source: String) -> Vec<Op> {
        source
            .split('\n')
            .enumerate()
            .flat_map(|(li, l)| {
                l.chars()
                    .enumerate()
                    .filter_map(|(ci, c)| Token::new(c).map(|t| (ci, t)))
                    .map(move |(ci, token)| Op {
                        token,
                        count: 1,
                        loc: Loc::Single {
                            pos: Pos { line: li, char: ci },
                        },
                    })
            })
            .collect()
    }

    fn collapse(source: Vec<Op>) -> Vec<Op> {
        source
            .into_iter()
            .chunk_by(|x| x.token.clone())
            .into_iter()
            .flat_map(|(token, chunk)| {
                let mut ops: Vec<Op> = chunk.collect_vec();
                match token {
                    Token::Right
                    | Token::Left
                    | Token::Inc
                    | Token::Dec
                    | Token::Output
                    | Token::Input => {
                        vec![Op {
                            token,
                            count: ops.len(),
                            loc: Loc::new_min(
                                ops.remove(0).loc.into_beg_or_pos(),
                                ops.pop().map_or(Pos { line: 0, char: 0 }, |op| {
                                    op.loc.into_end_or_pos()
                                }),
                            ),
                        }]
                    }
                    Token::JumpZero | Token::JumpNonZero => ops,
                }
            })
            .collect()
    }

    #[derive(Debug)]
    pub enum ParseErr {
        UnmatchedBracket(Option<Pos>),
    }

    fn parse_tree<T>(source: &mut Peekable<T>, depth: usize) -> Result<Tree<Op>, ParseErr>
    where
        T: Iterator<Item = Op>,
    {
        let mut tree = Tree::new();
        while let Some(t) = source.next() {
            tree.push(Node::Leaf(t.clone()));
            if t.token == Token::JumpZero {
                tree.push(Node::Internal(match parse_tree(source, depth + 1) {
                    Ok(token) => token,
                    Err(parse_err) => match parse_err {
                        ParseErr::UnmatchedBracket(_) => {
                            return Err(ParseErr::UnmatchedBracket(Some(t.loc.into_beg_or_pos())));
                        }
                    },
                }));
            }
            if t.token == Token::JumpNonZero {
                return if depth > 0 {
                    Ok(tree)
                } else {
                    Err(ParseErr::UnmatchedBracket(Some(t.loc.into_beg_or_pos())))
                };
            }
        }
        if depth == 0 {
            Ok(tree)
        } else {
            Err(ParseErr::UnmatchedBracket(None))
        }
    }

    fn into_tree(source: Vec<Op>) -> Result<Tree<Op>, ParseErr> {
        parse_tree(&mut source.into_iter().peekable(), 0)
    }

    #[derive(Debug)]
    pub enum Error {
        ParseErr(ParseErr),
    }

    impl From<ParseErr> for Error {
        fn from(err: ParseErr) -> Self {
            Self::ParseErr(err)
        }
    }

    pub fn parse(source: String) -> Result<Tree<Op>, Error> {
        let tokens = lex(source);
        let ops = collapse(tokens);
        let tree = into_tree(ops)?;
        Ok(tree)
    }
}
