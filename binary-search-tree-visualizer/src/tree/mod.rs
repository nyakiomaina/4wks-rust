use std::fmt::Debug;

/// A node in the binary search tree
#[derive(Debug)]
pub struct Node<T> {
    pub value: T,
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }
}

/// A binary search tree implementation
#[derive(Debug)]
pub struct BinarySearchTree<T> {
    pub root: Option<Box<Node<T>>>,
}

impl<T: Ord + Debug> BinarySearchTree<T> {
    /// Creates a new empty binary search tree
    pub fn new() -> Self {
        Self { root: None }
    }

    /// Inserts a value into the tree
    pub fn insert(&mut self, value: T) {
        Self::insert_recursive(&mut self.root, value);
    }

    fn insert_recursive(node: &mut Option<Box<Node<T>>>, value: T) {
        match node {
            None => *node = Some(Box::new(Node::new(value))),
            Some(ref mut n) => {
                if value < n.value {
                    Self::insert_recursive(&mut n.left, value);
                } else {
                    Self::insert_recursive(&mut n.right, value);
                }
            }
        }
    }

    /// Returns the height of the tree
    pub fn height(&self) -> usize {
        self.height_recursive(&self.root)
    }

    fn height_recursive(&self, node: &Option<Box<Node<T>>>) -> usize {
        match node {
            None => 0,
            Some(node) => {
                1 + std::cmp::max(
                    self.height_recursive(&node.left),
                    self.height_recursive(&node.right),
                )
            }
        }
    }

    /// Returns an iterator over the tree nodes in-order
    pub fn inorder(&self) -> InOrderIterator<T> {
        InOrderIterator::new(&self.root)
    }
}

/// Iterator for in-order traversal of the tree
pub struct InOrderIterator<'a, T> {
    stack: Vec<&'a Node<T>>,
    current: Option<&'a Node<T>>,
}

impl<'a, T> InOrderIterator<'a, T> {
    fn new(root: &'a Option<Box<Node<T>>>) -> Self {
        let mut iter = Self {
            stack: Vec::new(),
            current: None,
        };
        if let Some(node) = root {
            iter.current = Some(node);
        }
        iter
    }
}

impl<'a, T> Iterator for InOrderIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.current {
            self.stack.push(node);
            self.current = node.left.as_deref();
        }

        if let Some(node) = self.stack.pop() {
            self.current = node.right.as_deref();
            Some(&node.value)
        } else {
            None
        }
    }
}