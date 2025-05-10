use std::fmt::Write;
use super::structure::Graph;
use super::types::NodeType;

impl Graph {
    pub fn to_dot(&self) -> String {
        let mut output = String::from("graph ZX {\n");

        for node in self.nodes.values() {
            let label = match &node.node_type {
                NodeType::ZSpider(phase) => format!("Z\\n{:.2}", phase),
                NodeType::XSpider(phase) => format!("X\\n{:.2}", phase),
                NodeType::Boundary => "B".to_string(),
            };
            let _ = writeln!(
                output,
                "    {} [label=\"{}\", shape=circle];",
                node.id, label
            );
        }

        for ((a, b), _) in &self.edges {
            let _ = writeln!(output, "    {} -- {};", a, b);
        }

        output.push_str("}\n");
        output
    }
}
