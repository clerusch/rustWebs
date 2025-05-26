use rust_web::{
    graph_loader::load_graph, 
    fuckai::get_detection_webs,
    graph_visualizer,
    make_rg::make_rg
};
use std::path::PathBuf;
use std::error::Error;
use std::fs::create_dir_all;

fn main() -> Result<(), Box<dyn Error>> {
    // Configure logging
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Debug)
        .init();

    // Load the Steane code graph
    let manifest_dir = std::
    env::
    var("CARGO_MANIFEST_DIR")
    .map_err(|e| format!("Failed to get manifest dir: {}", e))?;
    
    let output_dir = "detection_web_visualizations";
    create_dir_all(output_dir)
    .map_err(|e| format!("Failed to create output directory: {}", e))?;
    let graph_path = PathBuf::from(&manifest_dir)
        .join("tests")
        .join("zxgs")
        .join("xx_stab.zxg");
    
    let mut graph = load_graph(graph_path.to_str().unwrap())?;
    make_rg(&mut graph);
    // let detection_webs = DetectionWebs::new(rg_graph.clone());
    let webs = get_detection_webs(&mut graph);
    let counter = 0;
    for web in webs {
        log::info!("Detection web: {:#?}", web);
        let output_filename = format!("detection_web_{}.png", counter + 1);
        let output_path = format!("{}/{}", output_dir, output_filename);
         // Create a temporary DOT file
         let dot_path = format!("{}/temp_detection_web_{}.dot", output_dir, counter + 1);
         let dot_content = graph_visualizer::to_dot_with_positions(&graph, Some(&web), false);
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
             
         if !status.success() {
             let error_msg = match status.code() {
                 Some(code) => format!("neato command failed with exit code: {}", code),
                 None => "neato command was terminated by a signal".to_string(),
             };
             log::error!("Failed to generate PNG: {}", error_msg);
             return Err(error_msg.into());
         }
         
         // Clean up the temporary DOT file
         let _ = std::fs::remove_file(dot_path);

    }
    Ok(())
}