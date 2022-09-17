use crate::bigraph::Bigraph;

use self::algorithm::Algorithm;

impl<Key> Bigraph<Key> {
    pub fn into_reuseable_online(self: Self, duration: usize) -> OnlineAdversarialBigraph<Key> {
        OnlineAdversarialBigraph {
            bigraph: self,
            duration,
            opt: None,
        }
    }
}

pub struct OnlineAdversarialBigraph<Key> {
    bigraph: Bigraph<Key>,
    duration: usize,
    opt: Option<f64>,
}

pub struct OnlineAdversarialBigraphIter<'a> {
    online_adjacency_list: &'a Vec<Vec<usize>>,
    online_index: usize,
}

impl<'a, Key> OnlineAdversarialBigraph<Key> {
    pub fn iter(self: &'a Self) -> OnlineAdversarialBigraphIter<'a> {
        OnlineAdversarialBigraphIter {
            online_adjacency_list: &self.bigraph.online_adjacency_list,
            online_index: 0,
        }
    }
}

impl<'a, Key> OnlineAdversarialBigraph<Key> {
    #[allow(non_snake_case)]
    pub fn OPT(self: &Self) -> f64 {
        // temporary unsound
        if let Some(opt) = self.opt {
            return opt;
        }
        self.bigraph.online_nodes.len() as f64
    }

    #[allow(non_snake_case)]
    pub fn ALG<Alg: Algorithm>(self: &Self) -> f64 {
        let mut alg = Alg::init(self.bigraph.offline_nodes.len(), self.duration);
        for online_adj in self.iter() {
            // println!("{:?}", online_adj);
            let alg_choose = alg.dispatch(online_adj);
            drop(alg_choose);
        }
        alg.alg_output()
    }
}

impl<'a> Iterator for OnlineAdversarialBigraphIter<'a> {
    type Item = &'a Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.online_index == self.online_adjacency_list.len() {
            None
        } else {
            self.online_index += 1;
            Some(&self.online_adjacency_list[self.online_index - 1])
        }
    }
}

pub mod algorithm {

    pub trait Algorithm
    where
        Self: Sized,
    {
        fn init(offline_size: usize, duration: usize) -> Self;

        fn dispatch(self: &mut Self, online_adjacent: &Vec<usize>) -> Option<usize>;

        fn alg_output(self: Self) -> f64;
    }
    use rand::thread_rng;

    pub struct Ranking {
        offline_nodes_available: Vec<i32>,
        offline_nodes_rank: Vec<i32>,
        alg: usize,
        duration: usize,
    }

    impl Algorithm for Ranking {
        fn init(offline_size: usize, duration: usize) -> Self {
            let mut vec = Vec::with_capacity(offline_size);
            vec.resize(offline_size, 0);
            use rand::seq::SliceRandom;
            let mut off_available = Vec::with_capacity(offline_size);
            off_available.resize(offline_size, true);
            let mut offline_nodes_rank = Vec::with_capacity(offline_size);
            for i in 0..offline_size {
                offline_nodes_rank.push(i as i32)
            }
            offline_nodes_rank.shuffle(&mut thread_rng());
            Ranking {
                offline_nodes_available: vec,
                offline_nodes_rank,
                alg: 0,
                duration,
            }
        }

        fn dispatch(self: &mut Self, online_adjacent: &Vec<usize>) -> Option<usize> {
            let mut available_nodes = Vec::with_capacity(online_adjacent.len());
            for &offline_node in online_adjacent.iter() {
                if self.offline_nodes_available[offline_node] == 0 {
                    available_nodes.push(offline_node);
                }
            }
            let ans;
            if available_nodes.is_empty() {
                ans = None;
            } else {
                let mut min = i32::MAX;
                let mut index = None;
                for &node in available_nodes.iter() {
                    let node_rank = self.offline_nodes_rank[node];
                    if node_rank < min {
                        min = node_rank;
                        index = Some(node);
                    }
                }

                self.alg += 1;
                self.offline_nodes_available[index.unwrap()] = self.duration as i32;
                ans = index;
            }
            for i in 0..self.offline_nodes_available.len() {
                if self.offline_nodes_available[i] != 0 {
                    self.offline_nodes_available[i] -= 1;
                }
            }
            ans
        }

        fn alg_output(self: Self) -> f64 {
            self.alg as f64
        }
    }
}

pub mod example {
    use super::OnlineAdversarialBigraph;
    use crate::papers::reuseableresource::identical::Bigraph;

    pub fn z_graph_with_duration(n: usize, d: usize) -> OnlineAdversarialBigraph<usize> {
        let mut edges = Vec::new();
        for v in 0..n {
            for u in v..n {
                edges.push((u, v));
            }
        }
        Bigraph::from_edges(&edges).into_reuseable_online(d)
    }
}
