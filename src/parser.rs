pub mod parser {
    use std::iter::Peekable;

    pub use crate::tree::tree::{Node, Tree};

    #[derive(Debug, Clone, Copy)]
    pub struct Loc {
        line: usize,
        char: usize,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Span {
        beg: Loc,
        end: Loc,
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
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
        span: Span,
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
                        span: Span {
                            beg: Loc {
                                line: li.clone(),
                                char: ci.clone(),
                            },
                            end: Loc {
                                line: li.clone(),
                                char: ci.clone(),
                            },
                        },
                    })
            })
            .collect()
    }

    fn collapse(source: Vec<Op>) -> Vec<Op> {
        let mut collapsed = Vec::new();
        let mut count: usize = 1;
        let mut beg: Loc = source
            .get(0)
            .map_or(Loc { line: 0, char: 0 }, |t| t.span.beg);
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

    #[derive(Debug)]
    pub enum ParseErr {
        UnmatchedBracket(Option<Loc>),
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
                            return Err(ParseErr::UnmatchedBracket(Some(t.span.beg)));
                        }
                    },
                }));
            }
            if t.token == Token::JumpNonZero {
                return if depth > 0 {
                    Ok(tree)
                } else {
                    Err(ParseErr::UnmatchedBracket(Some(t.span.beg)))
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
