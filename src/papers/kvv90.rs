use crate::papers::util;
use rand::{distributions::Uniform, thread_rng, Rng};

use super::algorithm::algorithm::OnlineAlgorithm;

type OfflineInfo = usize;

pub struct Random {
    offline_nodes_available: Vec<bool>,
    pub alg: usize,
}

impl OnlineAlgorithm<usize, OfflineInfo> for Random {
    fn init(offline_size: usize) -> Self {
        let mut vec = Vec::with_capacity(offline_size);
        vec.resize(offline_size, true);
        Random {
            offline_nodes_available: vec,
            alg: 0,
        }
    }

    fn dispatch(self: &mut Self, online_adjacent: &Vec<usize>) -> Option<usize> {
        let available_offline_nodes = util::get_available_offline_nodes_in_onlineadj(
            &self.offline_nodes_available,
            online_adjacent,
        );
        if available_offline_nodes.is_empty() {
            None
        } else {
            let mut rng = thread_rng();
            let index: usize = rng.sample(Uniform::new(0, available_offline_nodes.len()));
            self.alg += 1;
            self.offline_nodes_available[available_offline_nodes[index]] = false;
            Some(available_offline_nodes[index])
        }
    }

    // This should drop / move all the algotithm cause after output
    // It can't be used anymore.
    fn alg_output(self: Self) -> f64 {
        self.alg as f64
    }
}

pub struct Ranking {
    offline_nodes_available: Vec<bool>,
    offline_nodes_rank: Vec<i32>,
    alg: usize,
}

impl OnlineAlgorithm<usize, OfflineInfo> for Ranking {
    fn init(offline_size: usize) -> Self {
        use rand::seq::SliceRandom;
        let mut off_available = Vec::with_capacity(offline_size);
        off_available.resize(offline_size, true);
        let mut rank = Vec::with_capacity(offline_size);
        for i in 0..offline_size {
            rank.push(i as i32)
        }
        rank.shuffle(&mut thread_rng());
        Ranking {
            offline_nodes_available: off_available,
            offline_nodes_rank: rank,
            alg: 0,
        }
    }

    fn dispatch(self: &mut Self, online_adjacent: &Vec<usize>) -> Option<usize> {
        let available_offline_nodes = util::get_available_offline_nodes_in_onlineadj(
            &self.offline_nodes_available,
            online_adjacent,
        );
        if available_offline_nodes.is_empty() {
            None
        } else {
            let mut min = i32::MAX;
            let mut index = None;
            for &off_node in available_offline_nodes.iter() {
                let off_node_rank = self.offline_nodes_rank[off_node];
                if off_node_rank < min {
                    min = off_node_rank;
                    index = Some(off_node);
                }
            }

            self.alg += 1;
            self.offline_nodes_available[index.unwrap()] = false;
            index
        }
    }

    fn alg_output(self: Self) -> f64 {
        self.alg as f64
    }
}

pub mod example {
    use crate::{bigraph::Bigraph, papers::algorithm::OnlineAdversarialBigraph};

    /// N means the size of Graph, |U| = |V| = 2 * N
    /// This is a “blown-up” version of the simple
    /// 2 × 2 example on the left.
    /// Each side of the bipartition has n vertices
    /// divided into two parts of size n/2 each (U = U1 \cup U2 and V = V1 \cup V2 )
    /// There is a perfect matching between U and V
    /// (the i'th vertex in U and V have an edge between them).
    /// There is also a bipartite clique between V1 and U2 .
    /// It can be shown that Random achieves a **ratio** of 1/2 + o(1)
    pub fn random_worst_case(n: usize) -> OnlineAdversarialBigraph<usize> {
        let mut edges = Vec::new();
        for i in 0..(2 * n) {
            edges.push((i, i))
        }
        for v in 0..n {
            for u in n..(2 * n) {
                edges.push((u, v))
            }
        }
        Bigraph::from_edges(&edges).into_online()
    }

    /// N means the size of Graph, |U| = |V| = n
    /// The graph means for all i, $v_i$ connected
    /// with $\{ u_i, u_{i+1}, ..., u_n \}$
    /// and the ratio with any algorithm is (1 - 1 / e)
    /// when lim n \to \inf
    pub fn ranking_worst_case(n: usize) -> OnlineAdversarialBigraph<usize> {
        let mut edges = Vec::new();
        for v in 0..n {
            for u in v..n {
                edges.push((u, v));
            }
        }
        Bigraph::from_edges(&edges).into_online()
    }
}
