use std::{cell::RefCell, rc::Rc};

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
    K: PartialOrd,
{
    pub fn new() -> Self {
        RbTree { root: None }
    }

    pub fn insert(&mut self, key: K, value: V) {
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
                parent.borrow_mut().left_child = Some(new_node);
            } else {
                parent.borrow_mut().right_child = Some(new_node);
            }
        }

        todo!("Add fix-up function here")
    }
}

/// PRIVATE HELPERS
impl<K: PartialOrd, V> RbTree<K, V> {
    // Helper functions below

    /// Rotates tree to the left
    fn left_rotates(&mut self, x: Ptr<K, V>) {
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

        tree.left_rotates(x.clone());

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
