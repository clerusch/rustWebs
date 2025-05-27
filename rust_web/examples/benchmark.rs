// use env_logger;
// use log::{info, error};
// use std::env;
// use std::path::Path;

// fn main() {
//     // Initialize logger
//     env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    
//     // Get the input file path from command line arguments or use a default
//     let args: Vec<String> = env::args().collect();
//     let path = if args.len() > 1 {
//         args[1].clone()
//     } else {
//         // Default test file path - adjust this to your test file
//         "tests/zxgs/2_rounds_steane.zxg".to_string()
//     };
    
//     // Check if file exists
//     if !Path::new(&path).exists() {
//         error!("Input file not found: {}", path);
//         error!("Current working directory: {:?}", std::env::current_dir().unwrap_or_default());
//         std::process::exit(1);
//     }
    
//     info!("Starting benchmark for: {}", path);
    
//     // Run the detection web generation
//     if let Err(e) = use_detection_webs::use_det_web(&path) {
//         error!("Error: {}", e);
//         std::process::exit(1);
//     }
// }
fn main(){}