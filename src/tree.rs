use crate::{Node, Tree};
use std::fmt::{self, Display, Formatter};

// Node implementation
impl<T: PartialOrd> Node<T> {
    pub fn new(val_in: T) -> Box<Node<T>> {
        Box::new(Node {
            val: val_in,
            left_child: None,
            right_child: None,
        })
    }
}

impl<T: PartialOrd> Tree<T> {
    // creating a new Tree
    pub fn new() -> Self {
        Tree { root: None }
    }

    // Inserting Nodes into Tree
    pub fn insert(&mut self, val: T) -> &mut Self {
        if self.root.is_none() {
            self.root = Some(Node::new(val));
            return self;
        }

        let mut curr_node = &mut self.root;
        while let Some(ref mut node) = curr_node {
            if val < node.val {
                if node.left_child.is_none() {
                    node.left_child = Some(Node::new(val));
                    return self;
                } else {
                    curr_node = &mut node.left_child;
                }
            } else {
                if node.right_child.is_none() {
                    node.right_child = Some(Node::new(val));
                    return self;
                } else {
                    curr_node = &mut node.right_child;
                }
            }
        }

        return self;
    }

    pub fn does_exist(&self, val: T) -> bool {
        self.search(val).is_some()
    }

    fn search(&self, val: T) -> Option<&Node<T>> {
        return self.search_recurse(&self.root, val);
    }

    fn search_recurse<'a>(&self, node: &'a Option<Box<Node<T>>>, val: T) -> Option<&'a Node<T>> {
        match node {
            Some(ref n) if val == n.val => Some(n),
            Some(ref n) if val < n.val => self.search_recurse(&n.left_child, val),
            Some(ref n) => self.search_recurse(&n.right_child, val),
            _ => None,
        }
    }
}

impl<T: PartialOrd + std::fmt::Debug> Tree<T> {
    fn build_inorder_str(&self, node: &Option<Box<Node<T>>>, buffer: &mut String) {
        if let Some(ref n) = node {
            self.build_inorder_str(&n.left_child, buffer);

            if !buffer.is_empty() {
                buffer.push_str(", ");
            }
            buffer.push_str(&format!("{:?}", n.val));

            self.build_inorder_str(&n.right_child, buffer);
        }
    }
}

impl<T: PartialOrd + std::fmt::Debug> Display for Tree<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut out_str = String::new();
        self.build_inorder_str(&self.root, &mut out_str);
        return write!(f, "{}", out_str);
    }
}
