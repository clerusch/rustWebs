
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum NodeType {
    ZSpider(f64),
    XSpider(f64),
    Boundary,
}
#[derive(Debug, Clone)]
pub struct Node {
    pub id: usize,
    pub node_type: NodeType,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Edge {
    pub source: usize,
    pub target: usize,
}