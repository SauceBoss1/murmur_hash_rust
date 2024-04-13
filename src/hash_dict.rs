use std::fmt::Debug;

use serde::Serialize;

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
            table: new_table,
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> &mut Self {
        let seed = self.seed;
        if let Ok(index) = hash_anything(&key, seed) {
            let resize_index: usize = (index % (self.arr_length as u128)) as usize;

            self.table[resize_index].insert(key, value);
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

            self.table[index].delete(key.clone());
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
}

// macro
#[macro_export]
macro_rules! hash_dict {
    // This branch initializes an empty HashDict and returns it.
    () => {{
        HashDict::new(10, 42)
    }};

    // This branch initializes a HashDict and inserts given key-value pairs, then returns it.
    ($( $key:expr => $value:expr ),* $(,)?) => {{
        let mut table = HashDict::new(10, 42);
        $(
            table.insert($key, $value);
        )*
        table
    }};
}
