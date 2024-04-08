use std::{cell::RefCell, rc::Rc};

mod tree;
#[derive(Clone, Debug)]
struct Node<T: PartialOrd> {
    val: T,
    left_child: Option<Box<Node<T>>>,
    right_child: Option<Box<Node<T>>>,
}

#[derive(Clone, Debug)]
pub struct Tree<T: PartialOrd> {
    root: Option<Box<Node<T>>>,
}

mod rb_tree;
enum Color {
    Red,
    Black,
}

type Ptr<K: PartialOrd, V> = Rc<RefCell<RbNode<K, V>>>;
struct RbNode<K: PartialOrd, V> {
    val: V,
    key: K,
    color: Color,
    parent: Option<Ptr<K, V>>,
    left_child: Option<Ptr<K, V>>,
    right_child: Option<Ptr<K, V>>,
}

pub struct RbTree<K: PartialOrd, V> {
    root: Option<Ptr<K, V>>,
}
