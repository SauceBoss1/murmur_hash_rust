use std::{
    cell::RefCell,
    fmt::{self, Debug},
    rc::Rc,
};

use super::{Color, Ptr, RbNode, RbTree};

impl<K: PartialOrd, V> RbNode<K, V> {
    pub fn new(key: K, val: V) -> Option<Ptr<K, V>> {
        Some(Rc::new(RefCell::new(RbNode {
            key,
            val,
            parent: None,
            left_child: None,
            right_child: None,
            color: Color::Red,
        })))
    }
}
/// PUBLIC HELPERS
impl<K, V> RbTree<K, V>
where
    K: PartialOrd + Debug,
{
    pub fn new() -> Self {
        RbTree { root: None }
    }

    pub fn insert(&mut self, key: K, value: V) -> &mut Self {
        let new_node = RbNode::new(key, value).expect("Failed to create a new node");

        let mut res_node = None;
        let mut i = self.root.clone();

        while let Some(ref curr_node) = i.clone() {
            res_node = Some(curr_node.clone());
            if new_node.borrow().key < curr_node.borrow().key {
                i = curr_node.borrow().left_child.clone();
            } else {
                i = curr_node.borrow().right_child.clone();
            }
        }

        new_node.borrow_mut().parent = res_node.clone();

        if let Some(parent) = res_node.clone() {
            if new_node.borrow().key < parent.borrow().key {
                parent.borrow_mut().left_child = Some(new_node.clone());
            } else {
                parent.borrow_mut().right_child = Some(new_node.clone());
            }
            new_node.borrow_mut().parent = Some(parent);
        } else {
            self.root = Some(new_node.clone());
        }

        self.in_fix_up(new_node);
        return self;
    }

    pub fn inorder_traversal(&mut self) {}
}

/// PRIVATE HELPERS
impl<K: PartialOrd + Debug, V> RbTree<K, V> {
    // Helper functions below

    /// Rotates tree to the left
    fn left_rotate(&mut self, x: Ptr<K, V>) {
        let y = x.borrow().right_child.as_ref().unwrap().clone();

        let y_temp = y.borrow_mut().left_child.take();
        x.borrow_mut().right_child = y_temp.clone();

        if let Some(ref y_left) = y_temp {
            y_left.borrow_mut().parent = Some(x.clone());
        }

        y.borrow_mut().parent = x.borrow_mut().parent.clone();
        if x.borrow().parent.is_none() {
            self.root = Some(y.clone());
        } else {
            let x_left_is_child = {
                let x_parent = x.borrow().parent.as_ref().unwrap().clone();
                let x_parent_borrow = x_parent.borrow();
                if let Some(ref parent_left) = x_parent_borrow.left_child {
                    Rc::ptr_eq(parent_left, &x)
                } else {
                    false
                }
            };

            let x_parent = x.borrow().parent.as_ref().unwrap().clone();
            if x_left_is_child {
                x_parent.borrow_mut().left_child = Some(y.clone());
            } else {
                x_parent.borrow_mut().right_child = Some(y.clone());
            }
        }
        y.borrow_mut().left_child = Some(x.clone());
        x.borrow_mut().parent = Some(y.clone());
    }

    /// Rotates tree to the right
    fn right_rotate(&mut self, x: Ptr<K, V>) {
        let y = x.borrow().left_child.as_ref().unwrap().clone();

        let y_temp = y.borrow_mut().right_child.take();
        x.borrow_mut().left_child = y_temp.clone();

        if let Some(y_right) = y_temp {
            y_right.borrow_mut().parent = Some(x.clone());
        }

        y.borrow_mut().parent = x.borrow_mut().parent.clone();
        if x.borrow().parent.is_none() {
            self.root = Some(y.clone());
        } else {
            let x_right_is_child = {
                let x_parent = x.borrow().parent.as_ref().unwrap().clone();
                let x_parent_borrow = x_parent.borrow();
                if let Some(ref parent_right) = x_parent_borrow.right_child {
                    Rc::ptr_eq(parent_right, &x)
                } else {
                    false
                }
            };

            let x_parent = x.borrow().parent.as_ref().unwrap().clone();
            if x_right_is_child {
                x_parent.borrow_mut().right_child = Some(y.clone());
            } else {
                x_parent.borrow_mut().left_child = Some(y.clone());
            }
        }

        y.borrow_mut().right_child = Some(x.clone());
        x.borrow_mut().parent = Some(y.clone());
    }

