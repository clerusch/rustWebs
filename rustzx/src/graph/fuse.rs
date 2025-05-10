use super::{Graph, NodeType};

impl Graph {
    pub fn neighbors(&self, id: usize) -> Vec<usize> {
        self.edges
            .iter()
            .filter_map(|(&(a, b), _)| {
                if a == id {
                    Some(b)
                } else if b == id {
                    Some(a)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn fuse_spiders(&mut self, a: usize, b: usize) -> Result<(), String> {
        use NodeType::*;

        let node_a = self.nodes.get(&a).ok_or("Node a not found")?.clone();
        let node_b = self.nodes.get(&b).ok_or("Node b not found")?.clone();

        match (&node_a.node_type, &node_b.node_type) {
            (ZSpider(pa), ZSpider(pb)) => self.merge_spiders(a, b, (pa + pb)%1.0, ZSpider),
            (XSpider(pa), XSpider(pb)) => self.merge_spiders(a, b, (pa + pb)%1.0, XSpider),
            _ => return Err("Mismatched spider types".to_string()),
        }

        Ok(())
    }

    fn merge_spiders<F>(&mut self, target: usize, to_remove: usize, new_phase: f64, ctor: F)
    where
        F: Fn(f64) -> NodeType,
    {
        self.nodes.insert(
            target,
            super::types::Node {
                id: target,
                node_type: ctor(new_phase),
            },
        );

        let neighbors = self.neighbors(to_remove);
        for n in neighbors {
            if n != target {
                self.add_edge(target, n);
            }
        }

        self.remove_node(to_remove);
        self.remove_edge(target, to_remove);
    }
}
