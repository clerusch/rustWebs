use env_logger;
use log::{info, error};
use std::env;
use std::path::Path;
use std::time::Instant;
use rayon::prelude::*;

// Import necessary functions from the library
use rust_web::{
    graph_loader::load_graph,
    detection_webs::get_detection_webs,
    graph_visualizer,
    make_rg::make_rg,
};
fn main() {
    // Initialize logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    
    // Get the input file path from command line arguments or use a default
    let args: Vec<String> = env::args().collect();
    let path = if args.len() > 1 {
        args[1].clone()
    } else {
        // Default test file path - adjust this to your test file
        "tests/zxgs/2_rounds_steane.zxg".to_string()
    };
    
    // Check if file exists
    if !Path::new(&path).exists() {
        error!("Input file not found: {}", path);
        error!("Current working directory: {:?}", std::env::current_dir().unwrap_or_default());
        std::process::exit(1);
    }
    
    info!("Starting benchmark for: {}", path);
    let total_start = Instant::now();
    
    // 1. Load the graph
    let load_start = Instant::now();
    let mut graph = match load_graph(&path) {
        Ok(g) => g,
        Err(e) => {
            error!("Failed to load graph: {}", e);
            std::process::exit(1);
        }
    };
    info!("Graph loaded in: {:?}", load_start.elapsed());
    
    // 2. Process the graph with make_rg
    let make_rg_start = Instant::now();
    make_rg(&mut graph);
    info!("make_rg completed in: {:?}", make_rg_start.elapsed());
    
    // 3. Set inputs and outputs
    // graph.set_outputs(vec![132, 131, 94, 125, 169, 97, 170]);
    // graph.set_inputs(vec![19, 20, 21, 45, 46, 47, 48]);
    
    // 4. Generate detection webs
    let detection_start = Instant::now();
    let webs = get_detection_webs(&mut graph);
    info!("Generated {} detection webs in: {:?}", webs.len(), detection_start.elapsed());
    
    // 5. Visualize the main graph (just for timing, discard the result)
    let vis_start = Instant::now();
    let _ = graph_visualizer::to_dot_with_positions(&graph, None, false);
    info!("Main graph visualization generated in: {:?}", vis_start.elapsed());
    
    // 6. Visualize each web in parallel (just for timing, discard the results)
    let web_vis_start = Instant::now();
    webs.par_iter().for_each(|web| {
        let _ = graph_visualizer::to_dot_with_positions(&graph, Some(web), false);
    });
    info!("All web visualizations completed in: {:?}", web_vis_start.elapsed());
    
    // 7. Process detection webs in parallel (if they can be processed independently)
    let parallel_web_start = Instant::now();
    let webs_processed: Vec<_> = webs
        .par_iter()
        .map(|web| {
            // Process each web independently here
            // This is a placeholder - replace with actual processing
            web.clone()
        })
        .collect();
    info!("Parallel web processing completed in: {:?}", parallel_web_start.elapsed());
    info!("Processed {} webs", webs_processed.len());
    
    info!("Total execution time: {:?}", total_start.elapsed());
}
