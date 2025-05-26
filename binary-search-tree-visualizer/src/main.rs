use binary_search_tree_visualizer::{BinarySearchTree, AsciiVisualizer, SvgVisualizer};
use binary_search_tree_visualizer::visualizer::TreeVisualizer;
use std::fs;

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
    println!("ASCII Visualization:\n{}", ascii_output);

    // Generate SVG visualization
    let svg_viz = SvgVisualizer::default();
    let svg_output = svg_viz.visualize(&tree)?;

    // Save SVG to file
    fs::write("tree.svg", svg_output)?;
    println!("SVG visualization has been saved to 'tree.svg'");

    Ok(())
}
