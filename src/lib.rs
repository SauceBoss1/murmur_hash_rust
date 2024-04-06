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

#[cfg(test)]
mod test {
    use super::{Node, Tree};

    #[test]
    fn new_node() {
        let node = Node::new(10);
        println!("{:?}", node);
    }

    #[test]
    fn new_tree() {
        let tree: Tree<i32> = Tree::new();
        println!("Tree init: {:#?}", tree);
    }

    #[test]
    fn insert_tree() {
        let mut tree = Tree::new();
        tree.insert(5);
        tree.insert(3);
        tree.insert(7);
        tree.insert(2);
        tree.insert(4);
        tree.insert(6);
        tree.insert(7);
        tree.insert(8);
        println!("{tree}");
        let t2 = tree.clone();
        println!("{t2}");
    }

    #[test]
    fn str_trees() {
        let mut str_tree: Tree<String> = Tree::new();
        str_tree
            .insert("Hello ".to_string())
            .insert("World!".to_string())
            .insert("It is my Birthday!".to_string())
            .insert("I am old now.".to_string());

        println!("{str_tree}");
    }

    #[test]
    fn exist_test() {
        let mut t: Tree<i32> = Tree::new();
        t.insert(5)
            .insert(3)
            .insert(7)
            .insert(8)
            .insert(9)
            .insert(1)
            .insert(3);

        assert_eq!(t.does_exist(1), true);
        assert_eq!(t.does_exist(100), false);
    }

    #[test]
    fn simple_delete() {
        let mut t: Tree<i32> = Tree::new();
        t.insert(5)
            .insert(3)
            .insert(7)
            .insert(8)
            .insert(9)
            .insert(1)
            .insert(3);

        println!("{t}");

        t.delete(9);
        println!("{t}");
    }
}
