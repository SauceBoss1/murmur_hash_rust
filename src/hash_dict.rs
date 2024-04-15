use std::fmt::Debug;

use serde::Serialize;

use crate::HashDictIter;

use super::{hash_anything, HashDict, RbTree};

impl<K, V> HashDict<K, V>
where
    K: PartialOrd + Serialize + Debug + Clone,
    V: PartialOrd + Clone + Debug,
{
    pub fn new(len: usize, seed: u32) -> Self {
        let mut new_table: Vec<RbTree<K, V>> = Vec::new();
        for _ in 0..len {
            new_table.push(RbTree::new());
        }
        HashDict {
            arr_length: len,
            seed,
            tab_length: 0,
            table: new_table,
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> &mut Self {
        let seed = self.seed;
        if let Ok(index) = hash_anything(&key, seed) {
            let resize_index: usize = (index % (self.arr_length as u128)) as usize;

            self.table[resize_index].insert(key, value);
            self.tab_length += 1;
        }

        return self;
    }

    pub fn get(&self, key: &K) -> Option<V> {
        if let Ok(index) = hash_anything(key, self.seed) {
            let resize_index: usize = (index % (self.arr_length as u128)) as usize;

            let res = self.table[resize_index].get(&key);
            return res;
        }
        None
    }

    pub fn delete(&mut self, key: &K) -> &mut Self {
        if let Ok(hash) = hash_anything(&key, self.seed) {
            let index: usize = (hash % (self.arr_length as u128)) as usize;

            if self.table[index].key_exist(key.clone()) {
                self.table[index].delete(key.clone());
                self.tab_length -= 1;
            }
        }
        return self;
    }

    pub fn pop(&mut self, key: &K) -> Option<V> {
        if let Some(val) = self.get(key) {
            self.delete(key);
            return Some(val);
        }
        None
    }

    pub fn len(&self) -> usize {
        self.tab_length
    }

    pub fn get_mut<F>(&mut self, key: &K, f: F)
    where
        F: FnMut(&mut V),
    {
        if let Ok(hash) = hash_anything(key, self.seed) {
            let index = (hash % (self.arr_length as u128)) as usize;
            self.table[index].get_mut(key, f);
        }
    }
}

// developing iters
impl<K, V> HashDict<K, V>
where
    K: PartialOrd + PartialEq + Debug + Clone + Serialize,
    V: PartialEq + PartialOrd + Debug + Clone,
{
    fn stack_items(&self) -> Vec<(K, V)> {
        let mut stack: Vec<(K, V)> = Vec::new();
        for tree in self.table.iter() {
            if tree.len() > 0 {
                stack.extend(tree.iter());
            }
        }
        return stack;
    }

    pub fn iter(&self) -> HashDictIter<K, V> {
        let mut stack = self.stack_items();
        stack.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        return HashDictIter {
            iter: stack.into_iter(),
        };
    }
}

impl<K, V> Iterator for HashDictIter<K, V>
where
    K: Debug + Clone,
    V: Debug + Clone,
{
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        return self.iter.next();
    }
}

// macro
#[macro_export]
macro_rules! hash_dict {
    // This branch initializes an empty HashDict and returns it.
    ($length:expr, $seed:expr) => {{
        HashDict::new($length, $seed)
    }};

    // This branch initializes a HashDict and inserts given key-value pairs, then returns it.
    ($length:expr, $seed:expr, $( $key:expr => $value:expr ),* $(,)?) => {{
        let mut table = HashDict::new($length, $seed);
        $(
            table.insert($key, $value);
        )*
        table
    }};
}
