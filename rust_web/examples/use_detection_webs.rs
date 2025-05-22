use rust_web::{
    graph_loader::load_graph, 
    detection_webs::DetectionWebs,
    graph_visualizer,
    GraphLike
};
use std::path::PathBuf;
use std::time::Instant;
use std::error::Error;
use std::fs::create_dir_all;

fn main() -> Result<(), Box<dyn Error>> {
    // Configure logging
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    // Load the Steane code graph
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").map_err(|e| format!("Failed to get manifest dir: {}", e))?;
    let graph_path = PathBuf::from(&manifest_dir)
        .join("tests")
        .join("zxgs")
        .join("steane_style_steane_2_rounds.zxg");
    
    if !graph_path.exists() {
        return Err(format!("Graph file not found at: {}", graph_path.display()).into());
    }
    
    log::info!("Loading graph from: {}", graph_path.display());
    
    let start_time = Instant::now();
    let graph = match load_graph(graph_path.to_str().unwrap()) {
        Ok(g) => g,
        Err(e) => {
            log::error!("Failed to load graph: {}", e);
            return Err(e.into());
        }
    };
    
    let load_time = start_time.elapsed();
    log::info!("Graph loaded in {:?}", load_time);
    log::info!("Graph has {} vertices and {} edges", 
        graph.num_vertices(), 
        graph.num_edges());
    
    // Log basic graph information
    let total_vertices = graph.num_vertices();
    log::info!("Total vertices in graph: {}", total_vertices);
    
    // Log the number of inputs and outputs
    let num_inputs = graph.inputs().len();
    let num_outputs = graph.outputs().len();
    log::info!("Number of inputs: {}", num_inputs);
    log::info!("Number of outputs: {}", num_outputs);
    
    // Log the number of edges
    let num_edges = graph.num_edges();
    log::info!("Number of edges: {}", num_edges);
    
    // Count vertex types
    let mut boundary_count = 0;
    let mut x_count = 0;
    let mut z_count = 0;
    let mut h_count = 0;
    let mut other_count = 0;
    
    for v in graph.vertices() {
        match graph.vertex_type(v) {
            quizx::graph::VType::B => boundary_count += 1,
            quizx::graph::VType::X => x_count += 1,
            quizx::graph::VType::Z => z_count += 1,
            quizx::graph::VType::H => h_count += 1,
            _ => other_count += 1,
        }
    }
    
    log::info!("Vertex type counts:");
    log::info!("  Boundary (B): {}", boundary_count);
    log::info!("  X: {}", x_count);
    log::info!("  Z: {}", z_count);
    log::info!("  H: {}", h_count);
    if other_count > 0 {
        log::info!("  Other: {}", other_count);
    }
    
    // Log a few vertices for debugging
    log::debug!("Sample vertices: {:?}", graph.vertices().take(5).collect::<Vec<_>>());
    
    // Log graph information
    log::info!("Graph information:");
    log::info!("  Number of vertices: {}", graph.num_vertices());
    log::info!("  Number of edges: {}", graph.num_edges());
    
    // Create detection webs
    log::info!("\nFinding detection webs...");
    let detection_start = Instant::now();
    
    // Create a wrapper around the graph to log vertex access
    log::info!("Creating detection webs analyzer...");
    
    // Clone the graph before moving it into DetectionWebs
    let graph_clone = graph.clone();
    let detector = DetectionWebs::new(graph_clone);
    
    log::info!("Starting detection web analysis...");
    let webs_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        detector.get_detection_webs()
    }));
    
    let webs = match webs_result {
        Ok(Ok(w)) => w,
        Ok(Err(e)) => {
            log::error!("Error finding detection webs: {}", e);
            log::error!("This might be due to an issue with the graph structure or a bug in the detection algorithm.");
            return Err(e.into());
        },
        Err(panic) => {
            log::error!("Panic occurred during detection web analysis");
            if let Some(msg) = panic.downcast_ref::<&'static str>() {
                log::error!("Panic message: {}", msg);
            } else if let Some(msg) = panic.downcast_ref::<String>() {
                log::error!("Panic message: {}", msg);
            }
            return Err("Panic during detection web analysis".into());
        }
    };
    
    let detection_time = detection_start.elapsed();
    log::info!("Found {} detection webs in {:?}", webs.len(), detection_time);
    
    if webs.is_empty() {
        log::warn!("No detection webs found in the graph");
    } else {
        // Print information about each detection web
        for (i, web) in webs.iter().enumerate() {
            log::info!("\nDetection Web {}:", i + 1);
            log::info!("Number of edges: {}", web.edge_operators.len());
            
            // Count the different Pauli operators
            let mut x_count = 0;
            let mut y_count = 0;
            let mut z_count = 0;
            
            for (&(v1, v2), &pauli) in &web.edge_operators {
                match pauli {
                    rust_web::pauliweb::Pauli::X => x_count += 1,
                    rust_web::pauliweb::Pauli::Y => y_count += 1,
                    rust_web::pauliweb::Pauli::Z => z_count += 1,
                }
                log::debug!("Edge {} -- {}: {:?}", v1, v2, pauli);
            }
            
            log::info!("  X operators: {}", x_count);
            log::info!("  Y operators: {}", y_count);
            log::info!("  Z operators: {}", z_count);
            
            // Print the first few edges if there are many
            let edges_to_print = 5;
            if web.edge_operators.len() > edges_to_print {
                log::info!("  First {} edges:", edges_to_print);
                for (i, ((v1, v2), pauli)) in web.edge_operators.iter().take(edges_to_print).enumerate() {
                    log::info!("    {}. {} -- {}: {:?}", i + 1, v1, v2, pauli);
                }
                log::info!("    ... and {} more edges", web.edge_operators.len() - edges_to_print);
            } else if !web.edge_operators.is_empty() {
                log::info!("  Edges:");
                for (i, ((v1, v2), pauli)) in web.edge_operators.iter().enumerate() {
                    log::info!("    {}. {} -- {}: {:?}", i + 1, v1, v2, pauli);
                }
            }
        }
    }
    
    let total_time = start_time.elapsed();
    log::info!("\nTotal execution time: {:?}", total_time);
    log::info!("  - Graph loading: {:?} ({:.2}%)", 
        load_time, 
        (load_time.as_secs_f64() / total_time.as_secs_f64()) * 100.0);
    log::info!("  - Detection webs: {:?} ({:.2}%)",
        detection_time,
        (detection_time.as_secs_f64() / total_time.as_secs_f64()) * 100.0);
    
    // Create a directory to save the visualizations
    let output_dir = "detection_web_visualizations";
    create_dir_all(output_dir).map_err(|e| format!("Failed to create output directory: {}", e))?;
    log::info!("\nSaving visualizations to: {}", output_dir);
    
    // Visualize the first 5 detection webs
    if !webs.is_empty() {
        let num_webs_to_visualize = 5.min(webs.len());
        log::info!("\nVisualizing first {} detection webs...", num_webs_to_visualize);
        
        for i in 0..num_webs_to_visualize {
            let web = &webs[i];
            let output_filename = format!("detection_web_{}.png", i + 1);
            let output_path = format!("{}/{}", output_dir, output_filename);
            
            log::info!("  Creating visualization {}/{}: {}", 
                i + 1, 
                num_webs_to_visualize, 
                output_path);
            
            // Count the number of X, Y, Z operators in this web
            let mut x_count = 0;
            let mut y_count = 0;
            let mut z_count = 0;
            
            for (_, &pauli) in &web.edge_operators {
                match pauli {
                    rust_web::pauliweb::Pauli::X => x_count += 1,
                    rust_web::pauliweb::Pauli::Y => y_count += 1,
                    rust_web::pauliweb::Pauli::Z => z_count += 1,
                }
            }
            
            log::info!("    X operators: {}", x_count);
            log::info!("    Y operators: {}", y_count);
            log::info!("    Z operators: {}", z_count);
            
            // Create a temporary DOT file
            let dot_path = format!("{}/temp_detection_web_{}.dot", output_dir, i + 1);
            let dot_content = graph_visualizer::to_dot_with_positions(&graph, Some(web), false);
            std::fs::write(&dot_path, dot_content)
                .map_err(|e| format!("Failed to write DOT file: {}", e))?;
            
            // Convert DOT to PNG using Graphviz's neato for better layout control
            let status = std::process::Command::new("neato")
                .arg("-n2")  // Use node positions from the DOT file
                .arg("-Tpng")
                .arg(&dot_path)
                .arg("-o")
                .arg(&output_path)
                .status()
                .map_err(|e| format!("Failed to run neato command: {}", e))?;
            
            // Clean up the temporary DOT file
            let _ = std::fs::remove_file(dot_path);
            
            if !status.success() {
                log::error!("Failed to generate PNG for detection web {}", i + 1);
            } else {
                log::info!("    Saved to {}", output_path);
            }
        }
    } else {
        log::warn!("No detection webs to visualize");
    }
    
    Ok(())
}