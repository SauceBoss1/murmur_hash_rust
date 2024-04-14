// use std::borrow::Borrow;
use std::{
    cell::RefCell,
    fmt::{self, Debug},
    rc::Rc,
};

use PartialEq;

use crate::RbIter;

use super::{Color, Ptr, RbNode, RbTree};

impl<K: PartialOrd, V: PartialEq + Debug> RbNode<K, V> {
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
    V: PartialEq + PartialOrd + Debug,
{
    pub fn new() -> Self {
        RbTree {
            root: None,
            length: 0,
        }
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

        self.length += 1;
        return self;
    }

    pub fn delete(&mut self, key: K) -> &mut Self {
        if let Some(ref z) = self.search(&key) {
            let mut y = z.clone();
            let x;
            let mut y_og_color: Color = Self::node_color(&Some(y.clone()));

            if z.borrow().left_child.is_none() {
                x = z.borrow().right_child.clone();
                self.rb_transplant(z.clone(), x.clone());
            } else if z.borrow().right_child.is_none() {
                x = z.borrow().left_child.clone();
                self.rb_transplant(z.clone(), x.clone());
            } else {
                y = self.find_min(z.borrow().right_child.clone().expect("right must exist")); // this finds right most (which is a leaf)
                y_og_color = Self::node_color(&Some(y.clone()));
                x = y.borrow().right_child.clone(); // this breaks if right child is none

                if y.borrow()
                    .parent
                    .as_ref()
                    .map_or(false, |p| Rc::ptr_eq(p, z))
                {
                    if let Some(ref x) = x.clone() {
                        x.borrow_mut().parent = Some(z.clone());
                    }
                } else {
                    self.rb_transplant(y.clone(), x.clone());
                    y.borrow_mut().right_child = z.borrow().right_child.clone();
                    if let Some(ref right) = y.borrow().right_child {
                        right.borrow_mut().parent = Some(y.clone());
                    }
                }

                self.rb_transplant(z.clone(), Some(y.clone()));
                y.borrow_mut().left_child = z.borrow().left_child.clone();
                if let Some(ref left) = y.borrow().left_child {
                    left.borrow_mut().parent = Some(y.clone());
                }
                y.borrow_mut().color = z.borrow().color;
            }

            if y_og_color == Color::Black {
                // println!(
                //     "Before delete_fixup:\n x = {:?},\n\nroot = {:?},\n\n x color = {:?}",
                //     x,
                //     self.root,
                //     Self::node_color(&x)
                // );
                self.delete_fixup(x.clone());
            }

            self.length -= 1;
        }

        return self;
    }

    pub fn key_exist(&self, key: K) -> bool {
        self.search(&key).is_some()
    }

    pub fn clear(&mut self) -> &mut Self {
        self.root = None;
        self.length = 0;
        self
    }

    pub fn get(&self, key: &K) -> Option<V>
    where
        V: Clone,
    {
        match self.search(key) {
            Some(n) => Some(n.borrow().val.clone()),
            None => None,
        }
    }

    pub fn len(&self) -> i32 {
        self.length
    }
}

/// PRIVATE HELPERS
impl<K: PartialOrd + Debug, V: Debug> RbTree<K, V> {
    // Helper functions below

    //search
    fn search(&self, k: &K) -> Option<Ptr<K, V>> {
        let mut curr_node = self.root.clone();
        while let Some(ref n) = curr_node.clone() {
            if k < &n.borrow().key {
                curr_node = n.borrow().left_child.clone();
            } else if k > &n.borrow().key {
                curr_node = n.borrow().right_child.clone();
            } else {
                return Some(n.clone());
            }
        }
        return None;
    }

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

    // Let's start implementing delete helpers
    fn rb_transplant(&mut self, u: Ptr<K, V>, v: Option<Ptr<K, V>>) {
        let u_parent = u.borrow().parent.clone();

        match u_parent {
            Some(ref parent) => {
                if parent
                    .borrow()
                    .left_child
                    .as_ref()
                    .map_or(false, |left| Rc::ptr_eq(&u, &left))
                {
                    parent.borrow_mut().left_child = v.clone()
                } else {
                    parent.borrow_mut().right_child = v.clone();
                }
            }
            None => self.root = v.clone(),
        }

        // v.borrow_mut().parent = u.borrow().parent.clone();
        if let Some(ref v_inner) = v {
            v_inner.borrow_mut().parent = u.borrow().parent.clone();
        }
    }

