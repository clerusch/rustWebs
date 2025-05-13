mod test_fusion;
mod tikz_export;
use quizx::hash_graph::VType::{X, Z};
use quizx::graph::VData;
use quizx::{graph::GraphLike, hash_graph::Graph};
use num::rational::Rational64;
use std::env;
use std::fs::write;
use std::error::Error;

/// Creates a vertex with Z-type and given position data
fn create_z_vertex(g: &mut Graph, qubit: f64, row: f64) -> usize {
    g.add_vertex_with_data(VData {
        ty: Z,
        phase: Rational64::new(0, 1).into(),
        qubit,
        row,
    })
}

/// Creates a simple test graph with Z-type vertices
fn create_test_graph() -> Graph {
    let mut g = Graph::new();
    let a = g.add_vertex(Z);
    let b = create_z_vertex(&mut g, 0.0, 1.0);
    let c = create_z_vertex(&mut g, 1.0, 0.0);
    let d = create_z_vertex(&mut g, 1.0, 1.0);
    let e = create_z_vertex(&mut g, 2.0, 0.0);
    let f = create_z_vertex(&mut g, 2.0, 1.0);

    // Add edges to connect vertices
    g.add_edge(e, f);
    g.add_edge(c, d);
    g.add_edge(a, b);
    
    g
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Running from: {:?}", env::current_dir()?);
    
    // Create and export the test graph
    let g = create_test_graph();
    
    // Export in both DOT and TikZ formats
    write("target/debug/examples/graphtest.dot", g.to_dot())?;
    tikz_export::export_to_tikz(&g, "target/debug/examples/graphtest.tex")?;
    
    // Run fusion test
    test_fusion::test_fusion();
    
    Ok(())
}