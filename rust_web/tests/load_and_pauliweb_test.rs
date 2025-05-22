use rust_web::graph_loader;
use rust_web::graph_visualizer;
use rust_web::pauliweb::{PauliWeb, Pauli};
use quizx::graph::GraphLike;
use std::path::Path;

#[test]
fn test_load_and_pauliweb() -> Result<(), String> {
    // Create output directory if it doesn't exist
    let output_dir = Path::new("tests/output");
    std::fs::create_dir_all(output_dir).map_err(|e| e.to_string())?;
    
    // Load the graph
    let graph = graph_loader::load_graph("tests/zxgs/xxx_final.zxg")?;
    
    // Get the first three vertices from the graph
    let vertices: Vec<_> = graph.vertices().collect();
    
    // Make sure we have at least 3 vertices
    assert!(vertices.len() >= 3, "Graph must have at least 3 vertices");
    
    let v1 = vertices[0];
    let v2 = vertices[1];
    let v3 = vertices[2];
    
    // Create a PauliWeb and add operators to the edges
    let mut pauli_web = PauliWeb::new();
    
    // Only add edges if they exist in the graph
    if graph.neighbors(v1).any(|n| n == v2) {
        pauli_web.set_edge(v1.into(), v2.into(), Pauli::X);
    }
    
    if graph.neighbors(v2).any(|n| n == v3) {
        pauli_web.set_edge(v2.into(), v3.into(), Pauli::Z);
    }
    
    // Generate output paths
    let dot_path = output_dir.join("loadgraph_with_pw_test.dot");
    let png_path = output_dir.join("loadgraph_with_pw_test.png");
    
    // Generate the visualization with node IDs
    graph_visualizer::graph_to_png(
        &graph, 
        dot_path.to_str().unwrap(), 
        png_path.to_str().unwrap(), 
        Some(&pauli_web),
        true  // Show node IDs
    ).map_err(|e| e.to_string())?;
    
    // Verify the files were created
    assert!(dot_path.exists(), "DOT file was not created");
    assert!(png_path.exists(), "PNG file was not created");
    
    Ok(())
}
