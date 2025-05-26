use crate::bitwisef2linalg::Mat2;
use bitvec::prelude::*;

// Constants for F2 values
use quizx::hash_graph::{Graph, GraphLike};
use crate::make_rg::make_rg;
use std::collections::HashMap;
use quizx::graph::{VType, V};
use crate::pauliweb::PauliWeb;
use crate::pauliweb::Pauli;
use std::collections::BTreeSet;

fn get_adjacency_matrix(g: &Graph) -> Mat2 {
    let vertices: Vec<V> = g.vertices().collect();
    let n = vertices.len();
    // Create a new matrix and fill it
    let mut adj = Mat2::new(n, n);
    for i in 0..n {
        for j in 0..n {
            let connected = g.connected(vertices[i], vertices[j]);
            adj.set(i, j, connected);
        }
    }
    return adj
}


fn ordered_nodes(g: &Graph) -> (Vec<usize>, HashMap<usize, usize>) {
    // This is for keeping track of original node ordering
    let original: Vec<usize> = g.vertices().collect();
    let outputs: Vec<usize> = original.iter()
        .filter(|&&v| g.vertex_type(v) == VType::WOutput || g.vertex_type(v) == VType::WInput)
        .cloned()
        .collect();
    
    let mut vertices: Vec<usize> = outputs;
    vertices.extend(original.into_iter());
    
    let index_map: HashMap<usize, usize> = vertices.iter()
        .enumerate()
        .map(|(idx, &item)| (item, idx))
        .collect();
    
    return (vertices, index_map)
}

pub fn get_pw(index_map: &HashMap<usize, usize>, v: &BitVec<usize, Lsb0>, g: &Graph) -> PauliWeb {
    let n_outs = g.inputs().len() + g.outputs().len();
    let mut red_edges = BTreeSet::new();
    let mut green_edges = BTreeSet::new();
    let mut pw = PauliWeb::new();
    log::debug!("v: {:#?}", v);
    // Process each non-zero index in the bitvector
    for (index, is_set) in v.iter().enumerate() {
        log::debug!("Bit {}: {}", index, is_set);
        if *is_set {
            let node = *index_map.get(&(index - n_outs)).expect("Node index not found in index map.");
            let node_color = g.vertex_type(node);
            log::debug!("Node {}", node);
            log::debug!("Node color {:#?}", node_color);
            // Find all edges connected to this node
            for edge in g.edges() {
                if node == edge.0 || node == edge.1 {
                    if node_color == VType::Z {
                        green_edges.insert(edge);
                    } else if node_color == VType::X {
                        red_edges.insert(edge);
                    }
                    else {
                        unreachable!("Unexpected Node color: {:?}", node_color);
                    }
                }
            }
        }
    }
    // Add edges to PauliWeb
    for e in red_edges {
        pw.set_edge(e.0, e.1, Pauli::Z);
    }
    for e in green_edges {
        pw.set_edge(e.0, e.1, Pauli::X);
    }
    
    pw
}

fn draw_mat(name: &str, mat: &Mat2) {
    log::debug!("Matrix {} ({}x{}):", name, mat.rows(), mat.cols());
    for i in 0..mat.rows() {
        let row: String = (0..mat.cols())
            .map(|j| if mat.get(i, j) { '1' } else { '0' })
            .collect();
        log::debug!("{}", row);
    }
}

pub fn get_detection_webs(g: &mut Graph)-> Vec<PauliWeb> {
    make_rg(g);
    // let outs = g.inputs().len() + g.outputs().len();
    // Count the number of boundary nodes (VType::B)
    let outs = g.vertices()
        .filter(|&v| g.vertex_type(v) == VType::B)
        .count();
    let (_vertices, index_map) = ordered_nodes(&g);
    log::debug!("outs: {}", outs);
    
    // See borghans master thesis for this part
    let big_n = get_adjacency_matrix(&g);
    draw_mat("big_n", &big_n);
    let i_n = Mat2::id(outs);
    draw_mat("i_n", &i_n);
    let zeroblock = Mat2::zeros(big_n.cols() -outs,  outs);
    draw_mat("zeroblock", &zeroblock);
    let mdl = i_n.vstack(&zeroblock);
    draw_mat("mdl",&mdl);
    let md = mdl.hstack(&big_n);
    draw_mat("md", &md);
    
    // adds a stack of single-entry rows to eliminate outputs of the graph
    let zeros =  Mat2::zeros(2*outs, big_n.rows()-2*outs);
    draw_mat("zeros",&zeros);
    let no_output = Mat2::id(2*outs).hstack(&zeros);
    draw_mat("no_output", &no_output);

    let md_no_output = md.vstack(&no_output);
    
    // Log the final matrix in a readable format
    draw_mat("md_no_output", &md_no_output);
    
    let mdnons = md_no_output.nullspace(false);
    
    let mut pws = Vec::new();
    log::debug!("mdnons: {:#?}", mdnons);
    for basis in mdnons {
        // Create a bitvector from the basis vector
        let mut vec = bitvec![0; basis.rows()];
        for i in 0..basis.rows() {
            vec.set(i, basis.get(i, 0));
        }
        log::debug!("basis vector: {:#?}", vec);
        let pw = get_pw(&index_map, &vec, &g);
        pws.push(pw);
    }
    pws
}
