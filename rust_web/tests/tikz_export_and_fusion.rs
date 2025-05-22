use quizx::hash_graph::*;
use quizx::basic_rules::*;
use rust_web::create_graph::*;
use quizx::graph::VType::{X,Z};
use rust_web::tikz_export::export_to_tikz;


pub fn compress_graph(mut g:Graph)->Graph {
    loop {
        match g.find_edge(|v0, v1, _| check_spider_fusion(&g, v0, v1)) {
            Some((v0, v1, _)) => spider_fusion_unchecked(&mut g, v0, v1),
            None => break,
        }
    }
    return g
}

#[test]
fn compression_simple() {
    let g = create_chain(99999);
    let g = compress_graph(g);
    assert!(g.num_vertices() > 0);
}

#[test]
pub fn compression_spider() -> Result<(), std::io::Error> {
    let gx: Graph = create_spider_chain(10, X, false,true);
    let gz: Graph = create_spider_chain(10, Z, false,true);
    export_to_tikz(&gx, "./target/debug/examples/gx.tex")?;
    export_to_tikz(&gz, "./target/debug/examples/gz.tex")?;
    Ok(())
}