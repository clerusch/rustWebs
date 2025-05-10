mod test_fusion;
mod tikz_export;
use std::env;
use quizx::{graph::GraphLike, hash_graph::Graph};
use std::fs::write;
use quizx::hash_graph::VType::{X,Z};
fn main(){
    println!("Running from: {:?}", env::current_dir().unwrap());
    let mut g = Graph::new();
    let a = g.add_vertex(Z);
    let b = g.add_vertex(X);
    g.add_edge(a, b);
    // let g = test_fusion::create_chain(10);
    // let dot = g.to_dot();
    let _ = write("target/debug/examples/graphtest.dot", g.to_dot());
    test_fusion::test_fusion();
    
}