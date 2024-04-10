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
        // t.insert(11, 11).insert(12, 12);

        for i in 10..100 {
            t.insert(i, i * 2);
        }
        t.print_in_order();
        // println!("{:?}", t);
    }
}
