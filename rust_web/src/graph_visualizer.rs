use std::fs;
use std::process::Command;
use std::collections::HashMap;
use num::{Rational64, FromPrimitive};
use quizx::graph::GraphLike;
use crate::pauliweb::PauliWeb;
use ordered_float::OrderedFloat;

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

pub fn to_dot_with_positions<G: GraphLike>(
    graph: &G, 
    pauli_web: Option<&PauliWeb>,
    show_node_ids: bool
) -> String {
    let mut result = String::new();
    result.push_str("graph G {\n");
    // Set graph properties for better layout
    result.push_str("  graph [splines=true, overlap=false, pad=\"0.5\", nodesep=\"0.5\", ranksep=\"1.0\"];\n");
    
    // Set default node attributes for consistent sizing and appearance
    result.push_str("  node [style=\"filled\", shape=\"circle\", width=\"0.6\", height=\"0.6\", fixedsize=\"true\", \n");
    result.push_str("       fontsize=\"24\", fontname=\"Arial\", penwidth=\"1.5\", labelloc=\"c\"];\n");
    result.push_str("  node [fontname=\"Arial\"];\n");  // Set default font for all text elements
    
    // Set default edge style
    result.push_str("  edge [penwidth=2.0, color=\"#666666\"];\n");  // Default edge color is gray

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
        let (fill_color, border_color, shape, label, font_color) = match data.ty {
            quizx::graph::VType::Z => {
                let phase_str = format_phase(data.phase.to_f64());
                let label = if phase_str.is_empty() {
                    if show_node_ids { v.to_string() } else { String::new() }
                } else {
                    phase_str
                };
                ("#88ff88", "#000000", "circle", label, "#000000")  // Brighter green fill, black border
            },
            quizx::graph::VType::X => {
                let phase_str = format_phase(data.phase.to_f64());
                let label = if phase_str.is_empty() {
                    if show_node_ids { v.to_string() } else { String::new() }
                } else {
                    phase_str
                };
                ("#ff8888", "#000000", "circle", label, "#000000")  // Brighter red fill, black border
            },
            quizx::graph::VType::H => {
                ("#ffff88", "#000000", "square", String::new(), "#000000")  // Brighter yellow fill, black border
            },
            quizx::graph::VType::B => {
                ("#000000", "#000000", "circle", String::from("B"), "#ffffff")  // Black box with white text
            },
            _ => {
                ("#ffffff", "#000000", "circle", String::new(), "#000000")  // Default white circle
            },
        };

        let x = (data.row * time_spacing).round() as i32;
        let y = ((data.qubit - min_qubit) * grid_spacing).round() as i32;
        let pos = format!("{},{}!", x, y);
        
        // Create HTML-like label with ID above and phase inside
        let html_label = if show_node_ids || !label.is_empty() {
            let id_part = if show_node_ids {
                // Escape special characters in node ID
                let escaped_id = v.to_string()
                    .replace('&', "&amp;")
                    .replace('<', "&lt;")
                    .replace('>', "&gt;")
                    .replace('"', "&quot;");
                format!("<font point-size='12'>{}</font><br/>", escaped_id)
            } else {
                String::new()
            };
            let phase_part = if !label.is_empty() {
                // Escape special characters in phase label
                let escaped_label = label
                    .replace('&', "&amp;")
                    .replace('<', "&lt;")
                    .replace('>', "&gt;")
                    .replace('"', "&quot;");
                format!("<font point-size='16'>{}</font>", escaped_label)
            } else {
                String::new()
            };
            format!("label=<<table border='0' cellborder='0' cellspacing='0' cellpadding='0'>\
                   <tr><td align='center'>{}{}</td></tr></table>>", id_part, phase_part)
        } else {
            String::new()
        };
        
        let mut node_attrs = Vec::new();
        if !html_label.is_empty() {
            node_attrs.push(html_label);
        }

        let mut attrs = vec![
            format!("pos=\"{}\"", pos),
            format!("shape=\"{}\"", shape),
            format!("fillcolor=\"{}\"", fill_color),
            format!("color=\"{}\"", border_color),
            "style=\"filled,solid\"".to_string(),
            "width=0.6".to_string(),
            "height=0.6".to_string(),
            "fixedsize=true".to_string(),
            format!("fontcolor=\"{}\"", font_color),
            "labelloc=\"c\"".to_string(),  // Center the label inside the node
        ];
        
        // Add all node attributes
        attrs.extend(node_attrs);
        
        // Make H nodes slightly larger
        if data.ty == quizx::graph::VType::H {
            attrs.push("shape=square".to_string());
            attrs.push("margin=0.1".to_string());
        }
        
        if data.ty == quizx::graph::VType::H {
            // Make H-boxes square and slightly larger
            attrs.push("width=0.4".to_string());
            attrs.push("height=0.4".to_string());
        }
        
        // Ensure node ID is properly quoted if it contains special characters
        let node_id = if v.to_string().chars().any(|c| !c.is_ascii_alphanumeric() && c != '_') {
            format!("\"{}\"", v)
        } else {
            v.to_string()
        };
        vertices.push(format!("  {} [{}]", node_id, attrs.join(",")));
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
                // Default edge style (black)
                let mut edge_attrs = vec![
                    "len=1.0".to_string(),
                    "penwidth=1.5".to_string(),
                    "color=\"#000000\"".to_string(),
                    "style=solid".to_string()
                ];
                
                // Custom styling for Pauli web edges
                if let Some(pauli_web) = pauli_web {
                    if let Some(pauli) = pauli_web.get_edge(v.into(), n.into()) {
                        let (color, penwidth) = match pauli {
                            crate::pauliweb::Pauli::X => ("#ff0000", "2.5"),  // Red for X
                            crate::pauliweb::Pauli::Z => ("#00aa00", "2.5"),  // Green for Z
                            _ => ("#0000ff", "2.0"),                         // Blue for others
                        };
                        
                        // Update edge attributes for Pauli web edges
                        edge_attrs = vec![
                            "len=1.0".to_string(),
                            format!("penwidth={}", penwidth),
                            format!("color=\"{}\"", color),
                            "style=bold".to_string()
                        ];
                    }
                }
                
                // Add the edge with final attributes
                result.push_str(&format!("  {} -- {} [{}]\n", v, n, edge_attrs.join(",")));
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

/// Draw a graph with Pauli web overlaid and save to file
/// 
/// # Arguments
/// * `graph` - The graph to draw
/// * `pauli_web` - The Pauli web to overlay on the graph
/// * `output_path` - Path to save the output SVG file
/// 
/// # Returns
/// * `Result<(), String>` - Ok if successful, Err with error message otherwise
pub fn draw_graph_with_pauliweb<G: GraphLike>(
    graph: &G,
    pauli_web: &PauliWeb,
    output_path: &str,
) -> Result<(), String> {
    // Create a temporary DOT file
    let dot_path = format!("{}.dot", output_path);
    let dot_content = to_dot_with_positions(graph, Some(pauli_web), false);
    
    // Write DOT content to file
    std::fs::write(&dot_path, dot_content)
        .map_err(|e| format!("Failed to write DOT file: {}", e))?;
    
    // Run Graphviz to generate SVG
    let output = Command::new("dot")
        .arg("-Tsvg")
        .arg(&dot_path)
        .output()
        .map_err(|e| format!("Failed to execute dot command: {}", e))?;
    
    if !output.status.success() {
        return Err(format!(
            "Graphviz failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    
    // Write SVG to output file
    std::fs::write(output_path, &output.stdout)
        .map_err(|e| format!("Failed to write SVG file: {}", e))?;
    
    // Clean up temporary DOT file
    let _ = std::fs::remove_file(dot_path);
    
    Ok(())
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
