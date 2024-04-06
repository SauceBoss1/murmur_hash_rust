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

    pub fn new_mod(
        val_in: T,
        left_child: Option<Box<Node<T>>>,
        right_child: Option<Box<Node<T>>>,
    ) -> Box<Node<T>> {
        Box::new(Node {
            val: val_in,
            left_child: left_child,
            right_child: right_child,
        })
    }
}

impl<T: PartialOrd + Clone> Tree<T> {
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

    // public search function
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

    // finding min helper
    fn find_min(node: Box<Node<T>>) -> Box<Node<T>> {
        let mut current_node = node;
        while let Some(n) = current_node.left_child {
            current_node = n;
        }
        return current_node;
    }

    // delete
    pub fn delete(&mut self, val: T) -> &mut Self {
        self.root = Self::delete_recursive(self.root.take(), val);
        self
    }
    fn delete_recursive(node: Option<Box<Node<T>>>, val: T) -> Option<Box<Node<T>>> {
        match node {
            Some(n) if val < n.val => {
                let left_child_update = Self::delete_recursive(n.left_child, val);
                Some(Node::new_mod(n.val, left_child_update, n.right_child))
            }
            Some(n) if val > n.val => {
                let right_child_update = Self::delete_recursive(n.right_child, val);
                Some(Node::new_mod(n.val, n.left_child, right_child_update))
            }
            Some(n) => match (n.left_child, n.right_child) {
                (None, None) => None,
                (Some(l), None) => Some(l),
                (None, Some(r)) => Some(r),
                (Some(l), Some(r)) => {
                    let successor_val = Self::find_min(r.clone()).val.clone();
                    let r_update = Self::delete_recursive(Some(r), successor_val.clone());
                    Some(Node::new_mod(successor_val, Some(l), r_update))
                }
            },
            None => None,
        }
    }

    // deletes tree
    pub fn clear(&mut self) {
        self.root = None;
    }
}

impl<T: PartialOrd + std::fmt::Debug> Tree<T> {
    // prints the tree
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
