// use std::path::Path;
// use rust_web::{
//     DetectionWebs,
//     load_graph,
//     graph_visualizer,
// };

// #[test]
// fn test_detection_webs() -> Result<(), String> {
//     // Load a test graph
//     let graph_path = "tests/zxgs/steane_style_steane_2_rounds.zxg";
//     println!("Loading graph from: {}", graph_path);
    
//     let graph = match load_graph(graph_path) {
//         Ok(g) => g,
//         Err(e) => return Err(format!("Failed to load graph: {}", e)),
//     };

//     // Create detection webs analyzer
//     let detector = DetectionWebs::new(graph);
    
//     // Get detection webs
//     println!("Finding detection webs...");
//     let webs = match detector.get_detection_webs() {
//         Ok(w) => w,
//         Err(e) => return Err(format!("Failed to get detection webs: {}", e)),
//     };

//     println!("Found {} detection webs", webs.len());
    
//     // Create output directory if it doesn't exist
//     let output_dir = "tests/output";
//     if !Path::new(output_dir).exists() {
//         std::fs::create_dir_all(output_dir).map_err(|e| e.to_string())?;
//     }

//     // Draw each web
//     for (i, web) in webs.iter().enumerate() {
//         let base_path = format!("{}/detection_web_{}", output_dir, i);
//         let dot_path = format!("{}.dot", base_path);
//         let png_path = format!("{}.png", base_path);
        
//         println!("Drawing detection web {} to {}", i, png_path);
        
//         // Get a reference to the graph
//         let graph_ref = &detector.graph;
        
//         // Generate both DOT and PNG files
//         if let Err(e) = graph_visualizer::graph_to_png(
//             graph_ref, 
//             &dot_path, 
//             &png_path, 
//             Some(web),
//             true  // Show node IDs
//         ) {
//             eprintln!("Failed to draw detection web {}: {}", i, e);
//         } else {
//             println!("Created DOT: {}", dot_path);
//             println!("Created PNG: {}", png_path);
//         }
//     }

//     Ok(())
// }
