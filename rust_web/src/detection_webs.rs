use std::collections::HashMap;
use thiserror::Error;
use crate::f2linalg::{Mat2, F2};
use crate::pauliweb::{Pauli, PauliWeb};
use quizx::hash_graph::Graph;
use quizx::graph::{GraphLike, VType};

#[derive(Error, Debug)]
pub enum DetectionWebsError {
    #[error("Failed to convert matrix")]
    MatrixConversionError,
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
    /// - ordered_nodes: Vector of node indices with outputs first, then non-outputs, then inputs
    /// - index_map: Mapping from node index to its position in the ordered list
    fn ordered_nodes(&self) -> (Vec<usize>, HashMap<usize, usize>) {
        // Get all vertices
        let all_vertices: Vec<usize> = self.graph.vertices().collect();
        
        // Separate vertices into categories
        let outputs: Vec<usize> = self.graph.outputs().iter().cloned().collect();
        let inputs: Vec<usize> = self.graph.inputs().iter().cloned().collect();
        
        // Get non-input, non-output vertices
        let mut middle_vertices: Vec<usize> = all_vertices.iter()
            .filter(|&&v| !outputs.contains(&v) && !inputs.contains(&v))
            .cloned()
            .collect();
        
        // Sort for consistent ordering
        middle_vertices.sort();
        
        // Combine in order: outputs first, then middle vertices, then inputs
        let mut ordered = outputs;
        ordered.extend(middle_vertices);
        ordered.extend(inputs);
        
        // Create index map
        let index_map: HashMap<_, _> = ordered.iter()
            .enumerate()
            .map(|(i, &v)| (v, i))
            .collect();
        
        (ordered, index_map)
    }

    /// Convert the graph to RG form (Red-Green form)
    /// This implements the same logic as the Python version from detection_webs.py
    pub fn make_rg(mut self) -> Result<Self, DetectionWebsError> {
        // Clone the graph to work on a copy while preserving the original
        let mut working_graph = self.graph.clone();
        let mut modified = true;
        
        // We need to collect nodes first to avoid borrowing issues
        let nodes: Vec<_> = working_graph.vertices().collect();
        
        for &node in &nodes {
            let node_color = working_graph.vertex_type(node);
            let neighbors: Vec<_> = working_graph.neighbor_vec(node);
            
            for &neighbor in &neighbors {
                // Only process if both nodes have the same type
                if working_graph.vertex_type(neighbor) == node_color {
                    // Calculate position for the new vertex
                    let _row = (working_graph.row(node) + working_graph.row(neighbor)) / 2.0;
                    let _qubit = (working_graph.qubit(node) + working_graph.qubit(neighbor)) / 2.0;
                    
                    // Remove the edge between the nodes
                    working_graph.remove_edge(node, neighbor);
                    
                    // Add a new vertex with the opposite color
                    let new_color = match node_color {
                        VType::X => VType::Z,  // Toggle between X and Z
                        VType::Z => VType::X,
                        _ => node_color,  // Keep other types as is
                    };
                    // Create a new vertex with the given color
                    let new_vertex = working_graph.add_vertex(new_color);
                    
                    // Add edges for the new vertex
                    for neighbor in working_graph.neighbor_vec(node) {
                        if neighbor != node {  // Avoid self-loops
                            working_graph.add_edge(new_vertex, neighbor);
                        }
                    }
                    
                    // We've modified the graph, so we need to break and restart
                    // with the updated graph structure
                    modified = true;
                    break;
                }
            }
            
            if modified {
                break;
            }
        }
        
        // Update the graph in self with our modified version
        self.graph = working_graph;
        Ok(self)
    }

