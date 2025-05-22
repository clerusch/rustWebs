// use rust_web::detection_webs::DetectionWebs;
// use rust_web::graph_visualizer::graph_to_png;
// use quizx::hash_graph::Graph as QuizxGraph;
// use quizx::graph::{VType, VData, GraphLike};
// use std::path::Path;
// use num::rational::Rational64;
// use rust_web::PauliWeb;

// #[test]
// fn test_detection_webs_visualization() -> Result<(), Box<dyn std::error::Error>> {
//     // Create the test graph with proper vertex data
//     let mut graph = QuizxGraph::new();
    
//     // Helper function to add a vertex with position data
//     let mut add_vertex = |ty: VType, qubit: f64, row: f64| -> usize {
//         graph.add_vertex_with_data(VData {
//             ty,
//             phase: Rational64::new(0, 1).into(),
//             qubit,
//             row,
//         })
//     };
    
//     // Add vertices with positions
//     let v1 = add_vertex(VType::B, 0.0, 0.0);  // Input 1
//     let v2 = add_vertex(VType::B, 1.0, 0.0);  // Input 2
//     let v3 = add_vertex(VType::Z, 0.0, 1.0);  // Z-spider 1
//     let v4 = add_vertex(VType::Z, 1.0, 1.0);  // Z-spider 2
//     let v5 = add_vertex(VType::Z, 0.0, 2.0);  // Z-spider 3
//     let v6 = add_vertex(VType::Z, 1.0, 2.0);  // Z-spider 4
//     let v7 = add_vertex(VType::B, 0.0, 3.0);  // Output 1
//     let v8 = add_vertex(VType::B, 1.0, 3.0);  // Output 2
    
//     // Add edges
//     graph.add_edge(v1, v3);
//     graph.add_edge(v2, v4);
//     graph.add_edge(v3, v4);
//     graph.add_edge(v3, v5);
//     graph.add_edge(v4, v6);
//     graph.add_edge(v5, v6);
//     graph.add_edge(v5, v7);
//     graph.add_edge(v6, v8);
    
//     // Set inputs and outputs
//     graph.set_inputs(vec![v1, v2]);
//     graph.set_outputs(vec![v7, v8]);
    
//     // Create detection webs
//     let detector = DetectionWebs::new(graph.clone());
//     println!("Getting detection webs...");
//     let webs = detector.get_detection_webs()?;
//     println!("Found {} detection webs", webs.len());
    
//     // Print information about each web
//     for (i, web) in webs.iter().enumerate() {
//         println!("Web {} has {} edges", i, web.edge_operators.len());
//         for ((v1, v2), pauli) in &web.edge_operators {
//             println!("  Edge {} -- {}: {:?}", v1, v2, pauli);
//         }
//     }
    
//     // Create output directory if it doesn't exist
//     let output_dir = "tests/output";
//     if !Path::new(output_dir).exists() {
//         std::fs::create_dir_all(output_dir)?;
//     }
    
//     // Visualize the original graph
//     let dot_path = format!("{}/detection_web_original.dot", output_dir);
//     let png_path = format!("{}/detection_web_original.png", output_dir);
//     println!("Saving original graph to {}", png_path);
//     graph_to_png::<QuizxGraph>(&graph, &dot_path, &png_path, None::<&PauliWeb>, true)?;
    
//     // Visualize each detection web
//     for (i, web) in webs.iter().enumerate() {
//         // Create a new graph containing only the nodes and edges in this detection web
//         let mut subgraph = QuizxGraph::new();
//         let mut node_map = std::collections::HashMap::new();
        
//         // Add all nodes from the original graph that are in this detection web
//         for &(v1, v2) in web.edge_operators.keys() {
//             if !node_map.contains_key(&v1) {
//                 let data = graph.vertex_data(v1);
//                 let new_v = subgraph.add_vertex_with_data(VData {
//                     ty: data.ty,
//                     phase: data.phase,
//                     qubit: data.qubit,
//                     row: data.row,
//                 });
//                 node_map.insert(v1, new_v);
//             }
//             if !node_map.contains_key(&v2) {
//                 let data = graph.vertex_data(v2);
//                 let new_v = subgraph.add_vertex_with_data(VData {
//                     ty: data.ty,
//                     phase: data.phase,
//                     qubit: data.qubit,
//                     row: data.row,
//                 });
//                 node_map.insert(v2, new_v);
//             }
//         }
        
//         // Add all edges from the detection web
//         for (&(v1, v2), _) in &web.edge_operators {
//             let new_v1 = node_map[&v1];
//             let new_v2 = node_map[&v2];
//             subgraph.add_edge(new_v1, new_v2);
//         }
        
//         // Set inputs and outputs for the subgraph
//         let inputs: Vec<_> = graph.inputs()
//             .iter()
//             .filter(|&&v| node_map.contains_key(&v))
//             .map(|&v| node_map[&v])
//             .collect();
//         let outputs: Vec<_> = graph.outputs()
//             .iter()
//             .filter(|&&v| node_map.contains_key(&v))
//             .map(|&v| node_map[&v])
//             .collect();
            
//         subgraph.set_inputs(inputs);
//         subgraph.set_outputs(outputs);
        
//         // Visualize the subgraph with the detection web coloring
//         let dot_path = format!("{}/detection_web_{}.dot", output_dir, i);
//         let png_path = format!("{}/detection_web_{}.png", output_dir, i);
//         println!("Saving detection web {} to {}", i, png_path);
//         graph_to_png::<QuizxGraph>(&subgraph, &dot_path, &png_path, Some(web), true)?;
//     }
    
//     Ok(())
// }
