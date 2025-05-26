use quizx::hash_graph::Graph;
use quizx::graph::{GraphLike, VType};
use std::collections::HashSet;

pub fn make_rg(oldg: &mut Graph) -> () {
    // Modifies a graph in-place to make it in red-green form
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    loop {
        let g = oldg.clone(); // freshly cloned every time

        let mut modified = false;

        for node in g.vertices() {
            let node_type = g.vertex_type(node);

            for neighbor in g.neighbors(node) {
                let key = if node < neighbor { (node, neighbor) } else { (neighbor, node) };
                if visited.contains(&key) {
                    continue;
                }
                visited.insert(key);

                if g.vertex_type(neighbor) == node_type {
                    let row = (g.row(node) + g.row(neighbor)) / 2.0;
                    let qubit = (g.qubit(node) + g.qubit(neighbor)) / 2.0;

                    oldg.remove_edge(node, neighbor);

                    let new_type = match node_type {
                        VType::X => VType::Z,
                        VType::Z => VType::X,
                        _ => continue,
                    };

                    let new_vertex = oldg.add_vertex_with_data(quizx::graph::VData {
                        ty: new_type,
                        phase: 0.into(),
                        row,
                        qubit,
                    });

                    oldg.add_edge(node, new_vertex);
                    oldg.add_edge(new_vertex, neighbor);

                    modified = true;
                    break;
                }
            }

            if modified {
                break;
            }
        }

        if !modified {
            break;
        }
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use quizx::graph::GraphLike;
    
    #[test]
    fn test_make_rg() {
        // Create a simple graph with two X nodes connected by an edge
        let mut graph = Graph::new();
        let v1 = graph.add_vertex(VType::X);
        let v2 = graph.add_vertex(VType::X);
        graph.add_edge(v1, v2);
        
        
        
        // Debug output
        println!("Original graph: {} vertices, {} edges", graph.num_vertices(), graph.num_edges());
        // Apply RG transformation
        make_rg(&mut graph);
        println!("Transformed graph: {} vertices, {} edges", graph.num_vertices(), graph.num_edges());
        
        // In RG form, we expect:
        // 1. Original edge v1-v2 is removed
        // 2. A new Z node is added between them
        // 3. The new Z node is connected to both original nodes
        // Since this is a simple graph with just two connected nodes,
        // we expect exactly 2 edges in the transformed graph
        assert_eq!(graph.num_vertices(), 3, "Should have 3 vertices (2 original X nodes + 1 new Z node)");
        assert_eq!(graph.num_edges(), 2, "Should have 2 edges (v1-new_node and v2-new_node)");
        
        // Verify the new node is of type Z
        let new_node = graph.vertices()
            .find(|&v| v != v1 && v != v2)
            .expect("Should have a new node");
            
        assert_eq!(graph.vertex_type(new_node), VType::Z, "New node should be of type Z");
        
        // Verify connections
        assert!(graph.connected(v1, new_node), "v1 should be connected to new node");
        assert!(graph.connected(v2, new_node), "v2 should be connected to new node");
    }
}
