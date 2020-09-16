/// NodeKind indicates a type of B-Tree node.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum NodeKind {
    Root,
    Internal,
    Leaf,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Node<T: Ord> {
    // The maximum number of children each node can accomodates.
    pub(crate) order: usize,
    // A kind of a node.
    pub(crate) kind: NodeKind,
    // Keys of a node.
    pub(crate) keys: Vec<T>,
    // Child nodes.
    pub(crate) children: Vec<Node<T>>,
}

impl<T> Node<T>
where
    T: Ord + Clone,
{
    pub fn new(order: usize) -> Self {
        Self {
            order,
            kind: NodeKind::Root,
            keys: vec![],
            children: vec![],
        }
    }

    pub(crate) fn traverse(&self) -> Vec<T> {
        let mut extracted = vec![];
        if self.kind == NodeKind::Leaf {
            extracted = self.keys.clone();
        } else {
            extracted.append(&mut self.children[0].traverse());
            for i in 0..self.keys.len() {
                extracted.push(self.keys[i].clone());
                extracted.append(&mut self.children[i + 1].traverse());
            }
        }
        extracted
    }

    /// Search a node for a given key.
    pub(crate) fn get<'a>(&self, key: &'a T) -> Option<&'a T> {
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
            self.children[idx].get(key)
        }
    }

    pub(crate) fn is_overflow(&self) -> bool {
        self.keys.len() == self.order
    }

    pub(crate) fn insert(&mut self, key: T) {
        let index = match self.keys.binary_search(&key) {
            Ok(_) => return,
            Err(index) => index,
        };
        if self.children.len() == 0 {
            self.keys.insert(index, key);
            return;
        }
        self.children[index].insert(key);
        if self.children[index].is_overflow() {
            self.split_children(index);
        }
    }

    fn split_children(&mut self, index: usize) {
        let split_at = self.children[index].order / 2;
        let right_child = Node {
            order: self.children[index].order,
            kind: self.children[index].kind,
            keys: self.children[index].keys.split_off(split_at + 1),
            children: if self.children[index].kind != NodeKind::Leaf {
                self.children[index].children.split_off(split_at + 1)
            } else {
                vec![]
            },
        };
        self.children.insert(index + 1, right_child);
        let ascending_key = self.children[index].keys.pop().unwrap();
        self.keys.insert(index, ascending_key);
    }
}

#[cfg(test)]
mod tests {
    use crate::node::{Node, NodeKind};

    #[test]
    fn test_split_children() {
        let mut tree = Node {
            order: 3,
            kind: NodeKind::Internal,
            keys: vec![2, 6],
            children: vec![
                Node {
                    order: 3,
                    kind: NodeKind::Leaf,
                    keys: vec![1],
                    children: vec![],
                },
                Node {
                    order: 3,
                    kind: NodeKind::Leaf,
                    keys: vec![3, 4, 5],
                    children: vec![],
                },
                Node {
                    order: 3,
                    kind: NodeKind::Leaf,
                    keys: vec![7],
                    children: vec![],
                },
            ],
        };
        tree.split_children(1);
        assert_eq!(
            Node {
                order: 3,
                kind: NodeKind::Internal,
                keys: vec![2, 4, 6],
                children: vec![
                    Node {
                        order: 3,
                        kind: NodeKind::Leaf,
                        keys: vec![1],
                        children: vec![],
                    },
                    Node {
                        order: 3,
                        kind: NodeKind::Leaf,
                        keys: vec![3],
                        children: vec![],
                    },
                    Node {
                        order: 3,
                        kind: NodeKind::Leaf,
                        keys: vec![5],
                        children: vec![],
                    },
                    Node {
                        order: 3,
                        kind: NodeKind::Leaf,
                        keys: vec![7],
                        children: vec![],
                    },
                ],
            },
            tree,
        );
    }
}
