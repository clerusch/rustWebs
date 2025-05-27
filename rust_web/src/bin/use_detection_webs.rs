use rust_web::{
    graph_loader::load_graph,
    detection_webs::get_detection_webs,
    graph_visualizer,
    make_rg::make_rg
};
use std::error::Error;
use std::path::PathBuf;
use std::fs::create_dir_all;
use std::process::{Command, Stdio};
use std::env;
use std::io::Write;
use std::time::Instant;
use std::sync::{Arc, Mutex};
use log::{info, error, debug};
use rayon::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    
    // Get the input file path from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        error!("Please provide a path to a .zxg file");
        std::process::exit(1);
    }
    let path = &args[1];
    
    info!("Processing file: {}", path);
    
    // Run the detection web generation
    if let Err(e) = use_det_web(path) {
        error!("Error: {}", e);
        std::process::exit(1);
    }
    
    Ok(())
}

/// Main function to generate and visualize detection webs for a given ZXG file

pub fn use_det_web(path: &str) -> Result<(), Box<dyn Error>> {
    let total_start = Instant::now();
    info!("Starting detection web generation for: {}", path);

    // Set up output directory structure
    let input_path = std::path::Path::new(path);
    let base_output_dir = input_path.parent()
        .unwrap_or_else(|| std::path::Path::new("."))
        .join("detection_web_visualizations");
    
    // Create a subdirectory based on the input filename (without extension)
    let output_dir = base_output_dir.join(
        input_path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("output")
    );
    
    debug!("Output directory: {:?}", output_dir);
    create_dir_all(&output_dir)
        .map_err(|e| format!("Failed to create output directory: {}", e))?;

    // Try to find the input file in multiple possible locations
    let find_start = Instant::now();
    let possible_paths = [
        PathBuf::from(path),
        PathBuf::from(format!("{}.zxg", path.trim_end_matches(".zxg"))),
        PathBuf::from("tests").join("zxgs").join(path),
        PathBuf::from("tests").join("zxgs").join(format!("{}.zxg", path.trim_end_matches(".zxg"))),
    ];

    // Find the first path that exists and is a file
    let graph_path = possible_paths.iter()
        .find(|p| p.exists() && p.is_file())
        .ok_or_else(|| format!("Could not find input file: {}", path))?;
    debug!("Found graph at: {:?}", graph_path);
    info!("File search took: {:?}", find_start.elapsed());
    
    let load_start = Instant::now();
    let mut graph = load_graph(graph_path.to_str().ok_or("Invalid graph path encoding")?)?;
    info!("Graph loading took: {:?}", load_start.elapsed());
    
    let make_rg_start = Instant::now();
    make_rg(&mut graph);
    info!("make_rg took: {:?}", make_rg_start.elapsed());
    
    // Create output filenames
    let output_filename = "graph";
    let output_path = output_dir.join(output_filename).with_extension("png");
    
    // Generate and save the main graph visualization using piped I/O
    let vis_start = Instant::now();
    let dot_content = graph_visualizer::to_dot_with_positions(&graph, None, false);
    info!("Graph visualization generation took: {:?}", vis_start.elapsed());
    
    // Start neato process once
    let neato_start = Instant::now();
    let mut neato = Command::new("neato")
        .args(["-n2", "-Tpng"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    
    // Write dot content to neato's stdin
    if let Some(stdin) = neato.stdin.as_mut() {
        stdin.write_all(dot_content.as_bytes())?;
    }
    
    // Get the output and write to file
    let output = neato.wait_with_output()?;
    if !output.status.success() {
        return Err(format!("neato command failed with exit code: {}", 
            output.status.code().unwrap_or(-1)).into());
    }
    std::fs::write(&output_path, output.stdout)?;
    info!("Neato processing took: {:?}", neato_start.elapsed());
    
    // Process detection webs
    // graph.set_outputs(vec![132, 131, 94, 125, 169, 97, 170]);
    // graph.set_inputs(vec![19, 20, 21, 45, 46, 47, 48]);
    // This should no longer be needed
    
    let web_detection_start = Instant::now();
    let webs = get_detection_webs(&mut graph);
    info!("get_detection_webs took: {:?}", web_detection_start.elapsed());
    info!("Found {} detection webs", webs.len());
    
    let web_vis_start = Instant::now();
    let temp_dot_files = Arc::new(Mutex::new(Vec::new()));
    let graph = Arc::new(graph); // Share the graph between threads
    
    // Process webs in parallel
    let results: Vec<anyhow::Result<()>> = webs.into_par_iter().enumerate().map(|(i, web)| {
        let web_start = Instant::now();
        let web_output_path = output_dir.join(format!("web_{}.png", i + 1));
        let dot_path = output_dir.join(format!("temp_web_{}.dot", i + 1));
        
        // Add to temp files list
        temp_dot_files.lock().unwrap().push(dot_path.clone());
        
        // Generate DOT content for this specific web
        let web_dot_content = graph_visualizer::to_dot_with_positions(&*graph, Some(&web), false);
        
        // Write the DOT file
        if let Err(e) = std::fs::write(&dot_path, &web_dot_content) {
            return Err(anyhow::anyhow!("Failed to write DOT file for web {}: {}", i + 1, e));
        }
        debug!("  Web {} dot generation took: {:?}", i + 1, web_start.elapsed());
        
        // Process with neato
        let neato_start = Instant::now();
        let output = Command::new("neato")
            .args(["-n2", "-Tpng"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .and_then(|mut child| {
                if let Some(stdin) = child.stdin.as_mut() {
                    stdin.write_all(web_dot_content.as_bytes())?;
                }
                child.wait_with_output()
            });
            
        match output {
            Ok(output) if output.status.success() => {
                if let Err(e) = std::fs::write(&web_output_path, output.stdout) {
                    return Err(anyhow::anyhow!("Failed to write PNG for web {}: {}", i + 1, e));
                }
                debug!("  Web {} processing took: {:?}", i + 1, neato_start.elapsed());
                info!("  Web {} completed in: {:?}", i + 1, web_start.elapsed());
                Ok(())
            },
            Ok(output) => {
                let stderr = String::from_utf8_lossy(&output.stderr);
                Err(anyhow::anyhow!("neato failed for web {} with status {}: {}", 
                    i + 1, output.status, stderr))
            },
            Err(e) => Err(anyhow::anyhow!("Failed to execute neato for web {}: {}", i + 1, e)),
        }
    }).collect();
    
    // Check for any errors in the parallel processing
    for result in results {
        if let Err(e) = result {
            error!("Error processing web: {}", e);
            // Continue processing other webs but return an error at the end
        }
    }
    info!("All webs visualization took: {:?}", web_vis_start.elapsed());
    
    // Clean up temporary DOT files
    let cleanup_start = Instant::now();
    let temp_files = temp_dot_files.lock().unwrap();
    let cleanup_errors = temp_files.par_iter()
        .filter(|dot_file| std::fs::remove_file(dot_file).is_err())
        .count();
    
    if cleanup_errors > 0 {
        log::warn!("Failed to remove {} temporary files", cleanup_errors);
    } else if !temp_files.is_empty() {
        debug!("Cleaned up {} temporary DOT files in {:?}", temp_files.len(), cleanup_start.elapsed());
    }
    
    info!("Total execution time: {:?}", total_start.elapsed());
    Ok(())
}
