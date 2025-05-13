mod test_fusion;
mod tikz_export;
use quizx::hash_graph::VType::{X, Z};
use quizx::graph::VData;
use quizx::{graph::GraphLike, hash_graph::Graph};
use num::rational::Rational64;
use std::env;
use std::fs::write;
fn main() {
    println!("Running from: {:?}", env::current_dir().unwrap());
    let mut g = Graph::new();
    let a = g.add_vertex(Z);
    let b = g.add_vertex_with_data(VData {
        ty: Z,
        phase: Rational64::new(0, 1).into(),
        qubit: 0.0,
        row: 1.0,
    });
    let c = g.add_vertex_with_data(VData {
        ty: Z,
        phase: Rational64::new(0, 1).into(),
        qubit: 1.0,
        row: 0.0,
    });
    let d = g.add_vertex_with_data(VData {
        ty: Z,
        phase: Rational64::new(0, 1).into(),
        qubit: 1.0,
        row: 1.0,
    });
    let e = g.add_vertex_with_data(VData {
        ty: Z,
        phase: Rational64::new(0, 1).into(),
        qubit: 2.0,
        row: 0.0,
    });
    let f = g.add_vertex_with_data(VData {
        ty: Z,
        phase: Rational64::new(0, 1).into(),
        qubit: 2.0,
        row: 1.0,
    });
    g.add_edge(e, f);
    g.add_edge(c,d);
    // g.add_edge(b, c);
    g.add_edge(a, b);
    // let g = test_fusion::create_chain(10);
    // let dot = g.to_dot();
    let _ = write("target/debug/examples/graphtest.dot", g.to_dot());
    test_fusion::test_fusion();
}