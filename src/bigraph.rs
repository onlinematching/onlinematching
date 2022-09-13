use std::collections::BTreeMap;
use std::hash::{self, Hash};
use std::marker::PhantomData;

type Edge<Key> = (Key, Key);

pub struct bigraph<Key> {
    online_nodes: Vec<Key>,
    offline_nodes: Vec<Key>,
    nodes_edges: Vec<Edge<Key>>,
    nodes_edges_use_index: Vec<(usize, usize)>,
    online_key2index: BTreeMap<Key, usize>,
    offline_key2index: BTreeMap<Key, usize>,
    online_adjacency_list: Vec<Vec<usize>>,
    offline_adjacency_list: Vec<Vec<usize>>,
}

impl<Key: Ord + Copy> bigraph<Key> {
    pub fn new() -> bigraph<Key> {
        bigraph {
            online_nodes: vec![],
            offline_nodes: vec![],
            nodes_edges: vec![],
            nodes_edges_use_index: vec![],
            online_key2index: BTreeMap::new(),
            offline_key2index: BTreeMap::new(),
            online_adjacency_list: vec![],
            offline_adjacency_list: vec![],
        }
    }

    pub fn from_edges(edges: &Vec<Edge<Key>>) -> Self {
        let mut graph = Self::new();
        for edge in edges {
            assert!(
                !graph.nodes_edges.contains(edge),
                "edges should't contain the same edge"
            );
            let (online, offline) = edge;

            let online_index;
            // It means a new online node has arrived
            // so the adjacency_list and nodes list should be increased
            if !graph.online_nodes.contains(online) {
                online_index = graph.online_nodes.len();
                graph.online_key2index.insert(online.clone(), online_index);
                graph.online_nodes.push(online.clone());
                graph.online_adjacency_list.push(vec![]);
            } else {
                online_index = graph.online_key2index[online];
            }

            // exactly the same as above
            let offline_index;
            if !graph.offline_nodes.contains(offline) {
                offline_index = graph.offline_nodes.len();
                graph
                    .offline_key2index
                    .insert(offline.clone(), offline_index);
                graph.offline_nodes.push(offline.clone());
                graph.offline_adjacency_list.push(vec![]);
            } else {
                offline_index = graph.offline_key2index[offline];
            }

            graph.nodes_edges.push(edge.clone());
            graph
                .nodes_edges_use_index
                .push((online_index, offline_index));

            graph.online_adjacency_list[online_index].push(offline_index);
            graph.offline_adjacency_list[offline_index].push(online_index);
        }
        graph
    }

}
