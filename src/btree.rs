/// This structure represents a B-Tree node.
pub struct BTree<T: Ord> {
    // The maximum number of children each node can accomodates.
    order: usize,
    // A kind of a node.
    kind: NodeKind,
    // Keys of a node.
    keys: Vec<T>,
    // Child nodes.
    children: Vec<BTree<T>>
}

/// NodeKind indicates a type of B-Tree node.
enum NodeKind {
    Root,
    Internal,
    Leaf,
}
