/// This structure represents a B-Tree node.
pub struct BTree<T: Ord> {
    // The maximum number of children each node can accomodates.
    pub(crate) order: usize,
    // A kind of a node.
    pub(crate) kind: NodeKind,
    // Keys of a node.
    pub(crate) keys: Vec<T>,
    // Child nodes.
    pub(crate) children: Vec<BTree<T>>,
}

impl<T> BTree<T>
where
    T: Ord,
{
    pub fn new(order: usize, key: T) -> Self {
        Self {
            order,
            kind: NodeKind::Leaf,
            keys: vec![key],
            children: vec![],
        }
    }
}

/// NodeKind indicates a type of B-Tree node.
#[derive(Debug, Eq, PartialEq)]
pub(crate) enum NodeKind {
    Root,
    Internal,
    Leaf,
}
