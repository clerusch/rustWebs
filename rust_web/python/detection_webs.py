import pyzx as zx 
from lib.graph_loader import load_graph
import networkx as nx
from f2linalg.f2linalg import Mat2
import numpy as np
from pyzx.pauliweb import PauliWeb
from typing import Dict, Tuple, List

def ordered_nodes(g:zx.Graph) -> Tuple[List[int],Dict[int, int]]:
    """
    This function exists to keep track of original node ordering vs matrix ordering
    """
    original = list(g.vertices())
    outputs = [v for v in original if v not in g.inputs() and v not in g.outputs()]
    vertices = outputs + [v for v in original if g.ty[v]!=0 and v not in outputs]
    index_map = {vertices.index(item): item for item in original if item in vertices}
    
    return vertices, index_map

def get_pw(index_map: Dict, v:np.array,g) -> PauliWeb:
    """
    Create Pauliweb on given graph according to firing kernel vector v.
    Takes:
        index_map:  
                Dictionary from graph ordering to matrix ordering that is sorted for
                input + output nodes at the beginning
        v:          
                Kernel vector of detection matrix, aka valid detection web firing vector
        g:
                ZX graph on which we want to find the Pauliweb
    Output:
        pw:         
                Pauliweb corresponding to the firing kernel vector

    """
    
    # due to choi-jamalkowski isomorphism we can treat all inputs+outputs as outputs
    # to a state graph
    outs = len(g.inputs())+len(g.outputs())

    pw = PauliWeb(g)
    red_edges = set()
    green_edges = set()

    for i in np.nonzero(v)[0]:
        node_color =  g.ty[index_map[i-outs]]
        for edge in g.edges():
            if index_map[i-outs] in edge:
                if node_color == 1:
                    # Since we have rg form, double triggers aren't possible
                    green_edges.add(edge)
                elif node_color == 2:
                    red_edges.add(edge)
    
    for e in red_edges:
        pw.add_edge(e,'Z')
    for e in green_edges:
        pw.add_edge(e,'X')

    return pw

def make_rg(oldg: zx.Graph) -> zx.Graph:
    """
    Inplace rg-form transformation of a zx diagram
    I know this code looks weird, but don't touch it, it works
    """
    # As opposed to .copy(), .clone() preserves node naming
    g = oldg.clone()
    for node in g.vertices():
        nodecolor = g.ty[node]
        
        for neighbor in g.neighbors(node):
            if g.ty[neighbor] == nodecolor:
                row = (g.row(node)+g.row(neighbor)) /2
                qubit = (g.qubit(node)+g.qubit(neighbor)) /2
                oldg.remove_edge((neighbor, node))
                new_vertex = oldg.add_vertex(ty=(nodecolor%2)+1, qubit=qubit, row=row)
                oldg.add_edge((node, new_vertex))
                oldg.add_edge((new_vertex, neighbor))
                g = oldg.clone()
    return oldg

def get_detection_webs(g:zx.Graph) -> List[PauliWeb]:
    """
    Compute the detection webs for the given graph.
    """
    make_rg(g)
    # Keep track of old ordering
    new_order, index_map = ordered_nodes(g)
    # This is mostly because zx graphs dont have a to_array() method for the adjacency
    ng = nx.Graph(g.edges())
    outs = len(g.inputs())+len(g.outputs())
    
    # See borghan's master thesis for this part
    N = nx.to_numpy_array(ng, nodelist=new_order, dtype=np.uint8)
    I_n = np.eye(outs, dtype=np.uint8)
    zeroblock = np.zeros((N.shape[1]-outs, outs), dtype=np.uint8)
    mdl = np.vstack((I_n, zeroblock))
    md = Mat2(np.hstack((mdl, N)))
    
    # adds a stack of single-entry rows to eliminate outputs of the graphs firing vector
    no_output = np.hstack((np.eye(2*outs, dtype=np.uint8), np.zeros((2*outs, len(md.data[0])-2*outs), dtype=np.uint8)))
    md_no_output = Mat2(np.vstack((md.data, no_output)))
    mdnons = np.hstack([np.array(vec.data)for vec in md_no_output.nullspace()])
    
    pws = []
    for v in mdnons.T:
        pw = get_pw(index_map, v,g)
        pws.append(pw)

    return pws

def main():
    test_g4 = load_graph("zxgs/2_rounds_steane_rg.zxg")
    test_g4.set_inputs([19,20,21,17,16,15,14])
    test_g4.set_outputs([65,73,71,62,60,72,67])
    pws = get_detection_webs(test_g4)
    print(pws)

if __name__ == '__main__':
    main()