use std::fs;
use std::process::Command;
use std::collections::HashMap;
use num::{Rational64, FromPrimitive};
use quizx::graph::GraphLike;

// Helper function to format phase values with fractional notation when possible
fn format_phase(phase: f64) -> String {
    if phase == 0.0 {
        return String::new();
    }
    
    // Try to convert to a simple fraction
    let rat = Rational64::from_f64(phase).unwrap_or_else(|| Rational64::from_f64(phase * 10.0).unwrap() / 10);
    let numer = rat.numer().abs();
    let denom = rat.denom();
    
    // Check for common fractions (with denominator <= 4)
    let fraction = match (numer, denom) {
        (1, 1) => "π".to_string(),
        (1, 2) => "π/2".to_string(),
        (1, 3) => "π/3".to_string(),
        (1, 4) => "π/4".to_string(),
        (2, 3) => "2π/3".to_string(),
        (3, 2) => "3π/2".to_string(),
        (3, 4) => "3π/4".to_string(),
        _ => {
            // For other values, round to 1 decimal place
            let rounded = (phase * 10.0).round() / 10.0;
            if rounded.fract() == 0.0 {
                format!("{}π", rounded as i32)
            } else {
                format!("{}π", rounded)
            }
        }
    };
    
    // Add negative sign if needed
    if phase < 0.0 {
        format!("-{fraction}")
    } else {
        fraction
    }
}
use crate::pauliweb::PauliWeb;
use ordered_float::OrderedFloat;

pub fn to_dot_with_positions<G: GraphLike>(
    graph: &G, 
    pauli_web: Option<&PauliWeb>,
    show_node_ids: bool
) -> String {
    let mut result = String::new();
    result.push_str("graph G {\n");
    // Set default node attributes for consistent sizing
    result.push_str("  node [style=\"filled\", shape=\"circle\", width=\"0.3\", height=\"0.3\", fixedsize=\"true\", fontsize=\"12\", fontname=\"Arial\"];\n");
    result.push_str("  edge [penwidth=2.0];\n");  // Make edges thicker for better visibility

    // Calculate positions and collect vertex info
    let mut vertices = Vec::new();
    let mut qubits: HashMap<OrderedFloat<f64>, _> = HashMap::new();
    let mut min_qubit = f64::MAX;
    let mut max_qubit = f64::MIN;
    let mut max_time = 0.0;

    for v in graph.vertices() {
        let data = graph.vertex_data(v);
        qubits.entry(OrderedFloat(data.qubit)).or_insert_with(Vec::new).push((v, data.row));
        min_qubit = f64::min(min_qubit, data.qubit);
        max_qubit = f64::max(max_qubit, data.qubit);
        max_time = f64::max(max_time, data.row);
    }

    let grid_spacing = 100.0;
    let time_spacing = grid_spacing * 1.5;

    // Add vertices
    for v in graph.vertices() {
        let data = graph.vertex_data(v);
        let (color, shape, label) = match data.ty {
            quizx::graph::VType::Z => {
                let phase_str = format_phase(data.phase.to_f64());
                let label = if show_node_ids {
                    if phase_str.is_empty() {
                        format!("{}", v)
                    } else {
                        format!("{}\n({})", phase_str, v)
                    }
                } else {
                    phase_str
                };
                ("green", "circle", label)
            },
            quizx::graph::VType::X => {
                let phase_str = format_phase(data.phase.to_f64());
                let label = if show_node_ids {
                    if phase_str.is_empty() {
                        format!("{}", v)
                    } else {
                        format!("{}\n({})", phase_str, v)
                    }
                } else {
                    phase_str
                };
                ("red", "circle", label)
            },
            quizx::graph::VType::H => ("yellow", "box", String::new()),
            quizx::graph::VType::B => ("black", "circle", "B".to_string()),
            _ => ("white", "circle", String::new()),
        };

        let x = (data.row * time_spacing).round() as i32;
        let y = ((data.qubit - min_qubit) * grid_spacing).round() as i32;
        let pos = format!("{},{}!", x, y);
        
        vertices.push(format!(
            "  {} [color=\"{}\", shape=\"{}\", label=\"{}\", pos=\"{}\", style=\"filled\", width=\"0.3\", height=\"0.3\", fixedsize=\"true\"]",
            v, color, shape, label, pos
        ));
    }

    // Add vertices to the DOT string
    for vertex in vertices {
        result.push_str(&vertex);
        result.push_str("\n");
    }

    // Add edges with colors based on PauliWeb if provided
    for v in graph.vertices() {
        for n in graph.neighbors(v) {
            if v < n {  // Only add each edge once
                let edge_style = if let Some(pauli_web) = pauli_web {
                    if let Some(color) = pauli_web.get_edge_color(v.into(), n.into()) {
                        format!(" [color={}]", color)
                    } else {
                        "".to_string()
                    }
                } else {
                    "".to_string()
                };
                
                result.push_str(&format!("  {} -- {}{}\n", v, n, edge_style));
            }
        }
    }

    result.push_str("}\n");
    result
}

