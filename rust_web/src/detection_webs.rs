use std::collections::HashMap;
use thiserror::Error;
use crate::f2linalg::{Mat2, F2};
use crate::pauliweb::{Pauli, PauliWeb};
use quizx::hash_graph::Graph;
use quizx::graph::{GraphLike, VType};
use crate::make_rg::make_rg;

#[derive(Error, Debug)]
pub enum DetectionWebsError {
    #[error("Graph error: {0}")]
    GraphError(String),
    #[error("Matrix error: {0}")]
    MatrixError(String),
    #[error("Error: {0}")]
    GenericError(String),
    #[error("Invalid graph: {0}")]
    InvalidGraph(String),
    #[error("Linear algebra operation failed")]
    LinearAlgebraError,
}

/// Represents a detection web in a ZX diagram
pub struct DetectionWebs {
    /// The underlying ZX graph
    pub graph: Graph,
}

impl DetectionWebs {
    /// Create a new DetectionWebs instance from a ZX graph
    pub fn new(graph: Graph) -> Self {
        Self {graph: make_rg(graph) }
    }

    /// Get ordered nodes and their index mapping
    /// Returns a tuple of (ordered_nodes, index_map) where:
    /// - ordered_nodes: Vector of node indices with outputs first, then non-outputs
    /// - index_map: Mapping from node index to its position in the ordered list
    fn ordered_nodes(&self) -> (Vec<usize>, HashMap<usize, usize>) {
        // Get all vertices in the graph
        let all_vertices: Vec<usize> = self.graph.vertices().collect();
        let outputs: Vec<usize> = self.graph.outputs().to_vec();
        
        log::debug!("All vertices: {:?}", all_vertices);
        log::debug!("Output vertices: {:?}", outputs);
        
        // Split into outputs and non-outputs
        let (mut outputs, non_outputs): (Vec<_>, Vec<_>) = 
            all_vertices.into_iter().partition(|v| outputs.contains(v));
            
        log::debug!("Found {} outputs and {} non-outputs", outputs.len(), non_outputs.len());
        
        // Sort for determinism
        outputs.sort_unstable();
        let mut non_outputs = non_outputs;
        non_outputs.sort_unstable();
        
        // Combine with outputs first
        let mut ordered_nodes = outputs;
        ordered_nodes.extend(non_outputs);
        
        log::debug!("Ordered nodes: {:?}", ordered_nodes);
        
        // Create index map
        let index_map: HashMap<_, _> = ordered_nodes.iter()
            .enumerate()
            .map(|(i, &v)| (v, i))
            .collect();
        
        (ordered_nodes, index_map)
    }

