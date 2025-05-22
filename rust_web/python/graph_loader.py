import json
import pyzx as zx
def main():
    graph = load_graph()
    print(graph)

def load_graph(path  : str = "xxx_final.json"):
    # Load graph from JSON file
    with open(path, 'r') as f:
        data = json.load(f)
    # Initialize PyZX graph
    graph = zx.Graph()
    id_map = {}  # Maps JSON node IDs to PyZX vertex indices
    xcods = set()
    ycods = set()
    for node, dets in data['wire_vertices'].items():
        coord = dets["annotation"]["coord"]
        xcods.add(coord[0])
        ycods.add(coord[1])
    for node, dets in data['node_vertices'].items():
        coord = dets["annotation"]["coord"]
        xcods.add(coord[0])
        ycods.add(coord[1])
    x_list = sorted(xcods)
    y_list = sorted(ycods)
    x_cood_map = {}
    for n, x in enumerate(x_list): x_cood_map[x] = n
    y_cood_map = {}
    for n, y in enumerate(y_list): y_cood_map[y] = n
    # boundary vertices
    for node, dets in data['wire_vertices'].items():
        coord = dets["annotation"]["coord"]
        vid = graph.add_vertex(row=x_cood_map[coord[0]], qubit=y_cood_map[coord[1]],)
        id_map[node] = vid
        graph.set_vdata(vid, 'label', node)
    # actual vertices
    for node, dets in data['node_vertices'].items():
        coord = dets["annotation"]["coord"]
        # v_val = dets["data"]["value"]
        v_val = 0
        if "value" in dets["data"].keys():
            v_val = dets["data"]["value"]
        if dets["data"]["type"] == "X": 
            v_type = zx.VertexType.X
        elif dets["data"]["type"] == "Z": v_type = zx.VertexType.Z
        else: v_type = zx.VertexType.H_BOX
        vid = graph.add_vertex(ty = v_type, row=x_cood_map[coord[0]], qubit=y_cood_map[coord[1]], phase=v_val)
        id_map[node] = vid
        graph.set_vdata(vid, 'label', node)

    # edges
    for edge, dets in data['undir_edges'].items():
        src = dets["src"]
        tgt = dets["tgt"]
        # e_type = zx.EdgeType.HADAMARD if dets['type'] == 'hadamard' else zx.EdgeType.SIMPLE
        graph.add_edge((id_map[src],id_map[tgt]))#, e_type)

    return graph

if __name__ == "__main__":
    main()
