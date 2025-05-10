use std::fmt::Write;
use super::structure::Graph;
use super::types::NodeType;

impl Graph {
    pub fn to_dot(&self) -> String {
        let mut output = String::from("graph ZX {\n");

        for node in self.nodes.values() {
            let label = match node.node_type {
                NodeType::ZSpider => "Z",
                NodeType::XSpider => "X",
                NodeType::Boundary => "B",
            };
            let _ = writeln!(
                output,
                "    {} [label=\"{}\\n{:.2}\", shape=circle];",
                node.id, label, node.phase
            );
        }

        for ((a, b), _) in &self.edges {
            let _ = writeln!(output, "    {} -- {};", a, b);
        }

        output.push_str("}\n");
        output
    }
}
