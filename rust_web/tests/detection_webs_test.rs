use std::path::Path;
use rust_web::{
    DetectionWebs,
    draw_graph_with_pauliweb,
    load_graph,
};

#[test]
fn test_detection_webs() -> Result<(), String> {
    // Load a test graph
    let graph_path = "tests/zxgs/xxx_final.zxg";
    println!("Loading graph from: {}", graph_path);
    
    let graph = match load_graph(graph_path) {
        Ok(g) => g,
        Err(e) => return Err(format!("Failed to load graph: {}", e)),
    };

    // Create detection webs analyzer
    let detector = DetectionWebs::new(graph);
    
    // Get detection webs
    println!("Finding detection webs...");
    let webs = match detector.get_detection_webs() {
        Ok(w) => w,
        Err(e) => return Err(format!("Failed to get detection webs: {}", e)),
    };

    println!("Found {} detection webs", webs.len());
    
    // Create output directory if it doesn't exist
    let output_dir = "tests/output";
    if !Path::new(output_dir).exists() {
        std::fs::create_dir_all(output_dir).map_err(|e| e.to_string())?;
    }

    // Draw each web
    for (i, web) in webs.iter().enumerate() {
        let output_path = format!("{}/detection_web_{}.svg", output_dir, i);
        println!("Drawing detection web {} to {}", i, output_path);
        
        // Get a reference to the graph
        let graph_ref = &detector.graph;
        
        // Draw the graph with the Pauli web overlaid
        if let Err(e) = draw_graph_with_pauliweb(graph_ref, web, &output_path) {
            eprintln!("Failed to draw detection web {}: {}", i, e);
        }
    }

    Ok(())
}