    fn find_min(&mut self, x: Ptr<K, V>) -> Ptr<K, V> {
        let mut current = x;
        while let Some(ref left) = current.clone().borrow().left_child {
            current = left.clone();
        }

        return current;
    }
}

impl<K, V> RbTree<K, V>
where
    K: PartialOrd + Debug,
    V: PartialEq + Debug,
{
    /// get node color
    fn node_color(node: &Option<Ptr<K, V>>) -> Color {
        node.as_ref().map_or(Color::Black, |n| n.borrow().color)
    }

    /// check if left child
    fn is_left_child(node: &Ptr<K, V>) -> bool {
        match node.borrow().parent {
            Some(ref parent) => parent
                .borrow()
                .left_child
                .as_ref()
                .map_or(false, |l_child| Rc::ptr_eq(l_child, node)),
            None => false,
        }
    }

    /// retrieve sibling
    fn sibling(node: &Ptr<K, V>) -> Option<Ptr<K, V>> {
        node.borrow().parent.as_ref().and_then(|p| {
            if Self::is_left_child(node) {
                p.borrow().right_child.clone()
            } else {
                p.borrow().left_child.clone()
            }
        })
    }

    #[allow(dead_code)]
    fn replace_node(node: &Ptr<K, V>, replacement: Option<Ptr<K, V>>) {
        if let Some(ref parent) = node.borrow().parent {
            if Self::is_left_child(node) {
                parent.borrow_mut().left_child = replacement.clone();
            } else {
                parent.borrow_mut().right_child = replacement.clone();
            }
        }
        if let Some(ref replacement) = replacement {
            replacement.borrow_mut().parent = node.borrow().parent.clone();
        }
    }

    /// set's the color of a node
    fn set_color(node: &Option<Ptr<K, V>>, color: Color) {
        if let Some(ref n) = node {
            n.borrow_mut().color = color;
        }
    }

    fn delete_fixup(&mut self, mut x: Option<Ptr<K, V>>) {
        // check if pointers have the same location
        while !x.as_ref().map_or(false, |x_ref| {
            Rc::ptr_eq(x_ref, &self.root.clone().unwrap())
        }) && Self::node_color(&x) == Color::Black
        {
            if let Some(ref x_unwrapped) = x {
                if Self::is_left_child(x_unwrapped) {
                    let mut w = Self::sibling(x_unwrapped).expect("Sibling must exist");
                    if Self::node_color(&Some(w.clone())) == Color::Red {
                        // case 1
                        Self::set_color(&Some(w.clone()), Color::Black);
                        Self::set_color(&x_unwrapped.borrow().parent, Color::Red);
                        self.left_rotate(
                            x_unwrapped
                                .borrow()
                                .parent
                                .clone()
                                .expect("parent must exist"),
                        );
                        let new_w = Self::sibling(x_unwrapped).expect("sibling must exist");
                        w = new_w;
                    }

                    // case 2
                    if Self::node_color(&w.borrow().left_child) == Color::Black
                        && Self::node_color(&w.borrow().right_child) == Color::Black
                    {
                        Self::set_color(&Some(w.clone()), Color::Red);
                        let x_new = x_unwrapped.borrow().parent.clone();
                        x = x_new;
                    } else {
                        if Self::node_color(&w.borrow().right_child) == Color::Black {
                            Self::set_color(&w.borrow().left_child, Color::Black);
                            Self::set_color(&Some(w.clone()), Color::Red);
                            self.right_rotate(w.clone());
                            let new_w = Self::sibling(x_unwrapped).expect("Sibling must exist");
                            w = new_w;
                        }

                        Self::set_color(
                            &Some(w.clone()),
                            Self::node_color(&x_unwrapped.borrow().parent),
                        );
                        Self::set_color(&x_unwrapped.borrow().parent, Color::Black);
                        Self::set_color(&w.borrow().right_child, Color::Black);
                        self.left_rotate(
                            x_unwrapped
                                .borrow()
                                .parent
                                .clone()
                                .expect("parent must exist"),
                        );

                        let x_new = self.root.clone();
                        x = x_new;
                    }
                } else {
                    // Mirror image of the above code with left and right swapped.
                    let mut w = Self::sibling(x_unwrapped).expect("Sibling must exist");
                    if Self::node_color(&Some(w.clone())) == Color::Red {
                        Self::set_color(&Some(w.clone()), Color::Black);
                        Self::set_color(&x_unwrapped.borrow().parent, Color::Red);
                        self.right_rotate(x_unwrapped.borrow().parent.clone().unwrap());
                        let new_w = Self::sibling(x_unwrapped)
                            .expect("New sibling must exist after rotation");
                        w = new_w;
                    }

                    if Self::node_color(&w.borrow().right_child) == Color::Black
                        && Self::node_color(&w.borrow().left_child) == Color::Black
                    {
                        Self::set_color(&Some(w.clone()), Color::Red);
                        let new_x = x_unwrapped.borrow().parent.clone();
                        x = new_x;
                    } else {
                        if Self::node_color(&w.borrow().left_child) == Color::Black {
                            Self::set_color(&w.borrow().right_child, Color::Black);
                            Self::set_color(&Some(w.clone()), Color::Red);
                            self.left_rotate(w.clone());
                            let new_w = Self::sibling(x_unwrapped)
                                .expect("New sibling must exist after rotation");
                            w = new_w;
                        }

                        Self::set_color(
                            &Some(w.clone()),
                            Self::node_color(&x_unwrapped.borrow().parent),
                        );
                        Self::set_color(&x_unwrapped.borrow().parent, Color::Black);
                        Self::set_color(&w.borrow().left_child, Color::Black);
                        self.right_rotate(x_unwrapped.borrow().parent.clone().unwrap());
                        x = self.root.clone();
                    }
                }
            } else {
                break;
            }
        }
        if let Some(ref x) = x {
            Self::set_color(&Some(x.clone()), Color::Black);
        }
    }
}

// DEBUGGING STUFF BELOW
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
            .field("color", &self.color)
            // Omit the parent to prevent cyclic printing
            .field("left_child", &self.left_child)
            .field("right_child", &self.right_child)
            .finish()
    }
}

