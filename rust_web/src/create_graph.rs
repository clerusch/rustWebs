use quizx::vec_graph::VType;
use quizx::graph::VData;
use quizx::{graph::GraphLike, hash_graph::Graph};
use num::rational::Rational64;

// Creates a vertex with Z-type and given position data

fn create_spider_vertex(g: &mut Graph, ty: VType, phase: Rational64, qubit: f64, row: f64) -> usize {
    g.add_vertex_with_data(VData {
        ty,
        phase: phase.into(),
        qubit,
        row,
    })
}

/// Creates a chain of spiders of the same type with optional boundary nodes
pub fn create_spider_chain(n: usize, spider_type: VType, with_phases: bool, with_boundaries: bool) -> Graph {
    let mut g = Graph::new();
    let mut prev = if with_boundaries {
        g.add_vertex_with_data(VData {
            ty: VType::B,
            phase: Rational64::new(0, 1).into(),
            qubit: 0.0,
            row: 0.0,
        })
    } else {
        create_spider_vertex(
            &mut g,
            spider_type,
            if with_phases { Rational64::new(1, 4) } else { Rational64::new(0, 1) },
            0.0,
            0.0,
        )
    };

    // Create chain of spiders
    for i in 1..n {
        let phase = if with_phases {
            Rational64::new(i as i64, (i + 1) as i64)  // Creates different phases for each spider
        } else {
            Rational64::new(0, 1)
        };
        
        let current = create_spider_vertex(&mut g, spider_type, phase, i as f64, 0.0);
        g.add_edge(prev, current);
        prev = current;
    }

    // Add final boundary node if requested
    if with_boundaries {
        let final_node = g.add_vertex_with_data(VData {
            ty: VType::B,
            phase: Rational64::new(0, 1).into(),
            qubit: n as f64,
            row: 0.0,
        });
        g.add_edge(prev, final_node);
    }

    g
}

pub fn create_chain(n: i32)->Graph {
    let mut g = Graph::new();

    // Create the first node
    let first = g.add_vertex(VType::X);
    let mut prev = first;

    // Add 999 more nodes and chain them
    for _ in 0..n {
        let current = g.add_vertex(VType::X);
        g.add_edge(prev, current);
        prev = current;
    }
    return g;
}