use rust_web::graph_loader::load_graph;
// use quizx::hash_graph::Graph;
use quizx::phase::Phase;
use quizx::graph::VType;
use quizx::graph::GraphLike;
// use quizx::fscalar::Zero;
use std::collections::HashSet;
use tempfile::tempdir;
use std::fs;

#[test]
fn test_load_graph_vertices() {
    // Create a temporary test JSON file
    let test_json = r#"
    {
        "wire_vertices": {
            "w1": {
                "annotation": { "coord": [0, 0] }
            },
            "w2": {
                "annotation": { "coord": [1, 0] }
            }
        },
        "node_vertices": {
            "n1": {
                "annotation": { "coord": [0, 1] },
                "data": {
                    "type": "X",
                    "value": 0.5
                }
            },
            "n2": {
                "annotation": { "coord": [1, 1] },
                "data": {
                    "type": "Z",
                    "value": 1.0
                }
            }
        },
        "undir_edges": {
            "e1": {
                "src": "w1",
                "tgt": "n1"
            },
            "e2": {
                "src": "n1",
                "tgt": "n2"
            },
            "e3": {
                "src": "n2",
                "tgt": "w2"
            }
        }
    }"#;

    // Create temporary file
    let temp_dir = tempdir().unwrap();
    let temp_file = temp_dir.path().join("test_graph.json");
    std::fs::write(&temp_file, test_json).unwrap();

    // Load the graph
    let graph = load_graph(temp_file.to_str().unwrap()).unwrap();

    // Verify vertices
    assert_eq!(graph.num_vertices(), 4);
    
    // Verify vertex types and phases
    for v in graph.vertices() {
        let data = graph.vertex_data(v);
        match data.ty {
            VType::X => assert_eq!(data.phase, Phase::from_f64(0.5)),
            VType::Z => assert_eq!(data.phase, Phase::from_f64(1.0)),
            _ => assert_eq!(data.phase, Phase::from_f64(0.0)),
        }
    }

    // Verify edges
    assert_eq!(graph.num_edges(), 3);
}

#[test]
fn test_load_graph_coordinates() {
    let test_json = r#"
    {
        "wire_vertices": {
            "w1": {
                "annotation": { "coord": [0, 0] }
            },
            "w2": {
                "annotation": { "coord": [2, 0] }
            }
        },
        "node_vertices": {
            "n1": {
                "annotation": { "coord": [1, 1] },
                "data": {
                    "type": "X",
                    "value": 0.0
                }
            }
        },
        "undir_edges": {
            "e1": {
                "src": "w1",
                "tgt": "n1"
            },
            "e2": {
                "src": "n1",
                "tgt": "w2"
            }
        }
    }"#;

    let temp_dir = tempfile::tempdir().unwrap();
    let temp_file = temp_dir.path().join("test_graph.json");
    fs::write(&temp_file, test_json).unwrap();

    let graph = load_graph(temp_file.to_str().unwrap());

    match graph {
        Ok(graph) => {
            // Verify coordinates are properly mapped
            let mut coords = HashSet::new();
            for v in graph.vertices() {
                let data = graph.vertex_data(v);
                coords.insert((data.row as i64, data.qubit as i64));
            }

            // Should have coordinates (0,0), (2,0), (1,1)
            assert_eq!(coords.len(), 3);
            assert!(coords.contains(&(0, 0)));
            assert!(coords.contains(&(2, 0)));
            assert!(coords.contains(&(1, 1)));
        },
        Err(_) => panic!("Failed to load graph"),
    }
}

#[test]
fn test_load_graph_edge_types() {
    let test_json = r#"
    {
        "wire_vertices": {
            "w1": {
                "annotation": { "coord": [0, 0] }
            },
            "w2": {
                "annotation": { "coord": [1, 0] }
            }
        },
        "node_vertices": {
            "n1": {
                "annotation": { "coord": [0, 1] },
                "data": {
                    "type": "X",
                    "value": 0.0
                }
            }
        },
        "undir_edges": {
            "e1": {
                "src": "w1",
                "tgt": "n1"
            },
            "e2": {
                "src": "n1",
                "tgt": "w2"
            }
        }
    }"#;

    let temp_dir = tempfile::tempdir().unwrap();
    let temp_file = temp_dir.path().join("test_graph.json");
    std::fs::write(&temp_file, test_json).unwrap();

    let graph = load_graph(temp_file.to_str().unwrap()).unwrap();

    // Verify edges
    assert_eq!(graph.num_edges(), 2);
    
    // Verify connectivity
    let mut edges = Vec::new();
    for (src, tgt, _) in graph.edge_vec() {
        edges.push((src, tgt));
    }

    assert_eq!(edges.len(), 2);
    // We can't predict exact vertex IDs, but we can verify the connectivity pattern
    assert!(edges.iter().any(|&(src, tgt)| src != tgt)); // Should have at least one edge between different vertices
    assert!(edges.iter().all(|&(src, tgt)| src != tgt)); // No self-loops
}

#[test]
#[should_panic(expected = "Missing or invalid node_vertices")]
fn test_load_graph_invalid_json() {
    // Test loading with invalid JSON
    let invalid_json = r#"{
        "wire_vertices": {
            "w1": {
                "annotation": { "coord": [0, 0] }
            }
        }
    }"#;
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_file = temp_dir.path().join("invalid.json");
    std::fs::write(&temp_file, invalid_json).unwrap();
    
    load_graph(temp_file.to_str().unwrap()).unwrap();
}

#[test]
fn test_from_file() {
    use std::fs;

    use rust_web::graph_loader;
    use rust_web::graph_visualizer;

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let path = std::path::Path::new(&manifest_dir)
        .join("src")
        .join("zxgs")
        .join("xxx_final.zxg");
    
    // Load the graph
    let g = graph_loader::load_graph(path.to_str().unwrap()).unwrap();
    
    // Generate and save the DOT representation with positions
    let dot_string = graph_visualizer::to_dot_with_positions(&g);
    fs::write("graph2.dot", dot_string).unwrap();
}