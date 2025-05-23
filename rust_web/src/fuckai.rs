
pub fn ordered_nodes(g: &GraphLike) {
    let original = g.vertices().collect();
    let outputs = original.iter()
    .filter(|v| g.ty(*v) == VType::Output || g.ty(*v) == VType::Input)
    .collect();
    let vertices = outputs + original.iter();
    let index_map: HashMap<usize, usize> = original.iter()
    .filter(|&&item| vertices.contains(&item))
    .enumerate()
    .map(|(idx, &item)| (idx, item))
    .collect();
}

pub fn get_detection_webs(g: &mut GraphLike) {
    make_rg(g);
    let n_outs = g.inputs().len() + g.outputs().len();
    

}
    