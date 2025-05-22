use std::collections::HashMap;
use thiserror::Error;
use crate::f2linalg::{Mat2, F2};
use crate::pauliweb::{Pauli, PauliWeb};
use quizx::hash_graph::Graph;
use quizx::graph::{GraphLike, VType};

#[derive(Error, Debug)]
pub enum DetectionWebsError {
    #[error("Graph error: {0}")]
    GraphError(String),
    #[error("Matrix error: {0}")]
    MatrixError(String),
    #[error("Error: {0}")]
    GenericError(String),
    #[error("Invalid graph structure")]
    InvalidGraph,
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
        Self { graph }
    }

    /// Get ordered nodes and their index mapping
    /// Returns a tuple of (ordered_nodes, index_map) where:
    /// - ordered_nodes: Vector of node indices with outputs first, then non-outputs
    /// - index_map: Mapping from node index to its position in the ordered list
    fn ordered_nodes(&self) -> (Vec<usize>, HashMap<usize, usize>) {
        // Get all vertices
        let all_vertices: Vec<usize> = self.graph.vertices().collect();
        
        // Get output vertices first
        let outputs: Vec<usize> = self.graph.outputs().to_vec();
        
        // Get non-output vertices (including inputs and other nodes)
        let mut other_nodes: Vec<usize> = all_vertices.iter()
            .filter(|&&v| !outputs.contains(&v))
            .cloned()
            .collect();
        
        // Sort for consistent ordering
        other_nodes.sort();
        
        // Combine outputs first, then other nodes
        let ordered_nodes: Vec<usize> = outputs.into_iter().chain(other_nodes.into_iter()).collect();
        
        // Create index map
        let index_map: HashMap<_, _> = ordered_nodes.iter()
            .enumerate()
            .map(|(i, &v)| (v, i))
            .collect();
        
        (ordered_nodes, index_map)
    }

    /// Convert the graph to RG form (Red-Green form)
    /// This implements the same logic as the Python version from detection_webs.py
    pub fn make_rg(mut self) -> Result<Self, DetectionWebsError> {
        let mut modified = true;
        let mut iteration = 0;
        const MAX_ITERATIONS: usize = 100;  // Safety limit to prevent infinite loops
        
        while modified && iteration < MAX_ITERATIONS {
            iteration += 1;
            println!("RG conversion iteration {}", iteration);
            modified = false;
            let mut edges_to_process = Vec::new();
            
            // First pass: collect edges to process
            println!("  Current graph has {} vertices and {} edges", 
                    self.graph.num_vertices(), self.graph.num_edges());
            
            // Collect all edges first to avoid borrowing issues
            let edges: Vec<_> = self.graph.edges().collect();
            
            for (src, tgt, _) in edges {
                let src_type = self.graph.vertex_type(src);
                let tgt_type = self.graph.vertex_type(tgt);
                
                // We only process edges between nodes of the same type
                if src_type == tgt_type && (src_type == VType::X || src_type == VType::Z) {
                    println!("    Found edge between same-type nodes: {} ({:?}) - {} ({:?})", 
                            src, src_type, tgt, tgt_type);
                    edges_to_process.push((src, tgt));
                }
            }
            
            if iteration >= MAX_ITERATIONS {
                println!("WARNING: Reached maximum number of iterations ({}) in make_rg. Graph may not be in RG form.", MAX_ITERATIONS);
                break;
            }
            
            // Process edges in reverse order to avoid index shifting issues
            for (src, tgt) in edges_to_process.into_iter().rev() {
                // Skip if either node was already removed
                if !self.graph.contains_vertex(src) || !self.graph.contains_vertex(tgt) {
                    println!("    Skipping edge {} - {} (node already removed)", src, tgt);
                    continue;
                }
                
                println!("    Processing edge {} - {} (types: {:?})", 
                        src, tgt, self.graph.vertex_type(src));
                
                // Get the neighbors of both nodes (excluding each other)
                let src_neighbors: Vec<_> = self.graph.neighbor_vec(src)
                    .into_iter()
                    .filter(|&n| n != tgt)
                    .collect();
                
                let tgt_neighbors: Vec<_> = self.graph.neighbor_vec(tgt)
                    .into_iter()
                    .filter(|&n| n != src)
                    .collect();
                
                println!("      src neighbors (excluding tgt): {:?}", src_neighbors);
                println!("      tgt neighbors (excluding src): {:?}", tgt_neighbors);
                
                // Remove the original edge
                self.graph.remove_edge(src, tgt);
                
                // Add a new node of the opposite type
                let new_node_type = match self.graph.vertex_type(src) {
                    VType::X => VType::Z,
                    VType::Z => VType::X,
                    t => {
                        println!("      Unexpected node type: {:?}", t);
                        return Err(DetectionWebsError::GenericError("Unexpected node type".to_string()));
                    }
                };
                
                println!("      Adding new node of type {:?}", new_node_type);
                let new_node = self.graph.add_vertex(new_node_type);
                
                // Connect the new node to the original nodes
                println!("      Connecting new node to original nodes: {} and {}", src, tgt);
                self.graph.add_edge(src, new_node);
                self.graph.add_edge(tgt, new_node);
                
                // Connect the new node to all neighbors of src and tgt
                for &n in &src_neighbors {
                    println!("      Connecting new node to src's neighbor: {}", n);
                    self.graph.add_edge(new_node, n);
                }
                
                for &n in &tgt_neighbors {
                    println!("      Connecting new node to tgt's neighbor: {}", n);
                    self.graph.add_edge(new_node, n);
                }
                
                modified = true;
                println!("      Edge processing complete. Graph now has {} vertices and {} edges",
                        self.graph.num_vertices(), self.graph.num_edges());
            }
        }
        
        // Update the graph in self with our modified version
        Ok(self)
    }

    /// Get the detection webs for the graph
    pub fn get_detection_webs(&self) -> Result<Vec<PauliWeb>, DetectionWebsError> {
        use std::time::Instant;
        
        let start_time = Instant::now();
        const TIMEOUT: std::time::Duration = std::time::Duration::from_secs(30); // 30 second timeout
        
        println!("Converting graph to RG form...");
        let rg_graph = self.clone().make_rg()?;
        
        if start_time.elapsed() > TIMEOUT {
            return Err(DetectionWebsError::GenericError("Operation timed out during RG conversion".to_string()));
        }
        
        println!("RG conversion complete in {:?}. Graph now has {} vertices and {} edges", 
                start_time.elapsed(), rg_graph.graph.num_vertices(), rg_graph.graph.num_edges());
        
        // Get ordered nodes and index mapping
        let (new_order, index_map) = rg_graph.ordered_nodes();
        println!("Ordered nodes: {:?}", new_order);
        println!("Index map: {:?}", index_map);
        
        // Create a networkx-like graph for easier manipulation
        let mut adj_matrix = vec![vec![0u8; new_order.len()]; new_order.len()];
        
        // Fill adjacency matrix
        for (i, &node) in new_order.iter().enumerate() {
            for neighbor in rg_graph.graph.neighbor_vec(node) {
                if let Some(j) = new_order.iter().position(|&n| n == neighbor) {
                    adj_matrix[i][j] = 1;
                }
            }
        }
        
        // Number of inputs + outputs
        let outs = rg_graph.graph.inputs().len() + rg_graph.graph.outputs().len();
        
        // Create mdl matrix (diagonal matrix with 1s for Z-spiders, 0s for X-spiders)
        let mut mdl = Mat2::zeros(new_order.len(), new_order.len());
        println!("Node types and indices:");
        for (i, &node_idx) in new_order.iter().enumerate() {
            println!("  Node {}: index={}, type={:?}", i, node_idx, self.graph.vertex_type(node_idx));
            let node_type = self.graph.vertex_type(node_idx);
            if let VType::Z = node_type {
                mdl.set(i, i, F2::One);
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
        
        println!("Adjacency matrix ({}x{}):", n.rows(), n.cols());
        for i in 0..n.rows() {
            print!("  [");
            for j in 0..n.cols() {
                match n.get(i, j) {
                    Some(F2::One) => print!("1 "),
                    Some(F2::Zero) => print!(". "),
                    None => print!("? "),
                }
            }
            println!("]");
        }
        
        // Create md = [mdl | N] by horizontally stacking the matrices
        let mut md = mdl.hstack(&n);
        
        println!("md matrix ({}x{}):", md.rows(), md.cols());
        for i in 0..md.rows() {
            print!("  [");
            for j in 0..md.cols() {
                match md.get(i, j) {
                    Some(F2::One) => print!("1 "),
                    Some(F2::Zero) => print!(". "),
                    None => print!("? "),
                }
            }
            println!("]");
        }
        
        // Add single-entry rows to eliminate outputs of the graph's firing vector
        for i in 0..outs {
            // For X output
            let mut row = Mat2::zeros(1, md.cols());
            row.set(0, i, F2::One);
            md = md.vstack(&row);
            
            // For Z output
            let mut row = Mat2::zeros(1, md.cols());
            row.set(0, i + outs, F2::One);
            md = md.vstack(&row);
        }
        
        println!("Final matrix before nullspace ({}x{}):", md.rows(), md.cols());
        for i in 0..md.rows() {
            print!("  [");
            for j in 0..md.cols() {
                match md.get(i, j) {
                    Some(F2::One) => print!("1 "),
                    Some(F2::Zero) => print!(". "),
                    None => print!("? "),
                }
            }
            println!("]");
        }
        
        // Compute nullspace with timeout check
        println!("Computing nullspace...");
        let nullspace_start = Instant::now();
        let nullspace = md.nullspace(true);
        
        if start_time.elapsed() > TIMEOUT {
            return Err(DetectionWebsError::GenericError("Operation timed out during nullspace computation".to_string()));
        }
        
        println!("Found {} basis vectors in nullspace (took {:?})", nullspace.len(), nullspace_start.elapsed());
        
        // Convert nullspace vectors to PauliWebs
        let mut pauli_webs = Vec::new();
        
        for (i, vec) in nullspace.into_iter().enumerate() {
            println!("Processing nullspace vector {}: {:?}", i, vec);
            if let Some(pw) = self.vector_to_pauliweb(&vec, &new_order, &index_map) {
                println!("  Converted to PauliWeb with {} edges", pw.edge_operators.len());
                pauli_webs.push(pw);
            } else {
                println!("  Could not convert to PauliWeb (no valid edges)");
            }
        }
        
        Ok(pauli_webs)
    }
    
    /// Convert a vector from the nullspace to a PauliWeb
    fn vector_to_pauliweb(
        &self,
        vector: &Mat2,
        new_order: &[usize],
        index_map: &HashMap<usize, usize>,
    ) -> Option<PauliWeb> {
        let mut pw = PauliWeb::new();
        let outs = self.graph.inputs().len() + self.graph.outputs().len();
        let mut has_edges = false;
        
        println!("  Processing vector with {} entries (outs: {})", vector.rows(), outs);
        
        // Get the non-zero indices in the vector
        for i in outs..new_order.len() {
            // Check if the vector component is non-zero
            if let Some(F2::One) = vector.get(i, 0) {
                let node_idx = new_order[i];
                let node_type = self.graph.vertex_type(node_idx);
                println!("    Node {} (type: {:?}) is set in vector", node_idx, node_type);
                
                // Get all edges connected to this node
                let neighbors = self.graph.neighbor_vec(node_idx);
                println!("      Node has {} neighbors: {:?}", neighbors.len(), neighbors);
                
                for neighbor in neighbors {
                    if let Some(&_j) = index_map.get(&neighbor) {
                        let edge = if node_idx < neighbor {
                            (node_idx, neighbor)
                        } else {
                            (neighbor, node_idx)
                        };
                        
                        // Set the edge type based on the node type
                        let pauli = match node_type {
                            VType::X => {
                                println!("      Adding X edge: {:?}", edge);
                                Pauli::X  // X-spider -> X edge
                            },
                            VType::Z => {
                                println!("      Adding Z edge: {:?}", edge);
                                Pauli::Z  // Z-spider -> Z edge
                            },
                            _ => {
                                println!("      Skipping non-X/Z node type: {:?}", node_type);
                                continue  // Skip other node types
                            },
                        };
                        
                        pw.set_edge(edge.0, edge.1, pauli);
                        has_edges = true;
                        println!("      Edge added successfully");
                    }
                }
            }
        }
        
        if has_edges { Some(pw) } else { None }
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
        
        // Check that outputs come first
        assert_eq!(ordered[1], v8);  // Output should be first
        assert_eq!(ordered[0], v7);  // Output should be first
        
        // Check that the index map is consistent
        for (i, &v) in ordered.iter().enumerate() {
            assert_eq!(index_map[&v], i);
        }
    }
    
    // More tests would be needed for the full functionality
}
