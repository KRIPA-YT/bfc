pub mod parser {
    use std::{cmp::Ordering, iter::Peekable};

    pub use crate::tree::tree::{Node, Tree};

    #[derive(Debug, Clone, Copy)]
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

        fn into_beg_or_pos(self) -> Pos {
            *self.get_beg_or_pos()
        }

        fn into_end_or_pos(self) -> Pos {
            *self.get_end_or_pos()
        }

        fn get_beg_or_pos(&self) -> &Pos {
            match self {
                Self::Single { pos } | Self::Span { beg: pos, end: _ } => pos,
            }
        }

        fn get_end_or_pos(&self) -> &Pos {
            match self {
                Self::Single { pos } | Self::Span { beg: _, end: pos } => pos,
            }
        }
    }

    #[derive(Debug, PartialOrd, Eq, PartialEq, Clone, Copy)]
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

    #[derive(Debug, PartialEq, Clone, Copy)]
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

        fn is_collapsable(&self) -> bool {
            match self {
                Self::Right | Self::Left | Self::Inc | Self::Dec | Self::Output | Self::Input => {
                    true
                }
                Self::JumpZero | Self::JumpNonZero => false,
            }
        }
    }

    #[derive(Debug, Clone, Copy)]
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
        let mut collapsed: Vec<Op> = Vec::new();
        let mut count: usize = 1;
        let mut beg: &Pos = source
            .get(0)
            .map_or(&Pos { line: 0, char: 0 }, |t| t.loc.get_beg_or_pos());
        for (i, mut t) in source.clone().into_iter().enumerate() {
            if t.token.is_collapsable()
                && ({
                    let t_next = source.get(i + 1);
                    t_next.map_or(false, |t_next| t_next.token == t.token)
                })
            {
                count += 1;
            } else {
                t.count = count;
                let end = t.loc.into_end_or_pos();
                t.loc = Loc::new(*beg, end).unwrap_or(Loc::Single { pos: end });
                collapsed.push(t);
                beg = source
                    .get(i + 1)
                    .map_or(&Pos { line: 0, char: 0 }, |t| t.loc.get_beg_or_pos());
                count = 1;
            }
        }
        collapsed
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
