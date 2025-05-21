use quizx::hash_graph::Graph;
use quizx::phase::Phase;
use quizx::fscalar::*;
use quizx::graph::{VType, VData};
use serde_json::Value;
use quizx::hash_graph::GraphLike;
use std::collections::{HashMap, HashSet};
use std::fs;

#[allow(dead_code)] // Remove once used
pub fn load_graph(path: &str) -> Graph {
    // Load JSON file
    let data: Value = serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();

    let mut graph = Graph::new();
    let mut id_map = HashMap::new();
    let mut xcods = HashSet::new();
    let mut ycods = HashSet::new();

    // Collect coordinates
    for (_node, dets) in data["wire_vertices"].as_object().unwrap() {
        let coord = dets["annotation"]["coord"].as_array().unwrap();
        xcods.insert(coord[0].as_i64().unwrap());
        ycods.insert(coord[1].as_i64().unwrap());
    }
    for (_node, dets) in data["node_vertices"].as_object().unwrap() {
        let coord = dets["annotation"]["coord"].as_array().unwrap();
        xcods.insert(coord[0].as_i64().unwrap());
        ycods.insert(coord[1].as_i64().unwrap());
    }

    let mut x_list: Vec<_> = xcods.into_iter().collect();
    let mut y_list: Vec<_> = ycods.into_iter().collect();
    x_list.sort();
    y_list.sort();

    let x_cood_map: HashMap<_, _> = x_list.iter().enumerate().map(|(n, &x)| (x, n)).collect();
    let y_cood_map: HashMap<_, _> = y_list.iter().enumerate().map(|(n, &y)| (y, n)).collect();

    // Boundary vertices
    for (node, dets) in data["wire_vertices"].as_object().unwrap() {
        let coord = dets["annotation"]["coord"].as_array().unwrap();
        let row: f64 = x_cood_map[&coord[0].as_i64().unwrap()] as f64;
        let qubit: f64 = y_cood_map[&coord[1].as_i64().unwrap()] as f64;
        let data: VData = VData {
            ty: VType::Z,
            phase: Phase::zero(),
            qubit: qubit,
            row: row,
        };
        let vid = graph.add_vertex_with_data(data); // or whatever type is appropriate
        id_map.insert(node.clone(), vid);
        // quizx does not have set_vdata, so you may need to store labels elsewhere if needed
    }

    // Actual vertices
    for (node, dets) in data["node_vertices"].as_object().unwrap() {
        let coord = dets["annotation"]["coord"].as_array().unwrap();
        let row = x_cood_map[&coord[0].as_i64().unwrap()];
        let qubit = y_cood_map[&coord[1].as_i64().unwrap()];
        let v_val = dets["data"]["value"].as_f64().unwrap_or(0.0);
        let v_type = match dets["data"]["type"].as_str().unwrap() {
            "X" => VType::X,
            "Z" => VType::Z,
            _ => VType::H,
        };
        let data: VData = VData {
            ty: v_type,
            row: row as f64,
            qubit: qubit as f64,
            phase: Phase::from_f64(v_val),
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

    graph
} 