mod graph;

use graph::{Graph, NodeType};

fn main() {
    let mut g = Graph::new();
    let a = g.add_node(NodeType::ZSpider, 0.0);
    let b = g.add_node(NodeType::XSpider, 1.0);
    g.add_edge(a, b);

    let dot = g.to_dot();
    std::fs::write("graph.dot", dot).unwrap();
}