mod graph;

use graph::{Graph, NodeType};

fn main() {
    let mut g = Graph::new();
    let a = g.add_node(NodeType::ZSpider(0.0));
    let b = g.add_node(NodeType::XSpider(0.9));
    let c = g.add_node(NodeType::Boundary);
    g.add_edge(a, b);
    g.add_edge(b, c);

    let dot = g.to_dot();
    std::fs::write("target/debug/examples/graph.dot", dot).unwrap();
}