use binary_search_tree_visualizer::{BinarySearchTree, AsciiVisualizer, SvgVisualizer};
use binary_search_tree_visualizer::visualizer::TreeVisualizer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut tree = BinarySearchTree::new();
    for &value in &[8, 3, 10, 1, 6, 14, 4, 7, 13] {
        tree.insert(value);
    }

    let ascii = AsciiVisualizer;
    println!("ASCII Visualization:\n{}", ascii.visualize(&tree)?);

    let svg = SvgVisualizer::default();
    std::fs::write("example_tree.svg", svg.visualize(&tree)?)?;
    println!("SVG visualization has been saved to 'example_tree.svg'");

    Ok(())
}
