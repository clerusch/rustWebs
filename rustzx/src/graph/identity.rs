use super::{Graph, NodeType};

impl Graph {
    pub fn remove_identity_spider(&mut self, id: usize) -> Result<(), String> {
        use NodeType::*;

        let node = self.nodes.get(&id).ok_or("Node not found")?.clone();

        let (_, phase): (fn(f64) -> NodeType, f64) = match node.node_type {
            ZSpider(p) => (ZSpider as fn(f64) -> NodeType, p),
            XSpider(p) => (XSpider as fn(f64) -> NodeType, p),
            _ => return Err("Not a spider".to_string()),
        };
        
        let phase = phase%1.0;
        if phase != 0.0 {
            return Err("Spider phase is not 0".to_string());
        }

        let neighbors = self.neighbors(id);
        if neighbors.len() != 2 {
            return Err("Spider does not have degree 2".to_string());
        }

        // Remove the spider and reconnect its neighbors
        self.remove_node(id);
        self.add_edge(neighbors[0], neighbors[1]);
        
        Ok(())
    }
}
