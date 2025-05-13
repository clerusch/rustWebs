use quizx::hash_graph::*;
use std::fs::File;
use std::io::{Write, Result};

/// Exports a graph to a TikZ file for LaTeX visualization
pub fn export_to_tikz(g: &Graph, filename: &str) -> Result<()> {
    let mut file = File::create(filename)?;

    writeln!(file, "\\documentclass{{standalone}}")?;
    writeln!(file, "\\usepackage{{tikz}}")?;
    writeln!(file, "\\begin{{document}}")?;
    writeln!(file, "\\begin{{tikzpicture}}[scale=1]")?;

    // Define styles for X, Z and boundary spiders
    writeln!(file, "\\tikzstyle{{xspider}}=[draw,circle,fill=red!20]")?;
    writeln!(file, "\\tikzstyle{{zspider}}=[draw,circle,fill=green!20]")?;
    writeln!(file, "\\tikzstyle{{boundary}}=[draw,circle,fill=black!20]")?;

    let mut positions = vec![];

    // Assign simple horizontal positions to each vertex
    for (i, v) in g.vertices().enumerate() {
        let x = i as f64 * 1.5; // horizontal spacing
        let (style, label) = match g.vertex_type(v) {
            VType::X => {
                let phase = g.phase(v);
                let phase_str = if phase.to_string() == "0" {
                    String::from("")
                } else {
                    format!("{}π", phase)
                };
                ("xspider", phase_str)
            },
            VType::Z => {
                let phase = g.phase(v);
                let phase_str = if phase.to_string() == "0" {
                    String::from("")
                } else {
                    format!("{}π", phase)
                };
                ("zspider", phase_str)
            },
            _ => ("boundary", String::from("B")),
        };
        
        writeln!(
            file,
            "\\node[{}] (v{}) at ({},0) {{{}}};",
            style, v, x, label
        )?;
        positions.push((v, x));
    }

    // Draw edges
    for (v0, v1, _) in g.edges() {
        writeln!(file, "\\draw (v{}) -- (v{});", v0, v1)?;
    }

    writeln!(file, "\\end{{tikzpicture}}")?;
    writeln!(file, "\\end{{document}}")?;
    
    Ok(())
}
