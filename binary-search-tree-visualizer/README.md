# Binary Search Tree Visualizer

[![Crates.io](https://img.shields.io/crates/v/binary-search-tree-visualizer.svg)](https://crates.io/crates/binary-search-tree-visualizer)
[![Docs.rs](https://docs.rs/binary-search-tree-visualizer/badge.svg)](https://docs.rs/binary-search-tree-visualizer)

A Rust crate that provides visualization tools for binary search trees, including both ASCII art and SVG generation. This crate is great for educational purposes and debugging binary search tree implementations.

## Features

- ASCII art visualization of binary search trees
- SVG generation for high-quality tree visualization
- Simple and intuitive API
- Customizable visualization parameters
- Support for any type that implements `Ord` and `Display` traits

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
binary-search-tree-visualizer = "0.1.0"
```
Or use:
```sh
cargo add binary-search-tree-visualizer
```

## Usage

Here's a simple example of how to use the crate:

```rust
use binary_search_tree_visualizer::{BinarySearchTree, AsciiVisualizer, SvgVisualizer};
use binary_search_tree_visualizer::visualizer::TreeVisualizer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a binary search tree
    let mut tree = BinarySearchTree::new();

    // Insert some values
    tree.insert(5);
    tree.insert(3);
    tree.insert(7);
    tree.insert(1);
    tree.insert(9);

    // Generate ASCII visualization
    let ascii_viz = AsciiVisualizer;
    let ascii_output = ascii_viz.visualize(&tree)?;
    println!("{}", ascii_output);

    // Generate SVG visualization
    let svg_viz = SvgVisualizer::default();
    let svg_output = svg_viz.visualize(&tree)?;

    // Save SVG to file
    std::fs::write("tree.svg", svg_output)?;

    Ok(())
}
```

## Visualization Examples

### ASCII Art Output
```
└── 5
    ┌── 7
    │   ┌── 9
    └── 3
        └── 1
```

### SVG Output
The SVG visualization will be saved to a file and can be viewed in any web browser or SVG-compatible viewer.

## Customization

The `SvgVisualizer` can be customized with different parameters:

```rust
let svg_viz = SvgVisualizer {
    node_radius: 25.0,    // Size of node circles
    level_height: 80.0,   // Vertical spacing between levels
    horizontal_spacing: 50.0, // Horizontal spacing between nodes
};
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.