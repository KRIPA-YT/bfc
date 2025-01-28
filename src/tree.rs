pub mod tree {
    #[derive(Debug)]
    pub enum Node<T> {
        Internal(Branch<T>),
        Leaf(T),
    }

    impl<T> Node<T> {
        pub fn map_beg<F, R>(&self, func: F) -> Result<R, String>
        where
            F: Fn(&T) -> R,
        {
            match self {
                Self::Internal(children) => children
                    .first()
                    .map_or(Err("Why the hell is Internal empty".to_string()), |e| {
                        e.map_beg(func)
                    }),
                Self::Leaf(value) => Ok(func(value)),
            }
        }
    }

    pub type Branch<T> = Vec<Node<T>>;
    pub type Tree<T> = Branch<T>;
}