    /// Get the detection webs for the graph
    pub fn get_detection_webs(&self) -> Result<Vec<PauliWeb>, DetectionWebsError> {
        // Convert to RG form
        let rg_graph = self.clone().make_rg()?;
        
        // Get ordered nodes and index mapping
        let (new_order, index_map) = rg_graph.ordered_nodes();
        let num_nodes = new_order.len();
        // Get the number of inputs and outputs using non-mutating methods
        let num_outputs = rg_graph.graph.inputs().len() + rg_graph.graph.outputs().len();
        
        if num_nodes == 0 {
            return Ok(Vec::new());
        }
        
        // Create adjacency matrix
        let mut adj_matrix = vec![vec![0u8; num_nodes]; num_nodes];
        
        // Fill adjacency matrix
        for (i, &node) in new_order.iter().enumerate() {
            for neighbor in rg_graph.graph.neighbor_vec(node) {
                if let Some(j) = new_order.iter().position(|&n| n == neighbor) {
                    adj_matrix[i][j] = 1;
                }
            }
        }
        
        // Convert to Mat2
        let n = adj_matrix.len();
        let m = if n > 0 { adj_matrix[0].len() } else { 0 };
        let mut mat = Mat2::zeros(n, m);
        
        for i in 0..n {
            for j in 0..m {
                mat.set(i, j, F2::from_u8(adj_matrix[i][j]));
            }
        }
        
        // Create the matrix [I_n | N] as in the Python code
        let mut extended_matrix = Mat2::zeros(num_nodes, num_nodes + num_outputs);
        
        // Set identity matrix in the first num_outputs rows and columns
        for i in 0..num_outputs {
            extended_matrix.set(i, i, F2::One);
        }
        
        // Set the adjacency matrix in the remaining part
        for i in 0..num_nodes {
            for j in 0..num_nodes {
                let val = mat.get(i, j).unwrap_or_else(|| F2::Zero);
                extended_matrix.set(i, j + num_outputs, val);
            }
        }
        
        // Add a stack of single-entry rows to eliminate outputs of the graphs firing vector
        let mut full_matrix = extended_matrix;
        let num_cols = full_matrix.cols();
        
        // Add rows for outputs
        for i in 0..num_outputs {
            // For X output
            let mut new_row = Mat2::zeros(1, num_cols);
            new_row.set(0, i, F2::One);
            full_matrix = full_matrix.vstack(&new_row);
            
            // For Z output
            let mut new_row = Mat2::zeros(1, num_cols);
            new_row.set(0, i + num_outputs, F2::One);
            full_matrix = full_matrix.vstack(&new_row);
        }
        
        // Compute nullspace to get the detection webs
        let nullspace = full_matrix.nullspace(true);
        
        // Convert nullspace vectors to PauliWebs
        let mut pauli_webs = Vec::new();
        
        for vec in nullspace {
            if let Some(pw) = self.vector_to_pauliweb(&vec, &new_order, &index_map) {
                pauli_webs.push(pw);
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
        // Get the number of inputs and outputs using non-mutating methods
        let num_outputs = self.graph.inputs().len() + self.graph.outputs().len();
        let mut has_edges = false;
        
        // Process each non-zero entry in the vector
        for (i, &node_idx) in new_order.iter().enumerate() {
            // Use the public get method to access matrix elements
            match vector.get(i, 0) {
                Some(F2::Zero) => continue,
                None => continue,
                _ => {}
            }
            
            let node_type = self.graph.vertex_type(node_idx);
            
            // Check if this is a node we should process (not an input/output)
            if i >= num_outputs {
                // Get all edges connected to this node
                for neighbor in self.graph.neighbor_vec(node_idx) {
                    if let Some(&_j) = index_map.get(&neighbor) {
                        let edge = if node_idx < neighbor {
                            (node_idx, neighbor)
                        } else {
                            (neighbor, node_idx)
                        };
                        
                        // Set the edge type based on the node type
                        let pauli = match node_type {
                            VType::X => Pauli::X,  // X-spider
                            VType::Z => Pauli::Z,  // Z-spider
                            _ => continue,  // Skip other node types
                        };
                        
                        pw.set_edge(edge.0, edge.1, pauli);
                        has_edges = true;
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
        let v2 = graph.add_vertex(VType::X);  // X-spider
        let v3 = graph.add_vertex(VType::Z);  // Z-spider
        let v4 = graph.add_vertex(VType::WOutput);  // Output
        
        graph.add_edge(v1, v2);
        graph.add_edge(v2, v3);
        graph.add_edge(v3, v4);
        
        graph.set_inputs(vec![v1]);
        graph.set_outputs(vec![v4]);
        
        let detector = DetectionWebs::new(graph);
        let (ordered, _) = detector.ordered_nodes();
        
        // Check that outputs come first, then non-outputs
        assert_eq!(ordered.len(), 4);  // All nodes should be included
        assert_eq!(ordered[0], v4);    // Output should be first
        assert_eq!(ordered[3], v1);    // Input should be last
    }
    
    // More tests would be needed for the full functionality
}
