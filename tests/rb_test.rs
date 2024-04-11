#[cfg(test)]
mod rb_tree {
    // use lazy_static::lazy_static;
    use rust_data_structs::RbTree;

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

        assert_eq!(t.key_exist(2), true);
        assert_eq!(t.key_exist(600), false);

        t.print_tree();
        // println!("{:?}", t);

        t.delete(13).delete(10).delete(12);
        t.delete(5).delete(131);
        t.print_tree()
    }
}
