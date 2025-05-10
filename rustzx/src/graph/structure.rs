use std::collections::HashMap;
use super::types::{Node, Edge, NodeType};

#[derive(Debug)]
pub struct Graph {
    pub nodes: HashMap<usize, Node>,
    pub edges: HashMap<(usize, usize), Edge>,
    next_id: usize,
}
#[allow(dead_code)]
impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn add_node(&mut self, node_type: NodeType) -> usize {
        let id = self.next_id;
        let node = Node { id, node_type };
        self.nodes.insert(id, node);
        self.next_id += 1;
        id
    }    

    pub fn add_edge(&mut self, source: usize, target: usize) {
        if self.nodes.contains_key(&source) && self.nodes.contains_key(&target) {
            self.edges.insert(Self::edge_key(source, target), Edge { source, target });
        } else {
            panic!("Attempted to connect non-existent nodes");
        }
    }

    pub fn remove_edge(&mut self, a: usize, b: usize) {
        self.edges.remove(&Self::edge_key(a, b));
    }

    pub fn has_edge(&self, a: usize, b: usize) -> bool {
        self.edges.contains_key(&Self::edge_key(a, b))
    }

    pub fn remove_node(&mut self, id: usize) {
        if self.nodes.remove(&id).is_none() {
            panic!("Node {} does not exist", id);
        }
        self.edges.retain(|&(a, b), _| a != id && b != id);
    }

    fn edge_key(a: usize, b: usize) -> (usize, usize) {
        if a < b { (a, b) } else { (b, a) }
    }
}
