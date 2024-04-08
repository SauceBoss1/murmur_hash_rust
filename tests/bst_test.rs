#[cfg(test)]
mod tree_test {
    use lazy_static::lazy_static;
    use rust_data_structs::Tree;

    lazy_static! {
        static ref GLOBAL_TREE: Tree<i64> = {
            let mut tree = Tree::new();
            let elements = vec![8, 4, 12, 2, 6, 10, 14, 1, 3, 5, 7, 9, 11, 13, 15];
            for &elem in &elements {
                tree.insert(elem);
            }
            return tree;
        };
    }

    #[test]
    fn test_min() {
        assert_eq!(GLOBAL_TREE.min(), Some(&(1 as i64)))
    }

    #[test]
    fn test_max() {
        assert_eq!(GLOBAL_TREE.max(), Some(&(15 as i64)))
    }

    #[test]
    fn test_height() {
        println!("{}", GLOBAL_TREE.height())
    }

    #[test]
    fn test_search() {
        assert_eq!(GLOBAL_TREE.does_exist(10), true);
        assert_eq!(GLOBAL_TREE.does_exist(20), false);
    }
}
