use std::collections::BTreeMap;

type Edge<Key> = (Key, Key);

#[derive(Debug, PartialEq)]
pub struct WBigraph<Key, Weight> {
    pub v_nodes: Vec<Key>,
    pub u_nodes: Vec<Key>,
    pub nodes_edges: Vec<(Edge<Key>, Weight)>,
    nodes_edges_use_index: Vec<((usize, usize), Weight)>,
    v_key2index: BTreeMap<Key, usize>,
    u_key2index: BTreeMap<Key, usize>,
    pub v_adjacency_list: Vec<Vec<(usize, Weight)>>,
    pub u_adjacency_list: Vec<Vec<(usize, Weight)>>,
}

impl<Key, Weight> WBigraph<Key, Weight>
where
    Key: Ord + Copy + std::fmt::Debug,
    Weight: Ord + Copy + std::fmt::Debug,
{
    pub fn new() -> WBigraph<Key, Weight> {
        WBigraph {
            v_nodes: vec![],
            u_nodes: vec![],
            nodes_edges: vec![],
            nodes_edges_use_index: vec![],
            v_key2index: BTreeMap::new(),
            u_key2index: BTreeMap::new(),
            v_adjacency_list: vec![],
            u_adjacency_list: vec![],
        }
    }

    pub fn from_edges(edges: &Vec<(Edge<Key>, Weight)>) -> Self {
        let mut graph = Self::new();
        for edge in edges {
            assert!(
                !graph.nodes_edges.contains(edge),
                "edges should't contain the same edge: {:?}",
                edge
            );
            let ((ref u, ref v), w) = edge;
            let w = w.clone();

            let v_index;
            if !graph.v_nodes.contains(v) {
                v_index = graph.v_nodes.len();
                graph.v_key2index.insert(v.clone(), v_index);
                graph.v_nodes.push(v.clone());
                graph.v_adjacency_list.push(vec![]);
            } else {
                v_index = graph.v_key2index[v];
            }

            let u_index;
            if !graph.u_nodes.contains(u) {
                u_index = graph.u_nodes.len();
                graph.u_key2index.insert(u.clone(), u_index);
                graph.u_nodes.push(u.clone());
                graph.u_adjacency_list.push(vec![]);
            } else {
                u_index = graph.u_key2index[u];
            }

            graph.nodes_edges.push(edge.clone());
            graph.nodes_edges_use_index.push(((u_index, v_index), w));

            graph.v_adjacency_list[v_index].push((u_index, w));
            graph.u_adjacency_list[u_index].push((v_index, w));
        }
        graph
    }

    pub fn insert_u(self: &mut Self, key: Key) -> Result<(), String> {
        if self.u_nodes.contains(&key) {
            Err("The u nodes already have this key".to_owned())
        } else {
            let u_index = self.u_nodes.len();
            self.u_nodes.push(key);
            self.u_adjacency_list.push(vec![]);
            self.u_key2index.insert(key, u_index);
            Ok(())
        }
    }

    pub fn insert_v(self: &mut Self, key: Key) -> Result<(), String> {
        if self.v_nodes.contains(&key) {
            Err("The v nodes already have this key".to_owned())
        } else {
            let v_index = self.v_nodes.len();
            self.v_nodes.push(key);
            self.v_adjacency_list.push(vec![]);
            self.v_key2index.insert(key, v_index);
            Ok(())
        }
    }
}
