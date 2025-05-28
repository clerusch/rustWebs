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

fn get_adjacency_matrix(g: &Graph, nodelist: &[V]) -> Mat2 {
    // Takes a quizx graph and returns the adjacency matrix of the graph in the order of nodelist
    let n = nodelist.len();
    let mut adj = Mat2::new(n, n);
    
    // Fill the adjacency matrix
    for (i, &u) in nodelist.iter().enumerate() {
        for (j, &v) in nodelist.iter().enumerate() {
            // Check both directions since the graph is undirected
            let connected = g.connected(u, v) || g.connected(v, u);
            adj.set(i, j, connected);
        }
    }
    
    adj
}

fn ordered_nodes(g: &Graph) -> (Vec<usize>, HashMap<usize, usize>) {
    // Get all vertices and sort them for consistent ordering
    let mut original: Vec<usize> = g.vertices().collect();
    original.sort();
    
    // First put outputs (nodes that are neither inputs nor outputs in the original graph)
    let outputs: Vec<usize> = original.iter()
        .filter(|&&v| !g.inputs().contains(&v) && !g.outputs().contains(&v))
        .cloned()
        .collect();
    
    // Then add the rest (inputs and outputs) that have type != 0 (B type is 0 in Python)
    let mut vertices = outputs.clone();
    vertices.extend(
        original.iter()
            .filter(|&&v| {
                let vtype = g.vertex_type(v);
                vtype != VType::B && !outputs.contains(&v)
            })
            .cloned()
    );
    
    // Create index map (matrix index -> original node index)
    let index_map: HashMap<usize, usize> = vertices
        .iter()
        .enumerate()
        .map(|(i, &v)| (i, v))
        .collect();
    
    log::debug!("Ordered vertices: {:?}", vertices);
    log::debug!("Index map: {:?}", index_map);
    
    (vertices, index_map)
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
            .collect::<Vec<char>>()
            .chunks(4)  // Group into chunks of 4 for better readability
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join(" ");  // Add space between chunks
        log::debug!("[{}]", row);
    }
}
/// Returns all detection webs of a quizx graph
/// Will inplace convert the graph to rg form
/// 
/// TODO: perhaps handle the input/output stuff, currently we break it and just assume thats not a set
/// property
pub fn get_detection_webs(g: &mut Graph) -> Vec<PauliWeb> {
    // First convert to RG form
    make_rg(g);

    // Lets make the whole outputs thing native:
    let mut outputs = Vec::new();
    for v in g.vertices() {
        if g.vertex_type(v) == VType::B {
            outputs.push(v);
        }
    }
    g.set_outputs(outputs);
    
    // Get number of inputs + outputs
    let outs = g.inputs().len() + g.outputs().len();
    
    // Get ordered nodes and index map
    let (nodelist, index_map) = ordered_nodes(g);
    log::debug!("Ordered nodes: {:?}", nodelist);
    log::debug!("outs: {}", outs);
    
    // Get adjacency matrix in the specified node order
    let big_n = get_adjacency_matrix(g, &nodelist);
    draw_mat("N (adjacency)", &big_n);
    
    // Create I_n (identity matrix of size outs x outs)
    let i_n = Mat2::id(outs);
    draw_mat("I_n", &i_n);
    
    // Create zero block of size (n - outs) x outs
    let zeroblock = Mat2::zeros(big_n.rows() - outs, outs);
    draw_mat("zeroblock", &zeroblock);
    
    // Stack I_n on top of zeroblock vertically
    let mdl = i_n.vstack(&zeroblock);
    draw_mat("mdl", &mdl);
    
    // Horizontally concatenate mdl and big_n
    let md = mdl.hstack(&big_n);
    draw_mat("md", &md);
    
    // Create the no_output matrix that will be stacked below md
    // This is [I_{2*outs} | 0] where I is identity and 0 is zero matrix
    let eye_part = Mat2::id(2 * outs);
    let zero_part = Mat2::zeros(2 * outs, md.cols() - 2 * outs);
    let no_output = eye_part.hstack(&zero_part);
    
    // Vertically stack md and no_output
    let md_no_output = md.vstack(&no_output);
    draw_mat("md_no_output", &md_no_output);
    
    // Compute nullspace
    let mdnons = md_no_output.nullspace(false);
    log::debug!("Number of basis vectors in nullspace: {}", mdnons.len());
    
    // Convert each basis vector to a PauliWeb
    let mut pws = Vec::with_capacity(mdnons.len());
    for (i, basis) in mdnons.into_iter().enumerate() {
        log::debug!("Basis vector {}: {}", i, basis);
        
        // The basis vector is a row vector from the nullspace
        // We need to extract its elements to create our bitvector
        log::debug!("Creating bitvector of length: {}", basis.cols());
        let mut vec = bitvec![0; basis.cols()];
        for i in 0..basis.cols() {
            // Get the value from the basis row vector
            let val = basis.get(0, i);
            log::debug!("Setting bit {} to {}", i, val);
            vec.set(i, val);
        }
        log::debug!("Bitvector: {:#?}", vec);
        // Create and store the PauliWeb
        let pw = get_pw(&index_map, &vec, g);
        pws.push(pw);
    }
    
    pws
}
