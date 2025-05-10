mod graph;

use std::fs::write;

use graph::{Graph, NodeType};

fn main() {
    let mut g = Graph::new();
    let a = g.add_node(NodeType::ZSpider(0.2));
    let b = g.add_node(NodeType::ZSpider(0.9));
    let c = g.add_node(NodeType::Boundary);
    let d = g.add_node(NodeType::XSpider(0.0));
    g.add_edge(b, d);
    g.add_edge(a, b);
    g.add_edge(b, c);
    g.add_edge(d, a);

    let dot1 = g.to_dot();
    write("target/debug/examples/graphBeforeRules.dot", dot1).unwrap();

    if let Err(e) = g.remove_identity_spider(d) {
        eprintln!("Simplification failed: {}", e);
    }
    g.fuse_spiders(a, b).unwrap();
    
    
    
    let dot2 = g.to_dot();
    write("target/debug/examples/graphAfterRules.dot", dot2).unwrap();
}