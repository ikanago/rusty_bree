use crate::node::{Node, NodeKind};

/// This structure represents a B-Tree node.
#[derive(Clone, Debug)]
pub struct BTree<T: Ord> {
    root: Box<Node<T>>,
}

impl<T> BTree<T>
where
    T: Ord + Clone,
{
    pub fn new(order: usize) -> Self {
        Self {
            root: Box::new(Node::new(order)),
        }
    }

    /// Search a node for a given key.
    pub fn get<'a>(&self, key: &'a T) -> Option<&'a T> {
        self.root.get(key)
    }

    pub fn insert(&mut self, key: T) {
        self.root.insert(key);
        if self.root.is_overflow() {
            let index = self.root.order / 2;
            let left_child = Node {
                order: self.root.order,
                kind: self.root.kind,
                // Remove `to_vec()` to aviod requiring T to implement `Clone`.
                keys: self.root.keys[..index].to_vec(),
                children: if self.root.kind != NodeKind::Leaf {
                    self.root.children[..index + 1].to_vec()
                } else {
                    vec![]
                },
            };
            let right_child = Node {
                order: self.root.order,
                kind: self.root.kind,
                keys: self.root.keys[index + 1..].to_vec(),
                children: if self.root.kind != NodeKind::Leaf {
                    self.root.children[index + 1..].to_vec()
                } else {
                    vec![]
                },
            };
            let root = Node {
                order: self.root.order,
                kind: NodeKind::Internal,
                keys: vec![self.root.keys[index].clone()],
                children: vec![left_child, right_child],
            };
            self.root = Box::new(root);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::btree::BTree;
    use crate::node::{Node, NodeKind};
    use rand::Rng;

    // Asserts given B-Tree is valid.
    fn is_valid_btree<T: Ord>(node: &Node<T>) -> bool {
        assert!(node.keys.len() < node.order);
        assert!(node.children.len() < node.order + 1);
        match node.kind {
            // A root node must have more than 2 children.
            NodeKind::Root => assert!(node.children.len() >= 2),
            // A internal node must have more than ceil(order / 2).
            NodeKind::Internal => assert!(node.children.len() >= (node.order + 1) / 2),
            // A leaf node must have no child.
            NodeKind::Leaf => assert!(node.children.len() == 0),
        }
        if node.kind != NodeKind::Leaf {
            // If a node except leaf has `k` keys, it must have `k + 1` children.
            assert!(node.keys.len() + 1 == node.children.len());
        }
        // Check if each child node satisfies requirements to be B-Tree.
        assert!(node
            .children
            .iter()
            .all(|tree| { node.order == tree.order && is_valid_btree(&tree) }));
        true
    }

    #[test]
    fn valid_leaf() {
        let tree = Node {
            order: 3,
            kind: NodeKind::Leaf,
            keys: vec![1, 2],
            children: vec![],
        };
        is_valid_btree(&tree);
    }

    #[test]
    #[should_panic]
    fn invalid_leaf() {
        let tree = Node {
            order: 3,
            kind: NodeKind::Leaf,
            keys: vec![1, 2, 3],
            children: vec![],
        };
        is_valid_btree(&tree);
    }

    #[test]
    fn valid_tree() {
        let tree = Node {
            order: 4,
            kind: NodeKind::Root,
            keys: vec![4],
            children: vec![
                Node {
                    order: 4,
                    kind: NodeKind::Internal,
                    keys: vec![2],
                    children: vec![
                        Node {
                            order: 4,
                            kind: NodeKind::Leaf,
                            keys: vec![1],
                            children: vec![],
                        },
                        Node {
                            order: 4,
                            kind: NodeKind::Leaf,
                            keys: vec![3],
                            children: vec![],
                        },
                    ],
                },
                Node {
                    order: 4,
                    kind: NodeKind::Internal,
                    keys: vec![6, 8],
                    children: vec![
                        Node {
                            order: 4,
                            kind: NodeKind::Leaf,
                            keys: vec![5],
                            children: vec![],
                        },
                        Node {
                            order: 4,
                            kind: NodeKind::Leaf,
                            keys: vec![7],
                            children: vec![],
                        },
                        Node {
                            order: 4,
                            kind: NodeKind::Leaf,
                            keys: vec![9, 10],
                            children: vec![],
                        },
                    ],
                },
            ],
        };
        is_valid_btree(&tree);
    }

    #[test]
    fn get_tree() {
        let tree = Node {
            order: 4,
            kind: NodeKind::Root,
            keys: vec![4],
            children: vec![
                Node {
                    order: 4,
                    kind: NodeKind::Internal,
                    keys: vec![2],
                    children: vec![
                        Node {
                            order: 4,
                            kind: NodeKind::Leaf,
                            keys: vec![1],
                            children: vec![],
                        },
                        Node {
                            order: 4,
                            kind: NodeKind::Leaf,
                            keys: vec![3],
                            children: vec![],
                        },
                    ],
                },
                Node {
                    order: 4,
                    kind: NodeKind::Internal,
                    keys: vec![6, 8],
                    children: vec![
                        Node {
                            order: 4,
                            kind: NodeKind::Leaf,
                            keys: vec![5],
                            children: vec![],
                        },
                        Node {
                            order: 4,
                            kind: NodeKind::Leaf,
                            keys: vec![7],
                            children: vec![],
                        },
                        Node {
                            order: 4,
                            kind: NodeKind::Leaf,
                            keys: vec![9, 10],
                            children: vec![],
                        },
                    ],
                },
            ],
        };
        // The tree contains 1, 2, ..., 10.
        for i in 1..=10 {
            assert_eq!(tree.get(&i), Some(&i));
        }
        assert_eq!(tree.get(&11), None);
    }

    #[test]
    fn insert_elements() {
        let mut rng = rand::thread_rng();
        let mut keys = vec![];
        let key_num = 100;
        let key_range = 1000;
        for _ in 0..key_num {
            keys.push(rng.gen_range(0, key_range));
        }

        let mut tree = BTree::new(4);
        for key in &keys {
            tree.insert(key.clone());
        }

        for key in &keys {
            assert_eq!(tree.get(key), Some(key));
        }

        keys.sort();
        keys.dedup();
        assert_eq!(keys, tree.root.traverse());
    }
}