    /// Get the detection webs for the graph
    pub fn get_detection_webs(&self) -> Result<Vec<PauliWeb>, DetectionWebsError> {
        
        
        // Get ordered nodes and index mapping
        let (new_order, index_map) = self.ordered_nodes();
        println!("Ordered nodes: {:?}", new_order);
        println!("Index map: {:?}", index_map);
        
        // Create a networkx-like graph for easier manipulation
        let mut adj_matrix = vec![vec![0u8; new_order.len()]; new_order.len()];
        
        // Fill adjacency matrix with bounds checking
        log::debug!("Filling adjacency matrix for {} nodes", new_order.len());
        for (i, &node) in new_order.iter().enumerate() {
            log::debug!("Processing node {} at position {}", node, i);
            
            // Get neighbors with error handling
            let neighbors = self.graph.neighbor_vec(node);
            log::debug!("Node {} has {} neighbors", node, neighbors.len());
            
            for &neighbor in &neighbors {
                log::debug!("  Checking neighbor {} of node {}", neighbor, node);
                if let Some(j) = new_order.iter().position(|&n| n == neighbor) {
                    if i < adj_matrix.len() && j < adj_matrix[i].len() {
                        adj_matrix[i][j] = 1;
                    } else {
                        log::error!("Index out of bounds: adj_matrix[{}][{}] but dimensions are {}x{}", 
                                  i, j, adj_matrix.len(), 
                                  if !adj_matrix.is_empty() { adj_matrix[0].len() } else { 0 });
                    }
                } else {
                    log::warn!("Neighbor {} of node {} not found in new_order", neighbor, node);
                }
            }
        }
        
        // Number of inputs + outputs
        let _outs = self.graph.inputs().len() + self.graph.outputs().len();
        
        // Create mdl matrix (diagonal matrix with 1s for Z-spiders, 0s for X-spiders)
        let mut mdl = Mat2::zeros(new_order.len(), new_order.len());
        log::debug!("Creating MDL matrix for {} nodes", new_order.len());
        
        // Get all vertices in the RG graph
        let vertices: Vec<_> = self.graph.vertices().collect();
        log::debug!("Graph has {} vertices: {:?}", vertices.len(), vertices);
        
        log::debug!("Node types and indices ({} nodes):", new_order.len());
        for (i, &node_idx) in new_order.iter().enumerate() {
            // Verify node exists in RG graph before accessing its type
            if !vertices.contains(&node_idx) {
                let err_msg = format!("Node {} not found in RG graph when accessing vertex type. Available vertices: {:?}", node_idx, vertices);
                log::error!("{}", err_msg);
                return Err(DetectionWebsError::InvalidGraph(err_msg));
            }
            
            let node_type = self.graph.vertex_type(node_idx);
            match node_type {
                VType::Z => {
                    log::trace!("Node {} at position {} is type Z, setting MDL[{}][{}] = 1", node_idx, i, i, i);
                    mdl.set(i, i, F2::One);
                }
                _ => {
                    log::trace!("Node {} at position {} is type {:?}, leaving MDL[{}][{}] as 0", node_idx, i, node_type, i, i);
                }
            }
        }
        
        println!("mdl matrix ({}x{}):", mdl.rows(), mdl.cols());
        for i in 0..mdl.rows() {
            print!("  [");
            for j in 0..mdl.cols() {
                match mdl.get(i, j) {
                    Some(F2::One) => print!("1 "),
                    Some(F2::Zero) => print!(". "),
                    None => print!("? "),
                }
            }
            println!("]");
        }
        
        // Create N matrix (adjacency matrix)
        let mut n = Mat2::zeros(new_order.len(), new_order.len());
        for i in 0..new_order.len() {
            for j in 0..new_order.len() {
                n.set(i, j, F2::from_u8(adj_matrix[i][j]));
            }
        }
        
        log::trace!("Adjacency matrix ({}x{}):", n.rows(), n.cols());
        for i in 0..n.rows() {
            log::trace!("  [");
            for j in 0..n.cols() {
                match n.get(i, j) {
                    Some(F2::One) => log::trace!("1 "),
                    Some(F2::Zero) => log::trace!(". "),
                    None => log::trace!("? "),
                }
            }
            log::trace!("]");
        }
        
        // Create [I_n | N] matrix where I_n is identity of size outs x outs
        let outs = self.graph.inputs().len() + self.graph.outputs().len();
        let mut md = Mat2::zeros(n.rows(), outs + n.cols());
        
        // Set identity in the first 'outs' columns
        for i in 0..outs {
            md.set(i, i, F2::One);
        }
        
        // Set N matrix in the remaining columns
        for i in 0..n.rows() {
            for j in 0..n.cols() {
                if let Some(val) = n.get(i, j) {
                    md.set(i, outs + j, val);
                }
            }
        }
        
        // Create the no_output matrix: [I_{2*outs} | 0]
        let md_no_output = if outs > 0 {
            let mut no_output = Mat2::zeros(2 * outs, outs + n.cols());
            
            // Set identity in the first 2*outs columns
            for i in 0..2 * outs {
                no_output.set(i, i, F2::One);
            }
            
            log::trace!("md dimensions: {}x{}", md.rows(), md.cols());
            log::trace!("no_output dimensions: {}x{}", no_output.rows(), no_output.cols());
            
            // Stack md and no_output vertically
            md.vstack(&no_output)
        } else {
            // If no outputs, just use md as is
            log::trace!("No outputs detected, skipping output constraints");
            md
        };
        println!("Md_no_output looks like: {}", md_no_output);
        // Compute nullspace with timeout check
        println!("Computing nullspace...");
        // Get a basis for the nullspace of the augmented matrix
        let nullspace_basis = md_no_output.nullspace(true);
        let basis_size = nullspace_basis.len();
        
        println!("Found {} basis vectors for the nullspace (dimension {})", 
                  basis_size, basis_size);
        
        // Convert each basis vector to a PauliWeb
        let mut detection_webs = Vec::with_capacity(basis_size);
        for (i, vector) in nullspace_basis.into_iter().enumerate() {
            // Skip the first 'outs' columns which correspond to the identity part
            let vector_slice = vector.submatrix(0, outs, vector.rows(), vector.cols() - outs).unwrap_or_else(|| vector.clone());
            if let Some(pauli_web) = self.vector_to_pauliweb(&vector_slice, &new_order, &index_map) {
                detection_webs.push(pauli_web);
            } else {
                log::warn!("Failed to convert basis vector {} to PauliWeb", i);
            }
        }
        
        println!("Successfully converted {} out of {} basis vectors to PauliWebs", 
                  detection_webs.len(), basis_size);
        
        Ok(detection_webs)
    }
    
