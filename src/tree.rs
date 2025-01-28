pub mod tree {
    #[derive(Debug)]
    pub enum Node<T> {
        Internal { children: Branch<T> },
        Leaf { value: T },
    }

    pub type Branch<T> = Vec<Node<T>>;
    pub type Tree<T> = Branch<T>;
}
