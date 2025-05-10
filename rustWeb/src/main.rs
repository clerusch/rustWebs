mod test_fusion;
mod tikz_export;
use std::env;
fn main(){
    println!("Running from: {:?}", env::current_dir().unwrap());
    // test_fusion::test_fusion();
    let g = test_fusion::create_chain(10);
    tikz_export::export_to_tikz(&g, "../../masterarbeit/figures/graph.tex");
}