    /// Convert a vector from the nullspace to a PauliWeb
    fn vector_to_pauliweb(
        &self,
        vector: &Mat2,
        new_order: &[usize],
        index_map: &HashMap<usize, usize>,
    ) -> Option<PauliWeb> {
        log::debug!("Converting vector to PauliWeb");
        log::debug!("  Vector dimensions: {}x{}", vector.rows(), vector.cols());
        log::debug!("  New order length: {}", new_order.len());
        log::debug!("  Index map size: {}", index_map.len());
        
        // Log the first few nodes in new_order for debugging
        let sample_size = new_order.len().min(10);
        log::debug!("  First {} nodes in new_order: {:?}", sample_size, &new_order[..sample_size]);
        use std::collections::HashSet;
        let mut edges = HashSet::new();
        let outs = 0; // Assuming no outputs for now
        
        println!("  Processing vector with {} entries (outs: {})", vector.rows(), outs);
        
        // Get the set of valid vertex indices in the graph
        let vertices: std::collections::HashSet<_> = self.graph.vertices().collect();
        log::debug!("  Graph contains {} vertices", vertices.len());
        
        // Log the range of vertex indices in the graph
        if let (Some(min_idx), Some(max_idx)) = (vertices.iter().min(), vertices.iter().max()) {
            log::debug!("  Vertex indices range: {} to {}", min_idx, max_idx);
        }
        
        // Filter out nodes that don't exist in the graph
        let valid_new_order: Vec<_> = new_order.iter()
            .filter(|&&idx| vertices.contains(&idx))
            .cloned()
            .collect();
            
        if valid_new_order.len() != new_order.len() {
            let missing_count = new_order.len() - valid_new_order.len();
            log::warn!("Filtered out {} nodes that don't exist in the graph", missing_count);
            
            // Log the first few filtered indices for debugging
            let filtered_indices: Vec<_> = new_order.iter()
                .filter(|&&idx| !vertices.contains(&idx))
                .take(10)
                .collect();
            log::debug!("  Example filtered indices: {:?}...", filtered_indices);
        }
        
        // If no valid nodes remain, return None
        if valid_new_order.is_empty() {
            log::warn!("No valid nodes remaining after filtering");
            return None;
        }
        
        // Use the filtered new_order for processing
        let new_order = &valid_new_order;
        
        // Get the non-zero indices in the vector
        for i in outs..new_order.len() {
            // Check if the vector component is non-zero
            if let Some(F2::One) = vector.get(i, 0) {
                let node_idx = new_order[i];
                
                // Get node type
                let node_type = self.graph.vertex_type(node_idx);
                log::trace!("Node {} has type: {:?}", node_idx, node_type);
                
                println!("    Node {} (type: {:?}) is set in vector", node_idx, node_type);
                
                // Get all edges connected to this node
                let neighbors = self.graph.neighbor_vec(node_idx);
                log::trace!("Node {} has {} neighbors", node_idx, neighbors.len());
                
                println!("      Node has {} neighbors: {:?}", neighbors.len(), neighbors);
                
                for neighbor in neighbors {
                    if !index_map.contains_key(&neighbor) {
                        log::warn!("Neighbor {} of node {} not found in index_map", neighbor, node_idx);
                        continue;
                    }
                    
                    let edge = if node_idx < neighbor {
                        (node_idx, neighbor)
                    } else {
                        (neighbor, node_idx)
                    };
                    
                    // Set the edge type based on the node type
                    match node_type {
                        VType::X | VType::Z => {
                            let pauli = match node_type {
                                VType::X => Pauli::X,
                                VType::Z => Pauli::Z,
                                _ => unreachable!(),
                            };
                            
                            println!("      Adding {:?} edge: {:?}", pauli, edge);
                            edges.insert((edge.0, edge.1, pauli));
                        }
                        _ => {
                            println!("      Skipping non-X/Z node type: {:?}", node_type);
                            continue;
                        }
                    }
                }
            }
        }
        
        if edges.is_empty() {
            println!("  No edges in PauliWeb");
            None
        } else {
            println!("  Converted to PauliWeb with {} edges", edges.len());
            let mut pauli_web = PauliWeb::new();
            for (from, to, pauli) in edges {
                pauli_web.set_edge(from, to, pauli);
            }
            Some(pauli_web)
        }
    }
}