impl<K: std::fmt::Debug + PartialOrd, V: std::fmt::Debug> RbTree<K, V> {
    pub fn print_tree(&self) {
        self.print_node(&self.root, 0);
    }

    fn print_node(&self, node: &Option<Ptr<K, V>>, depth: usize) {
        if let Some(n) = node {
            let indent = "->".repeat(depth); // Create indentation based on the depth
            println!("{}|{:?}|", indent, n.borrow().key); // Print the current node

            // Recursively print the left child, if it exists
            if n.borrow().left_child.is_some() {
                // println!("{}left:", indent); // Label for left child
                self.print_node(&n.borrow().left_child, depth + 1); // Recursion with increased depth
            }

            // Recursively print the right child, if it exists
            if n.borrow().right_child.is_some() {
                // println!("{}right:", indent); // Label for right child
                self.print_node(&n.borrow().right_child, depth + 1); // Recursion with increased depth
            }
        }
    }
}

impl<K, V> RbTree<K, V>
where
    K: PartialOrd + Debug,
    V: Debug,
{
    pub fn print_ascii_tree(&self) {
        Self::print_ascii(&self.root, 0, 0, false);
    }
    fn print_ascii(node: &Option<Ptr<K, V>>, space: usize, depth: usize, is_left: bool) {
        if let Some(node) = node.clone() {
            let borrowed_node = node.borrow();
            let color = match borrowed_node.color {
                Color::Black => "B",
                Color::Red => "R",
            };
            let offset = 5;

            Self::print_ascii(&borrowed_node.left_child, space + offset, depth + 1, true);

            let indent = " ".repeat(space);
            if depth != 0 {
                // not root node
                println!(
                    "{}{}{:?}({})",
                    indent,
                    if is_left { "┌──" } else { "└──" },
                    borrowed_node.key,
                    color
                );
            } else {
                println!("{}{:?}({})", indent, borrowed_node.key, color);
            }

            Self::print_ascii(&borrowed_node.right_child, space + offset, depth + 1, false);
        }
    }
}

// developing iterators

impl<K, V> Iterator for RbIter<K, V>
where
    K: Debug + Clone,
    V: Debug + Clone,
{
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        return self.iter.next();
    }
}

impl<K, V> RbTree<K, V>
where
    K: PartialEq + PartialOrd + Debug + Clone,
    V: PartialEq + Debug + Clone,
{
    fn inorder_stack_recurse(&self, node: &Option<Ptr<K, V>>, vec_in: &mut Vec<(K, V)>) {
        if let Some(ref n) = node.clone() {
            self.inorder_stack_recurse(&n.borrow().left_child, vec_in);

            vec_in.push((n.borrow().key.clone(), n.borrow().val.clone()));

            self.inorder_stack_recurse(&n.borrow().right_child, vec_in);
        }
    }

    fn inorder_items(&self) -> Vec<(K, V)> {
        let mut stack: Vec<(K, V)> = Vec::new();
        self.inorder_stack_recurse(&self.root, &mut stack);
        return stack;
    }

    pub fn iter(&self) -> RbIter<K, V> {
        let stack = self.inorder_items();
        return RbIter {
            iter: stack.into_iter(),
        };
    }
}
