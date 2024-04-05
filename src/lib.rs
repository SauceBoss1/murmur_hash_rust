mod tree;
#[allow(dead_code)]
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
    use super::Node;

    #[test]
    fn new_node() {
        let node = Node::new(10);
        println!("{}", node.val);
    }
}
