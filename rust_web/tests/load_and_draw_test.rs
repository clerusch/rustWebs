
#[test]
fn test_from_file() {
    use std::fs;

    use rust_web::graph_loader;
    use rust_web::graph_visualizer;

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let path = std::path::Path::new(&manifest_dir)
        .join("tests")
        .join("zxgs")
        .join("xxx_final.zxg");
    
    // Load the graph
    let g = graph_loader::load_graph(path.to_str().unwrap()).unwrap();
    
    // Generate and save the DOT representation with positions
    let dot_string = graph_visualizer::to_dot_with_positions(&g, None, true);
    fs::write("tests/output/load_test_graph.dot", dot_string).unwrap();
    
    // Also generate a PNG for easier viewing
    graph_visualizer::graph_to_png(
        &g,
        "tests/output/load_test_graph.dot",
        "tests/output/load_test_graph.png",
                                 None, 
        true
    ).unwrap();
}