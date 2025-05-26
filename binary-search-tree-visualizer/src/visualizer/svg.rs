use crate::tree::{BinarySearchTree, Node};
use crate::Result;
use crate::visualizer::TreeVisualizer;
use svg::node::element::{Circle, Line, Text};
use svg::Document;

/// SVG visualizer for binary search trees
pub struct SvgVisualizer {
    pub node_radius: f64,
    pub level_height: f64,
    pub horizontal_spacing: f64,
}

impl Default for SvgVisualizer {
    fn default() -> Self {
        Self {
            node_radius: 20.0,
            level_height: 60.0,
            horizontal_spacing: 40.0,
        }
    }
}

impl<T: std::fmt::Display> TreeVisualizer<T> for SvgVisualizer {
    fn visualize(&self, tree: &BinarySearchTree<T>) -> Result<String> {
        let mut document = Document::new()
            .set("width", "800")
            .set("height", "600")
            .set("viewBox", "0 0 800 600");

        if let Some(root) = &tree.root {
            let width = self.calculate_width(root);
            let start_x = 400.0 - (width * self.horizontal_spacing) / 2.0;
            self.draw_node(root, start_x, 50.0, &mut document);
        }

        Ok(document.to_string())
    }
}

impl SvgVisualizer {
    fn calculate_width<T>(&self, node: &Node<T>) -> f64 {
        let left_width = if let Some(left) = &node.left {
            self.calculate_width(left)
        } else {
            0.0
        };
        let right_width = if let Some(right) = &node.right {
            self.calculate_width(right)
        } else {
            0.0
        };
        1.0 + left_width + right_width
    }

    fn draw_node<T: std::fmt::Display>(
        &self,
        node: &Node<T>,
        x: f64,
        y: f64,
        document: &mut Document,
    ) {
        // Draw node circle
        let circle = Circle::new()
            .set("cx", x)
            .set("cy", y)
            .set("r", self.node_radius)
            .set("fill", "white")
            .set("stroke", "black")
            .set("stroke-width", 2);

        // Draw node value
        let text = Text::new()
            .set("x", x)
            .set("y", y + 5.0)
            .set("text-anchor", "middle")
            .set("font-size", "14")
            .add(svg::node::Text::new(format!("{}", node.value)));

        *document = document.clone().add(circle).add(text);

        // Draw connections to children
        if let Some(left) = &node.left {
            let left_x = x - self.horizontal_spacing;
            let left_y = y + self.level_height;

            let line = Line::new()
                .set("x1", x)
                .set("y1", y + self.node_radius)
                .set("x2", left_x)
                .set("y2", left_y - self.node_radius)
                .set("stroke", "black")
                .set("stroke-width", 2);

            *document = document.clone().add(line);
            self.draw_node(left, left_x, left_y, document);
        }

        if let Some(right) = &node.right {
            let right_x = x + self.horizontal_spacing;
            let right_y = y + self.level_height;

            let line = Line::new()
                .set("x1", x)
                .set("y1", y + self.node_radius)
                .set("x2", right_x)
                .set("y2", right_y - self.node_radius)
                .set("stroke", "black")
                .set("stroke-width", 2);

            *document = document.clone().add(line);
            self.draw_node(right, right_x, right_y, document);
        }
    }
}