/// NodeKind indicates a type of B-Tree node.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum NodeKind {
    Root,
    Internal,
    Leaf,
}

/// This structure represents a B-Tree node.
#[derive(Clone, Debug)]
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

    /// Search a node for a given key.
    pub fn search<'a>(&self, key: &'a T) -> Option<&'a T> {
        // Binary search the keys of the node for a given element.
        // If it is found, return it. otherwise, `idx` will be an index
        // of subtree the element should be.
        let idx = match self.keys.binary_search(&key) {
            Ok(_) => return Some(key),
            Err(idx) => idx,
        };

        // If the node is leaf, stop searching because there's nowhere to search.
        // Or search subtree.
        if self.kind == NodeKind::Leaf {
            None
        } else {
            self.children[idx].search(key)
        }
    }
}