    /// This function fixes up the rb-tree for insertions
    ///
    /// It looks very disgusting but that is what I get
    /// for using safe rust
    fn in_fix_up(&mut self, mut z: Ptr<K, V>) {
        while z
            .borrow()
            .parent
            .as_ref()
            .map_or(false, |p| p.borrow().color == Color::Red)
        {
            let zp = z.borrow().parent.as_ref().unwrap().clone(); // parent
            let zpp = zp.borrow().parent.as_ref().unwrap().clone(); // grandparent

            // if Rc::ptr_eq(&zp, &zpp.borrow().left_child.as_ref().unwrap()) {
            if zpp
                .borrow()
                .left_child
                .as_ref()
                .map_or(false, |left| Rc::ptr_eq(&zp, left))
            {
                //z.parent is the left child
                let y = zpp.borrow().right_child.clone();

                if let Some(y) = y.as_ref().filter(|y| y.borrow().color == Color::Red) {
                    // case 1
                    zp.borrow_mut().color = Color::Black;
                    y.borrow_mut().color = Color::Black;
                    zpp.borrow_mut().color = Color::Red;
                    z = zpp.clone();
                } else {
                    // case 2
                    if zp
                        .borrow()
                        .right_child
                        .as_ref()
                        .map_or(false, |right| Rc::ptr_eq(&z, right))
                    {
                        z = zp.clone();
                        self.left_rotate(z.clone());
                    }

                    // case 3
                    zp.borrow_mut().color = Color::Black;
                    zpp.borrow_mut().color = Color::Red;
                    self.right_rotate(zpp.clone());
                }
            } else {
                let y = zpp.borrow().left_child.clone();

                if let Some(y) = y.as_ref().filter(|y| y.borrow().color == Color::Red) {
                    // case 1
                    zp.borrow_mut().color = Color::Black;
                    y.borrow_mut().color = Color::Black;
                    zpp.borrow_mut().color = Color::Red;
                    z = zpp.clone();
                } else {
                    // case 2
                    if zp
                        .borrow()
                        .left_child
                        .as_ref()
                        .map_or(false, |left| Rc::ptr_eq(&z, left))
                    {
                        z = zp.clone();
                        self.right_rotate(z.clone());
                    }

                    //case 3
                    zp.borrow_mut().color = Color::Black;
                    zpp.borrow_mut().color = Color::Red;
                    self.left_rotate(zpp);
                }
            }
        }

        if let Some(ref root) = self.root {
            root.borrow_mut().color = Color::Black;
        }
    }
}

impl<K, V> RbTree<K, V>
where
    K: PartialOrd + std::fmt::Debug,
    V: std::fmt::Debug,
{
    pub fn print_in_order(&self) {
        fn in_order<K, V>(node: &Option<Rc<RefCell<RbNode<K, V>>>>)
        where
            K: PartialOrd + std::fmt::Debug,
            V: std::fmt::Debug,
        {
            if let Some(ref n) = node {
                in_order(&n.borrow().left_child);
                println!("Key: {:?}, Value: {:?}", n.borrow().key, n.borrow().val);
                in_order(&n.borrow().right_child);
            }
        }

        println!("In-order Traversal:");
        in_order(&self.root);
    }
}

impl<K: Debug + PartialOrd, V: Debug> fmt::Debug for RbNode<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("RbNode")
            .field("key", &self.key)
            .field("val", &self.val)
            .field("color", &self.color)
            // Omit the parent to prevent cyclic printing
            .field("left_child", &self.left_child)
            .field("right_child", &self.right_child)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_left_rotate() {
        // Step 2: Manually construct the tree goes here.
        let mut tree = RbTree::<i32, i32>::new(); // Create a new tree instance.

        let x = Rc::new(RefCell::new(RbNode {
            key: 1,
            val: 10,
            color: Color::Black, // Assume colors are correctly set for your needs.
            parent: None,
            left_child: None,
            right_child: None,
        }));

        let y = Rc::new(RefCell::new(RbNode {
            key: 2,
            val: 20,
            color: Color::Red,
            parent: None,
            left_child: None,
            right_child: None,
        }));

        // Manually link x and y.
        x.borrow_mut().right_child = Some(y.clone());
        y.borrow_mut().parent = Some(x.clone());

        // Set x as the tree's root.
        tree.root = Some(x.clone());

        tree.left_rotate(x.clone());

        assert_eq!(tree.root.as_ref().unwrap().borrow().key, 2); // y is now the root.
        assert!(tree.root.as_ref().unwrap().borrow().left_child.is_some()); // y has a left child.
        assert_eq!(
            tree.root
                .unwrap()
                .borrow()
                .left_child
                .as_ref()
                .unwrap()
                .borrow()
                .key,
            1
        ); // The left child of y is x.
    }

    #[test]
    fn test_right_rotate() {
        let mut tree = RbTree::<i32, i32>::new();

        let x = Rc::new(RefCell::new(RbNode {
            key: 2,
            val: 20,
            color: Color::Black,
            parent: None,
            left_child: None,
            right_child: None,
        }));

        let y = Rc::new(RefCell::new(RbNode {
            key: 1,
            val: 10,
            color: Color::Red,
            parent: None,
            left_child: None,
            right_child: None,
        }));

        // Manually link x and y.
        x.borrow_mut().left_child = Some(y.clone());
        y.borrow_mut().parent = Some(x.clone());

        // Set x as the tree's root.
        tree.root = Some(x.clone());

        tree.right_rotate(x.clone());

        assert_eq!(tree.root.as_ref().unwrap().borrow().key, 1); // y is now the root.
        assert!(tree.root.as_ref().unwrap().borrow().right_child.is_some()); // y has a right child.
        assert_eq!(
            tree.root
                .unwrap()
                .borrow()
                .right_child
                .as_ref()
                .unwrap()
                .borrow()
                .key,
            2
        ); // The right child of y is x.
    }
}
