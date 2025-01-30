pub mod interpreter {
    use std::u8;

    use crate::parser::parser::{Op, Token};
    use crate::tree::tree::{Node, Tree};
    pub fn interpret(ast: &Tree<Op>, memlayout: &mut Vec<u8>, ptr: &mut usize) -> Vec<char> {
        let mut output = Vec::new();
        let mut ast_iter = ast.iter();
        while let Some(op) = ast_iter.next() {
            match op {
                Node::Internal(branch) => {
                    interpret(branch, memlayout, ptr);
                }
                Node::Leaf(op) => match op.token {
                    Token::Inc => {
                        let data = get_mut_or_insert(memlayout, ptr.clone(), 0);
                        *data = data.wrapping_add(op.count as u8);
                    }
                    Token::Dec => {
                        let data = get_mut_or_insert(memlayout, ptr.clone(), 0);
                        *data = data.wrapping_sub(op.count as u8)
                    }
                    Token::Right => {
                        *ptr += op.count; // On 32 bit systems this overflows after 500MB, on 64
                                          // bit it's 2.3EB, so it should be safe left unchecked
                    }
                    Token::Left => {
                        *ptr = ptr.checked_sub(op.count).expect("Memory underflow");
                    }
                    Token::Output => {
                        output.extend(vec![
                            *get_mut_or_insert(memlayout, ptr.clone(), 0) as char;
                            op.count
                        ]);
                    }
                    Token::Input => {
                        todo!();
                    }
                    Token::JumpZero => {
                        if *get_mut_or_insert(memlayout, ptr.clone(), 0) == 0 {
                            let _ = ast_iter.next();
                        }
                    }
                    Token::JumpNonZero => {
                        if *get_mut_or_insert(memlayout, ptr.clone(), 0) != 0 {
                            ast_iter = ast.iter();
                        }
                    }
                },
            }
        }
        output
    }

    fn get_mut_or_insert<T: std::clone::Clone>(vec: &mut Vec<T>, i: usize, default: T) -> &mut T {
        if let None = vec.get_mut(i) {
            vec.extend(vec![default; i - vec.len() + 1]);
        }

        &mut vec[i]
    }
}
