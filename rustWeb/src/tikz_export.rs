use quizx::hash_graph::*;
use std::fs::File;
use std::io::Write;

pub fn export_to_tikz(g: &Graph, filename: &str) {
    let mut file = File::create(filename).expect("Unable to create file");

    writeln!(file, "\\documentclass{{standalone}}").unwrap();
    writeln!(file, "\\usepackage{{tikz}}").unwrap();
    writeln!(file, "\\begin{{document}}").unwrap();
    writeln!(file, "\\begin{{tikzpicture}}[scale=1]").unwrap();

    let mut positions = vec![];

    // Assign simple horizontal positions to each vertex
    for (i, v) in g.vertices().enumerate() {
        let x = i as f64 * 1.5; // horizontal spacing
        let label = format!("v{}", v);
        let node_type = format!("{:?}", g.vertex_type(v));
        writeln!(
            file,
            "\\node[draw,circle,fill=gray!20] (v{}) at ({},0) {{{}}};",
            v, x, node_type
        ).unwrap();
        positions.push((v, x));
    }

    // Draw edges
    for (v0, v1, _) in g.edges() {
        writeln!(file, "\\draw (v{}) -- (v{});", v0, v1).unwrap();
    }

    writeln!(file, "\\end{{tikzpicture}}").unwrap();
    writeln!(file, "\\end{{document}}").unwrap();
}
