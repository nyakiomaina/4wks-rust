//! # Binary Search Tree Visualizer
//!
//! This crate provides visualization tools for binary search trees, including both ASCII art and SVG generation.
//! It is great for educational purposes and debugging binary search tree implementations.
//!
//! ## Features
//! - ASCII art visualization of binary search trees
//! - SVG generation for high-quality tree visualization
//! - Simple and intuitive API
//! - Customizable visualization parameters
//! - Support for any type that implements `Ord` and `Display` traits

/// The binary search tree module.
pub mod tree;
/// The visualizer module, including ASCII and SVG visualizers.
pub mod visualizer;

pub use tree::BinarySearchTree;
pub use visualizer::{AsciiVisualizer, SvgVisualizer};

/// Error types for the binary search tree visualizer.
#[derive(Debug, thiserror::Error)]
pub enum VisualizerError {
    /// The tree structure is invalid.
    #[error("Invalid tree structure: {0}")]
    InvalidTree(String),
    /// An IO error occurred.
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Result type for the binary search tree visualizer.
pub type Result<T> = std::result::Result<T, VisualizerError>;