use std::{
    cell::RefCell,
    fmt::Debug,
    rc::Rc,
    vec::{self},
};

// Below is a standard binary search tree
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

#[derive(PartialEq, PartialOrd, Clone, Debug, Copy)]
enum Color {
    Red,
    Black,
}

type Ptr<K, V> = Rc<RefCell<RbNode<K, V>>>;

#[derive(Clone, PartialEq)]
struct RbNode<K: PartialOrd, V: Debug> {
    val: V,
    key: K,
    color: Color,
    parent: Option<Ptr<K, V>>,
    left_child: Option<Ptr<K, V>>,
    right_child: Option<Ptr<K, V>>,
}

pub struct RbIter<K, V>
where
    K: Debug + Clone,
    V: Debug + Clone,
{
    iter: vec::IntoIter<(K, V)>,
}

/// # Red Black Tree
///
/// types: K, V
///
/// public functions:
/// - insert
/// - delete
/// - clear
/// - new
/// - key_exists
/// - get
/// - len
#[derive(Debug, Clone)]
pub struct RbTree<K: PartialOrd, V: Debug> {
    root: Option<Ptr<K, V>>,
    length: i32,
}

// hash table
mod murmur;
pub use murmur::{hash_anything, hash_mumur3};
use serde::Serialize;

mod hash_dict;

pub struct HashDictIter<K, V>
where
    K: Debug + Clone,
    V: Debug + Clone,
{
    iter: vec::IntoIter<(K, V)>,
}

#[derive(Debug, Clone)]
pub struct HashDict<K, V>
where
    K: PartialOrd + Serialize,
    V: Debug,
{
    arr_length: usize,
    seed: u32,
    tab_length: usize,
    table: Vec<RbTree<K, V>>,
}
