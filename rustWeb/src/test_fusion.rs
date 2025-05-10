use quizx::hash_graph::*;
use quizx::basic_rules::*;

pub fn create_chain(n: i32)->Graph {
    let mut g = Graph::new();

    // Create the first node
    let first = g.add_vertex(VType::X);
    let mut prev = first;

    // Add 999 more nodes and chain them
    for _ in 0..n {
        let current = g.add_vertex(VType::X);
        g.add_edge(prev, current);
        prev = current;
    }
    return g;
}

pub fn compress_graph(mut g:Graph)->Graph {
    loop {
        match g.find_edge(|v0, v1, _| check_spider_fusion(&g, v0, v1)) {
            Some((v0, v1, _)) => spider_fusion_unchecked(&mut g, v0, v1),
            None => break,
        }
    }
    return g
}

pub(super) fn test_fusion() {
    let g: Graph = create_chain(99999);
    println!("Graph has {} vertices", g.num_vertices());

    // Optional: Apply spider fusion rule
    let g = compress_graph(g);
    println!("Graph has {} vertices", g.num_vertices());

}