pub fn graph_to_png<G: GraphLike>(
    graph: &G, 
    dot_path: &str, 
    png_path: &str,
    pauli_web: Option<&PauliWeb>,
    show_node_ids: bool
) -> std::io::Result<()> {
    // Create output directory if it doesn't exist
    if let Some(parent) = std::path::Path::new(png_path).parent() {
        std::fs::create_dir_all(parent)?;
    }
    
    // Generate DOT string with optional PauliWeb coloring and node IDs
    let dot_string = to_dot_with_positions(graph, pauli_web, show_node_ids);
    
    // Write DOT file
    fs::write(dot_path, dot_string)?;

    // Call neato to generate PNG
    let status = Command::new("neato")
        .args(&["-n2", "-Tpng", dot_path, "-o", png_path])
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "neato command failed",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quizx::{hash_graph::Graph, phase::Phase};
    use crate::pauliweb::{Pauli, PauliWeb};
    use std::convert::TryInto;
    
    #[test]
    fn test_draw_graph_simple() -> std::io::Result<()> {
        let mut graph = Graph::new();
        let v1 = graph.add_vertex_with_phase(quizx::graph::VType::Z, Phase::from(0.0));
        let v2 = graph.add_vertex_with_phase(quizx::graph::VType::X, Phase::from(1.0));
        let v3 = graph.add_vertex_with_phase(quizx::graph::VType::Z, Phase::from(0.0));
        graph.add_edge(v1, v2);
        graph.add_edge(v2, v3);

        // Create output directory
        std::fs::create_dir_all("tests/output")?;

        // Test with node IDs
        graph_to_png(
            &graph,
            "tests/output/simple_graph_with_ids.dot",
            "tests/output/simple_graph_with_ids.png",
            None,
            true
        )?;
        
        // Test without node IDs
        graph_to_png(
            &graph,
            "tests/output/simple_graph_no_ids.dot",
            "tests/output/simple_graph_no_ids.png",
            None,
            false
        )?;
        
        // Verify the DOT strings
        let dot_string_no_ids = to_dot_with_positions(&graph, None, false);
        
        // Check that node IDs don't appear in the label
        assert!(
            !dot_string_no_ids.contains(&format!("\n({})\n", v1)) && 
            !dot_string_no_ids.contains(&format!("label=\"{}\\n({})\"", "", v1)) &&
            !dot_string_no_ids.contains(&format!("label=\"{}\\n({})\"", "0", v1)),
            "Node ID {} should not be in the output. Full output:\n\n{}", v1, dot_string_no_ids
        );
        
        let dot_string_with_ids = to_dot_with_positions(&graph, None, true);
        
        // Check that node IDs appear as the label
        // The format is: label="0" for node 0, or "π\n(1)" for node 1 with phase π
        assert!(
            dot_string_with_ids.contains(&format!("label=\"{}\"", v1)) ||
            dot_string_with_ids.contains(&format!("label=\"0\"")),
            "Node ID {} not found in DOT output. Full output:\n\n{}", v1, dot_string_with_ids
        );
        
        // For v2, we expect "π\n(1)" as the label
        assert!(
            dot_string_with_ids.contains(&format!("label=\"π\
({})\"", v2)) ||
            dot_string_with_ids.contains("label=\"π\n(1)\""),
            "Node ID {} not found in DOT output. Full output:\n\n{}", v2, dot_string_with_ids
        );
        
        // For v3, we expect the node ID as the label
        assert!(
            dot_string_with_ids.contains(&format!("label=\"{}\"", v3)) ||
            dot_string_with_ids.contains("label=\"2\""),
            "Node ID {} not found in DOT output. Full output:\n\n{}", v3, dot_string_with_ids
        );
        
        Ok(())
    }

    #[test]
    fn test_draw_graph_with_pauliweb() -> std::io::Result<()> {
        let mut g = Graph::new();
        let v1 = g.add_vertex_with_phase(quizx::graph::VType::Z, Phase::from(0.0));
        let v2 = g.add_vertex_with_phase(quizx::graph::VType::Z, Phase::from(0.0));
        let v3 = g.add_vertex_with_phase(quizx::graph::VType::Z, Phase::from(0.0));
        g.add_edge(v1, v2);
        g.add_edge(v2, v3);

        // Create a simple PauliWeb for testing
        let mut pauli_web = PauliWeb::new();
        pauli_web.set_edge(v1.try_into().unwrap(), v2.try_into().unwrap(), Pauli::X);
        pauli_web.set_edge(v2.try_into().unwrap(), v3.try_into().unwrap(), Pauli::Z);

        // Create output directory
        std::fs::create_dir_all("tests/output")?;

        // Test with node IDs
        graph_to_png(
            &g, 
            "tests/output/pauli_web_graph_with_ids.dot", 
            "tests/output/pauli_web_graph_with_ids.png", 
            Some(&pauli_web),
            true
        )?;
        
        // Test without node IDs
        graph_to_png(
            &g, 
            "tests/output/pauli_web_graph_no_ids.dot", 
            "tests/output/pauli_web_graph_no_ids.png", 
            Some(&pauli_web),
            false
        )?;
        
        // Verify the DOT strings
        let dot_string_no_ids = to_dot_with_positions(&g, Some(&pauli_web), false);
        
        // Check that node IDs don't appear in the label
        assert!(
            !dot_string_no_ids.contains(&format!("\n({})\n", v1)) && 
            !dot_string_no_ids.contains(&format!("label=\"{}\\n({})\"", "", v1)) &&
            !dot_string_no_ids.contains(&format!("label=\"{}\\n({})\"", "0", v1)),
            "Node ID {} should not be in the output. Full output:\n\n{}", v1, dot_string_no_ids
        );
        
        let dot_string_with_ids = to_dot_with_positions(&g, Some(&pauli_web), true);
        
        // Check that node IDs appear as the label
        // The format is: label="0" for node 0
        assert!(
            dot_string_with_ids.contains(&format!("label=\"{}\"", v1)) ||
            dot_string_with_ids.contains("label=\"0\""),
            "Node ID {} not found in DOT output. Full output:\n\n{}", v1, dot_string_with_ids
        );
        
        // Check for red color in the output (in node or edge attributes)
        assert!(
            dot_string_with_ids.contains("color=red") ||
            dot_string_with_ids.contains("color=\"red\"") ||
            dot_string_with_ids.contains("0 -- 1 [color=red]") ||
            dot_string_with_ids.contains("0 -- 1 [color=\"red\"]"),
            "Red color not found in DOT output. Full output:\n\n{}", 
            dot_string_with_ids
        );
        
        // Check for green color in the output (in node or edge attributes)
        assert!(
            dot_string_with_ids.contains("color=green") ||
            dot_string_with_ids.contains("color=\"green\"") ||
            dot_string_with_ids.contains("1 -- 2 [color=green]") ||
            dot_string_with_ids.contains("1 -- 2 [color=\"green\"]"),
            "Green color not found in DOT output. Full output:\n\n{}",
            dot_string_with_ids
        );
        
        // Check that node 0 is green
        assert!(
            dot_string_with_ids.contains("0 [color=green") ||
            dot_string_with_ids.contains("0 [color=\"green\""),
            "Node 0 should be green. Full output:\n\n{}",
            dot_string_with_ids
        );
        
        // Check that node 1 is green
        assert!(
            dot_string_with_ids.contains("1 [color=green") ||
            dot_string_with_ids.contains("1 [color=\"green\""),
            "Node 1 should be green. Full output:\n\n{}",
            dot_string_with_ids
        );
        
        // Check that node 2 is green
        assert!(
            dot_string_with_ids.contains("2 [color=green") ||
            dot_string_with_ids.contains("2 [color=\"green\""),
            "Node 2 should be green. Full output:\n\n{}",
            dot_string_with_ids
        );
        
        Ok(())
    }
}
