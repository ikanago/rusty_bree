mod tests {
    use crate::btree::{BTree, NodeKind};

    // Asserts given B-Tree is valid.
    fn is_valid_btree<T: Ord>(btree: &BTree<T>) -> bool {
        assert!(btree.keys.len() < btree.order);
        assert!(btree.children.len() < btree.order + 1);
        match btree.kind {
            // A root node must have more than 2 children.
            NodeKind::Root => assert!(btree.children.len() >= 2),
            // A internal node must have more than ceil(order / 2).
            NodeKind::Internal => assert!(btree.children.len() >= (btree.order + 1) / 2),
            // A leaf node must have no child.
            NodeKind::Leaf => assert!(btree.children.len() == 0),
        }
        if btree.kind != NodeKind::Leaf {
            // If a node except leaf has `k` keys, it must have `k + 1` children.
            assert!(btree.keys.len() + 1 == btree.children.len());
        }
        // Check if each child node satisfies requirements to be B-Tree.
        assert!(btree
            .children
            .iter()
            .all(|tree| { btree.order == tree.order && is_valid_btree(&tree) }));
        true
    }

    #[test]
    fn valid_leaf() {
        let tree = BTree {
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
        let tree = BTree {
            order: 3,
            kind: NodeKind::Leaf,
            keys: vec![1, 2, 3],
            children: vec![],
        };
        is_valid_btree(&tree);
    }

    #[test]
    fn valid_tree() {
        let tree = BTree {
            order: 4,
            kind: NodeKind::Root,
            keys: vec![4],
            children: vec![
                BTree {
                    order: 4,
                    kind: NodeKind::Internal,
                    keys: vec![2],
                    children: vec![BTree::new(4, 1), BTree::new(4, 3)],
                },
                BTree {
                    order: 4,
                    kind: NodeKind::Internal,
                    keys: vec![6, 8],
                    children: vec![
                        BTree::new(4, 5),
                        BTree::new(4, 7),
                        BTree {
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
}
