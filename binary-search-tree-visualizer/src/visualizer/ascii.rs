use crate::tree::{BinarySearchTree, Node};
use crate::Result;
use crate::visualizer::TreeVisualizer;

/// ASCII art visualizer for binary search trees
pub struct AsciiVisualizer;

impl<T: std::fmt::Display> TreeVisualizer<T> for AsciiVisualizer {
    fn visualize(&self, tree: &BinarySearchTree<T>) -> Result<String> {
        let mut result = String::new();
        if let Some(root) = &tree.root {
            self.visualize_node(root, "", true, &mut result);
        }
        Ok(result)
    }
}

impl AsciiVisualizer {
    fn visualize_node<T: std::fmt::Display>(
        &self,
        node: &Node<T>,
        prefix: &str,
        is_left: bool,
        result: &mut String,
    ) {
        let connector = if is_left { "└── " } else { "┌── " };
        result.push_str(&format!("{}{}{}\n", prefix, connector, node.value));

        let new_prefix = format!("{}{}", prefix, if is_left { "    " } else { "│   " });

        if let Some(right) = &node.right {
            self.visualize_node(right, &new_prefix, false, result);
        }
        if let Some(left) = &node.left {
            self.visualize_node(left, &new_prefix, true, result);
        }
    }
}