use crate::Tree;

use super::Node;

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
    pub fn new() -> Self {
        Tree { root: None }
    }
}
