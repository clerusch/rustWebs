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
    
    // Find three connected vertices in the graph
    let mut found_vertices = None;
    
    // Look for a vertex that has at least two neighbors
    for v1 in graph.vertices() {
        let neighbors: Vec<_> = graph.neighbors(v1).collect();
        if neighbors.len() >= 2 {
            // Found a vertex with at least two neighbors
            let v2 = neighbors[0];
            // Find a common neighbor between v1 and v2 (if any)
            let v3 = neighbors[1];
            found_vertices = Some((v1, v2, v3));
            break;
        }
    }
    
    // If no vertex with two neighbors found, find any three connected vertices
    if found_vertices.is_none() {
        'outer: for v1 in graph.vertices() {
            for v2 in graph.neighbors(v1) {
                for v3 in graph.neighbors(v2) {
                    if v3 != v1 {  // Ensure we don't have a triangle
                        found_vertices = Some((v1, v2, v3));
                        break 'outer;
                    }
                }
            }
        }
    }
    
    // If still no connected vertices found, create a simple triangle
    let (v1, v2, v3) = if let Some(verts) = found_vertices {
        println!("Found connected vertices: {}, {}, {}", verts.0, verts.1, verts.2);
        verts
    } else {
        // If no connected vertices found, use the first three vertices
        println!("No three connected vertices found, using first three vertices");
        let vertices: Vec<_> = graph.vertices().take(3).collect();
        if vertices.len() < 3 {
            return Err("Graph must have at least 3 vertices".to_string());
        }
        (vertices[0], vertices[1], vertices[2])
    };
    
    // Create a PauliWeb and add operators to the edges
    let mut pauli_web = PauliWeb::new();
    
    // Add edges between the vertices
    if graph.neighbors(v1).any(|n| n == v2) {
        println!("Adding edge {} -- {} with Pauli X", v1, v2);
        pauli_web.set_edge(v1, v2, Pauli::X);
    }
    
    if graph.neighbors(v2).any(|n| n == v3) {
        println!("Adding edge {} -- {} with Pauli Z", v2, v3);
        pauli_web.set_edge(v2, v3, Pauli::Z);
    }
    
    // Also try to add an edge between v1 and v3 if it exists
    if graph.neighbors(v1).any(|n| n == v3) {
        println!("Adding edge {} -- {} with Pauli Y", v1, v3);
        pauli_web.set_edge(v1, v3, Pauli::Y);
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
