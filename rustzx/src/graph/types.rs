
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