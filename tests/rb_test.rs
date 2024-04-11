use rust_data_structs::RbTree;
fn create_test_tree(t_size: i32) -> RbTree<i32, i32> {
    let mut t: RbTree<i32, i32> = RbTree::new();
    for i in 0..t_size {
        t.insert(i, i % 2);
    }
    return t;
}

#[cfg(test)]
mod rb_tree {
    use rust_data_structs::RbTree;

    use crate::create_test_tree;

    #[test]
    fn test_insert() {
        let mut t: RbTree<i32, i32> = RbTree::new();

        t.insert(2, 2);
        t.insert(3, 4);
        t.insert(5, 6);
        t.insert(6, 7);
        t.insert(10, 10);
        t.insert(11, 11).insert(12, 12);

        for i in 10..500 {
            t.insert(i, i * 2);
        }
    }

    #[test]
    fn test_delete() {
        let mut t = create_test_tree(500);
        println!("{}", t.len());
        t.delete(132).delete(10).delete(255).delete(1);
        println!("{}", t.len());
    }

    #[test]
    fn test_clear() {
        let mut t = create_test_tree(1000);
        assert_eq!(t.len(), 1000);
        t.clear();
        assert_eq!(t.len(), 0);
    }

    #[test]
    fn test_vec() {
        let mut vec_tree: Vec<RbTree<i32, String>> = Vec::new();

        for _ in 0..3 {
            vec_tree.push(RbTree::new());
        }

        vec_tree[0].insert(10, "hi".to_string());
        vec_tree[1]
            .insert(11, "hi".to_string())
            .insert(13, "world".to_string());
        vec_tree[2].insert(12, "hi".to_string());

        vec_tree[1].print_tree();
    }
}
