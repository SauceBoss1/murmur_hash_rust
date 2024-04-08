use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

use super::{Color, Ptr, RbNode, RbTree};

impl<K: PartialOrd, V> RbNode<K, V> {
    pub fn new(key: K, val: V) -> Option<Ptr<K, V>> {
        Some(Rc::new(RefCell::new(RbNode {
            key,
            val,
            parent: None,
            left_child: None,
            right_child: None,
            color: Color::Red,
        })))
    }
}

impl<K: PartialOrd, V> RbTree<K, V> {
    pub fn new() -> Self {
        RbTree { root: None }
    }
}
