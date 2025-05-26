mod ascii;
mod svg;

pub use ascii::AsciiVisualizer;
pub use svg::SvgVisualizer;

use crate::tree::BinarySearchTree;
use crate::Result;

/// Trait for tree visualizers
pub trait TreeVisualizer<T> {
    /// Generates a visualization of the tree
    fn visualize(&self, tree: &BinarySearchTree<T>) -> Result<String>;
}