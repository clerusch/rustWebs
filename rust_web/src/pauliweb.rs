use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Pauli {
    X,
    Y,
    Z,
}

/// Represents a Pauli web in a ZX diagram
#[derive(Debug, Default, Clone)]
pub struct PauliWeb {
    /// Maps edge (from, to) to Pauli operator
    /// Note: from < to to ensure consistent ordering
    pub edge_operators: HashMap<(usize, usize), Pauli>,
}

impl PauliWeb {
    /// Create a new empty PauliWeb
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the Pauli operator for an edge between two nodes
    pub fn set_edge(&mut self, from: usize, to: usize, pauli: Pauli) {
        self.edge_operators.insert((from.min(to), from.max(to)), pauli);
    }

    /// Get the Pauli operator for an edge between two nodes
    pub fn get_edge(&self, from: usize, to: usize) -> Option<Pauli> {
        self.edge_operators.get(&(from.min(to), from.max(to))).copied()
    }

    /// Get the color to use when drawing an edge
    pub fn get_edge_color(&self, from: usize, to: usize) -> Option<&'static str> {
        self.get_edge(from, to).map(|pauli| match pauli {
            Pauli::X => "green",  // Green for X operators
            Pauli::Y => "blue",   // Blue for Y operators
            Pauli::Z => "red",    // Red for Z operators
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_pauliweb() {
        let pw = PauliWeb::new();
        assert!(pw.edge_operators.is_empty());
    }

    #[test]
    fn test_set_and_get_edge() {
        let mut pw = PauliWeb::new();
        
        // Test setting and getting an edge
        pw.set_edge(1, 2, Pauli::X);
        assert_eq!(pw.get_edge(1, 2), Some(Pauli::X));
        assert_eq!(pw.get_edge(2, 1), Some(Pauli::X)); // Should work in both directions
        
        // Test updating an edge
        pw.set_edge(1, 2, Pauli::Z);
        assert_eq!(pw.get_edge(1, 2), Some(Pauli::Z));
        
        // Test non-existent edge
        assert_eq!(pw.get_edge(1, 3), None);
    }

    #[test]
    fn test_get_edge_color() {
        let mut pw = PauliWeb::new();
        
        // Test colors for each Pauli operator
        pw.set_edge(1, 2, Pauli::X);
        pw.set_edge(2, 3, Pauli::Y);
        pw.set_edge(3, 4, Pauli::Z);
        
        assert_eq!(pw.get_edge_color(1, 2), Some("green"));
        assert_eq!(pw.get_edge_color(2, 3), Some("blue"));
        assert_eq!(pw.get_edge_color(3, 4), Some("red"));
        assert_eq!(pw.get_edge_color(4, 5), None); // Non-existent edge
    }

    #[test]
    fn test_edge_ordering() {
        let mut pw = PauliWeb::new();
        
        // Test that edge ordering doesn't matter for get/set
        pw.set_edge(2, 1, Pauli::X);
        assert_eq!(pw.get_edge(1, 2), Some(Pauli::X));
        
        // Test that updating with different order works
        pw.set_edge(1, 2, Pauli::Z);
        assert_eq!(pw.get_edge(2, 1), Some(Pauli::Z));
    }
}
