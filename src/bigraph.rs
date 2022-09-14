use std::collections::BTreeMap;

type Edge<Key> = (Key, Key);

pub struct Bigraph<Key> {
    pub online_nodes: Vec<Key>,
    pub offline_nodes: Vec<Key>,
    pub nodes_edges: Vec<Edge<Key>>,
    nodes_edges_use_index: Vec<(usize, usize)>,
    online_key2index: BTreeMap<Key, usize>,
    offline_key2index: BTreeMap<Key, usize>,
    online_adjacency_list: Vec<Vec<usize>>,
    offline_adjacency_list: Vec<Vec<usize>>,
}

impl<Key: Ord + Copy + std::fmt::Debug> Bigraph<Key> {
    pub fn new() -> Bigraph<Key> {
        Bigraph {
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
                "edges should't contain the same edge: {:?}",
                edge
            );
            let (offline, online) = edge;

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

    pub fn insert_offline(self: &mut Self, key: Key) -> Result<(), String> {
        if self.offline_nodes.contains(&key) {
            Err("The offline nodes already have this key".to_owned())
        } else {
            let offline_index = self.offline_nodes.len();
            self.offline_nodes.push(key);
            self.offline_adjacency_list.push(vec![]);
            self.offline_key2index.insert(key, offline_index);
            Ok(())
        }
    }

    pub fn insert_online(self: &mut Self, key: Key) -> Result<(), String> {
        if self.online_nodes.contains(&key) {
            Err("The online nodes already have this key".to_owned())
        } else {
            let online_index = self.online_nodes.len();
            self.online_nodes.push(key);
            self.online_adjacency_list.push(vec![]);
            self.online_key2index.insert(key, online_index);
            Ok(())
        }
    }

    pub fn into_online(self: Self) -> OnlineAdversarialBigraph<Key> {
        let offline_size = self.offline_nodes.len();
        let mut vec = Vec::with_capacity(offline_size);
        vec.resize(offline_size, true);
        OnlineAdversarialBigraph {
            bigraph: self,
            // offline_nodes_available: vec,
        }
    }
}

pub trait Algorithm
where
    Self: Sized,
{
    fn init(offline_size: usize) -> Self;

    fn dispatch(self: &mut Self, online_adjacent: &Vec<usize>) -> Option<usize>;

    fn alg_output(self: Self) -> f64;
}

pub struct OnlineAdversarialBigraph<Key> {
    bigraph: Bigraph<Key>,
    // TODO
    // offline_nodes_available: Vec<bool>,
}

impl<'a, Key> OnlineAdversarialBigraph<Key> {
    pub fn iter(self: &'a Self) -> OnlineAdversarialBigraphIter<'a> {
        OnlineAdversarialBigraphIter {
            online_adjacency_list: &self.bigraph.online_adjacency_list,
            online_index: 0,
        }
    }

    #[allow(non_snake_case)]
    pub fn OPT(self: &Self) -> f64 {
        // temporary unsound
        self.bigraph.offline_nodes.len() as f64
    }

    #[allow(non_snake_case)]
    pub fn ALG<Alg: Algorithm>(self: &Self) -> f64 {
        let mut alg = Alg::init(self.bigraph.offline_nodes.len());
        for online_adj in self.iter() {
            // println!("{:?}", online_adj);
            alg.dispatch(online_adj);
        }
        alg.alg_output()
    }
}

pub struct OnlineAdversarialBigraphIter<'a> {
    online_adjacency_list: &'a Vec<Vec<usize>>,
    online_index: usize,
}

impl<'a> Iterator for OnlineAdversarialBigraphIter<'a> {
    type Item = &'a Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.online_index == self.online_adjacency_list.len() {
            None
        } else {
            let t = Some(&self.online_adjacency_list[self.online_index]);
            self.online_index += 1;
            t
        }
    }
}
