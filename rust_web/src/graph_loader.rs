use quizx::hash_graph::Graph;
use quizx::phase::Phase;
use quizx::graph::{VType, VData};
use serde_json::Value;
use quizx::hash_graph::GraphLike;
use std::collections::{HashMap, HashSet};
use std::fs;

#[allow(dead_code)] // Remove once used
pub fn load_graph(path: &str) -> Result<Graph, String> {
    // Load as JSON file
    let file_content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => return Err(format!("Failed to read file: {}", e)),
    };
    
    let data: Value = match serde_json::from_str(&file_content) {
        Ok(json) => json,
        Err(e) => return Err(format!("Failed to parse JSON: {}", e)),
    };

    // Verify required JSON structure
    let wire_vertices = data["wire_vertices"].as_object().ok_or("Missing or invalid wire_vertices")?;
    let node_vertices = data["node_vertices"].as_object().ok_or("Missing or invalid node_vertices")?;
    let _undir_edges = data["undir_edges"].as_object().ok_or("Missing or invalid undir_edges")?;

    let mut xcods: HashSet<i64> = HashSet::new();
    let mut ycods: HashSet<i64> = HashSet::new();

    // Collect coordinates from wire vertices
    for (_node, dets) in wire_vertices {
        let coord = match dets["annotation"].get("coord") {
            Some(coord) => coord.as_array().ok_or("Invalid coordinate format")?,
            None => {
                // Handle boundary vertices with boundary field
                let boundary = dets["annotation"]["boundary"].as_bool().ok_or("Invalid boundary field")?;
                if !boundary {
                    return Err("Invalid boundary vertex format".to_string());
                }
                continue;
            }
        };
        let x = (coord[0].as_f64().ok_or("Invalid x coordinate (not a number)")? * 1000.0) as i64;
        let y = (coord[1].as_f64().ok_or("Invalid y coordinate (not a number)")? * 1000.0) as i64;
        xcods.insert(x);
        ycods.insert(y);
    }

    // Collect coordinates from node vertices
    for (_node, dets) in node_vertices {
        let coord = dets["annotation"]["coord"].as_array().ok_or("Invalid coordinate format")?;
        let x = (coord[0].as_f64().ok_or("Invalid x coordinate (not a number)")? * 1000.0) as i64;
        let y = (coord[1].as_f64().ok_or("Invalid y coordinate (not a number)")? * 1000.0) as i64;
        xcods.insert(x);
        ycods.insert(y);
    }

    let mut graph = Graph::new();
    let mut id_map = HashMap::new();

    // Collect coordinates from wire vertices
    for (_node, dets) in wire_vertices {
        let coord = dets["annotation"]["coord"].as_array().ok_or("Invalid coordinate format")?;
        let x = (coord[0].as_f64().ok_or("Invalid x coordinate (not a number)")? * 1000.0) as i64;
        let y = (coord[1].as_f64().ok_or("Invalid y coordinate (not a number)")? * 1000.0) as i64;
        xcods.insert(x);
        ycods.insert(y);
    }

    // Collect coordinates from node vertices
    for (_node, dets) in node_vertices {
        let coord = dets["annotation"]["coord"].as_array().ok_or("Invalid coordinate format")?;
        let x = (coord[0].as_f64().ok_or("Invalid x coordinate (not a number)")? * 1000.0) as i64;
        let y = (coord[1].as_f64().ok_or("Invalid y coordinate (not a number)")? * 1000.0) as i64;
        xcods.insert(x);
        ycods.insert(y);
    }

    let mut x_list: Vec<_> = xcods.iter().cloned().collect();
    let mut y_list: Vec<_> = ycods.iter().cloned().collect();
    x_list.sort();
    y_list.sort();

    let x_cood_map: HashMap<i64, usize> = x_list.iter().enumerate().map(|(n, &x)| (x, n)).collect();
    let y_cood_map: HashMap<i64, usize> = y_list.iter().enumerate().map(|(n, &y)| (y, n)).collect();

    let x_cood_map_f64: HashMap<i64, f64> = x_list.iter().enumerate().map(|(_n, &x)| (x, x as f64 / 1000.0)).collect();
    let y_cood_map_f64: HashMap<i64, f64> = y_list.iter().enumerate().map(|(_n, &y)| (y, y as f64 / 1000.0)).collect();

    // Boundary vertices
    for (node, dets) in data["wire_vertices"].as_object().unwrap() {
        let coord = dets["annotation"]["coord"].as_array().unwrap();
        let row = coord[0].as_f64().unwrap();
        let qubit = coord[1].as_f64().unwrap();
        let v_val = dets["data"]["value"].as_f64().unwrap_or(0.0);
        let data: VData = VData {
            ty: VType::B,
            phase: Phase::from_f64(v_val),
            qubit: qubit,
            row: row,
        };
        let vid = graph.add_vertex_with_data(data);
        id_map.insert(node.clone(), vid);
    }

    // Actual vertices
    for (node, dets) in data["node_vertices"].as_object().unwrap() {
        let coord = dets["annotation"]["coord"].as_array().unwrap();
        let x = (coord[0].as_f64().unwrap() * 1000.0) as i64;
        let y = (coord[1].as_f64().unwrap() * 1000.0) as i64;
        let _row = x_cood_map[&x];
        let _qubit = y_cood_map[&y];
        let v_val = dets["data"]["value"].as_f64().unwrap_or(0.0);
        let v_type = match dets["data"]["type"].as_str().unwrap() {
            "X" => VType::X,
            "Z" => VType::Z,
            _ => VType::H,
        };
        let data: VData = VData {
            ty: v_type,
            phase: Phase::from_f64(v_val),
            qubit: y_cood_map_f64[&y],
            row: x_cood_map_f64[&x],
        };
        let vid = graph.add_vertex_with_data(data);
        id_map.insert(node.clone(), vid);
    }

    // Edges
    for (_edge, dets) in data["undir_edges"].as_object().unwrap() {
        let src = dets["src"].as_str().unwrap();
        let tgt = dets["tgt"].as_str().unwrap();
        let src_id = id_map[src];
        let tgt_id = id_map[tgt];
        graph.add_edge(src_id, tgt_id);//, ety); for now lets just do simple edges
    }

    Ok(graph)
} 