
#[derive(Debug, Clone)]
#[allow(dead_code)]
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
#[allow(dead_code)]
pub struct Edge {
    pub source: usize,
    pub target: usize,
}