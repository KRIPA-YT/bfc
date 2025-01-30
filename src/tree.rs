pub mod tree {
    #[derive(Debug)]
    pub enum Node<T> {
        Internal(Branch<T>),
        Leaf(T),
    }

    pub type Branch<T> = Vec<Node<T>>;
    pub type Tree<T> = Branch<T>;
}
