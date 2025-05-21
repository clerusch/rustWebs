use quizx::graph::VType;
use quizx::hash_graph::GraphLike;
use std::process::Command;
use std::fs;

pub fn to_dot_with_positions<G: GraphLike>(graph: &G) -> String {
    let mut result = String::from("graph G {\n  node [shape=circle, style=filled, width=0.5, fixedsize=true];\n  graph [splines=true, overlap=false, layout=neato];\n");
    let grid_spacing = 100.0;

    // Find min/max for normalization
    let mut min_row = f64::INFINITY;
    let mut min_qubit = f64::INFINITY;
    let mut max_qubit = f64::NEG_INFINITY;
    for v in graph.vertices() {
        let data = graph.vertex_data(v);
        if data.row < min_row { min_row = data.row; }
        if data.qubit < min_qubit { min_qubit = data.qubit; }
        if data.qubit > max_qubit { max_qubit = data.qubit; }
    }

    // Collect all vertices and their positions
    let mut vertices = Vec::new();
    for v in graph.vertices() {
        let data = graph.vertex_data(v);
        let (color, shape, label) = match data.ty {
            VType::X => ("red", "circle", format!("{}", v)),
            VType::Z => ("green", "circle", format!("{}", v)),
            VType::H => ("yellow", "box",format!("{}", v)), // Hadamard: yellow box, label "H"
            _ => ("black", "circle", format!("{}", v)),
        };
        let x = ((data.row - min_row) * grid_spacing).round() as i32;
        let y = ((data.qubit - min_qubit) * grid_spacing).round() as i32;
        let pos = format!("{},{}!", x, y);
        vertices.push(format!(
            "  {} [color={}, shape={}, label=\"{}\", pos=\"{}\", style=filled]",
            v, color, shape, label, pos
        ));
    }

    // Add vertices
    for vertex in vertices {
        result.push_str(&vertex);
        result.push_str("\n");
    }

    // Add edges
    for v in graph.vertices() {
        for n in graph.neighbors(v) {
            if v < n {
                result.push_str(&format!("  {} -- {}\n", v, n));
            }
        }
    }

    result.push_str("}\n");
    result
}

pub fn graph_to_png<G: quizx::hash_graph::GraphLike>(graph: &G, dot_path: &str, png_path: &str) {
    // Write DOT file
    let dot_string = crate::graph_visualizer::to_dot_with_positions(graph);
    fs::write(dot_path, dot_string).expect("Failed to write DOT file");

    // Call neato to generate PNG
    let status = Command::new("neato")
        .args(&["-n2", "-Tpng", dot_path, "-o", png_path])
        .status()
        .expect("Failed to run neato");
    assert!(status.success(), "neato command failed");
}