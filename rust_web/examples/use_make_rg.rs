use rust_web::graph_loader::load_graph;
use rust_web::make_rg::make_rg;
use rust_web::graph_visualizer::graph_to_png;
use quizx::hash_graph::Graph;

fn main()-> Result<(), Box<dyn std::error::Error>>{
    // Load the Steane code graph
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let graph_path = std::path::Path::new(&manifest_dir)
        .join("tests")
        .join("zxgs")
        .join("steane_style_steane_2_rounds.zxg");
    println!("Loading graph from: {}", graph_path.display());
    let graph = load_graph(graph_path.to_str().unwrap())?;
    let name = "steane_style_steane_2_rounds".to_owned();
    
    graph_to_png(
        &graph,
        &(name.clone()+".dot"),
        &(name.clone()+".png"),
        None,true)?;
    println!("Made it to after drawing first one");
    let rg_graph: Graph = make_rg(graph);
    println!("At least got graph before saving");
    graph_to_png(
        &rg_graph,
         &(name.clone()+"_rg.dot"),
          &(name.clone()+"_rg.png"),
           None, true
        )?;
        Ok(())

}