// Core modules
pub mod tikz_export;
pub mod create_graph;
pub mod graph_loader;
pub mod graph_visualizer;
pub mod pauliweb;
pub mod make_rg;
pub mod detection_webs;
pub mod bitwisef2linalg;

// Re-export detection_web function from the binary target
// pub use use_detection_webs::use_det_web;
// pub use detection_webs::DetectionWebs;
pub use graph_visualizer::draw_graph_with_pauliweb;
pub use pauliweb::PauliWeb;
pub use graph_loader::load_graph;
pub use quizx::hash_graph::Graph;
pub use quizx::graph::GraphLike;