impl Clone for DetectionWebs {
    fn clone(&self) -> Self {
        // Implement clone by creating a new DetectionWebs with a clone of the graph
        // Note: This assumes Graph implements Clone. If not, you'll need to implement
        // a proper deep copy of the graph.
        Self {
            graph: self.graph.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quizx::hash_graph::Graph;
    
    #[test]
    fn test_ordered_nodes() {
        // Create a simple graph for testing
        let mut graph = Graph::new();
        let v1 = graph.add_vertex(VType::WInput);  // Input
        let v2 = graph.add_vertex(VType::WInput);  // Input

        let v3 = graph.add_vertex(VType::Z);  
        let v4 = graph.add_vertex(VType::Z); 
        let v5 = graph.add_vertex(VType::Z);
        let v6 = graph.add_vertex(VType::Z);
        let v7 = graph.add_vertex(VType::WOutput);
        let v8 = graph.add_vertex(VType::WOutput);  // Output
        
        graph.add_edge(v1, v3);
        graph.add_edge(v2, v4);
        graph.add_edge(v3, v4);
        graph.add_edge(v3,v5);
        graph.add_edge(v4,v6);
        graph.add_edge(v5,v6);
        graph.add_edge(v5,v7);
        graph.add_edge(v6,v8);
        
        graph.set_inputs(vec![v1,v2]);
        graph.set_outputs(vec![v7,v8]);
        
        let detector = DetectionWebs::new(graph);
        let (ordered, index_map) = detector.ordered_nodes();
        
        // Check that all nodes are included
        assert_eq!(ordered.len(), 8);
        
        // Debug output to understand node ordering
        println!("Ordered nodes: {:?}", ordered);
        println!("v7: {}, v8: {}", v7, v8);
        
        // Check that outputs come first
        // The exact order might depend on the graph implementation's vertex numbering
        assert!(ordered[0] == v7 || ordered[0] == v8, 
               "First element should be an output, got: {} (expected {} or {})", 
               ordered[0], v7, v8);
        assert!(ordered[1] == v7 || ordered[1] == v8, 
               "Second element should be an output, got: {} (expected {} or {})", 
               ordered[1], v7, v8);
        assert_ne!(ordered[0], ordered[1], "Outputs should be different");
        
        // Check that the index map is consistent
        for (i, &v) in ordered.iter().enumerate() {
            assert_eq!(index_map[&v], i);
        }
    }
    
    // More tests would be needed for the full functionality
}
