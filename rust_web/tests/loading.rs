// use super::graph_loader;

// #[test]
// fn test_loading() {
//     println!("Running from: {:?}", env::current_dir()?);
    
//     // Create different test cases
//     let test_cases = vec![
//         ("z_chain_no_phase", create_spider_chain(5, Z, false, false)),
//         ("z_chain_with_phase", create_spider_chain(5, Z, true, false)),
//         ("z_chain_with_boundaries", create_spider_chain(5, Z, true, true)),
//         ("x_chain_no_phase", create_spider_chain(5, X, false, false)),
//         ("x_chain_with_phase", create_spider_chain(5, X, true, false)),
//         ("x_chain_with_boundaries", create_spider_chain(5, X, true, true)),
//     ];

//     // Export each test case
//     for (name, graph) in test_cases {
//         let dot_file = format!("target/debug/examples/{}.dot", name);
//         let tex_file = format!("target/debug/examples/{}.tex", name);
        
//         write(&dot_file, graph.to_dot())?;
//         tikz_export::export_to_tikz(&graph, &tex_file)?;
//         println!("Exported {}", name);
//     }
//     let graph = graph_loader::load_graph("zxgs/xx_stab.zxg");

// }