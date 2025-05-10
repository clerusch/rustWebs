#[derive(Debug, Clone)]
pub enum NodeType {
    ZSpider,
    XSpider,
    Boundary,
}
#[derive(Debug, Clone)]
pub struct Node {
    pub id: usize,
    pub node_type: NodeType,
    pub phase: f64,
}
#[derive(Debug)]
pub struct Edge {
    pub source: usize,
    pub target: usize,
}
#[derive(Debug)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    next_id: usize,
}
impl Graph {
    //Create empty graph
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            edges: vec![],
            next_id: 0,
        }
    }
    pub fn add_node(&mut self, node_type: NodeType, phase: f64)->usize{
        let node = Node {
            id: self.next_id,
            node_type,
            phase,
        };
        self.nodes.push(node);
        self.next_id += 1;
        self.next_id - 1
    }
}


