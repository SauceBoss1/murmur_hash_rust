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
}
