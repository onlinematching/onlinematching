use crate::papers::adwords::util::get_available_offline_nodes_in_weighted_onlineadj;

use super::graph::algorithm::AdaptiveAlgorithm;
use super::graph::OfflineInfo;
use super::graph::Prob;
use rand::Rng;

pub struct Balance {
    offline_nodes_available: Vec<bool>,
    offline_nodes_loads: Vec<Prob>,
}

pub fn f(p: Prob, l: f64) -> f64 {
    p * f64::exp(-l)
}

impl AdaptiveAlgorithm<(usize, Prob), OfflineInfo> for Balance {
    fn init(length: OfflineInfo) -> Self {
        let l = length;
        let mut offline_nodes_available = Vec::with_capacity(l);
        offline_nodes_available.resize(l, true);
        let mut offline_nodes_loads: Vec<Prob> = Vec::with_capacity(l);
        offline_nodes_loads.resize(l, 0.);
        Balance {
            offline_nodes_available,
            offline_nodes_loads,
        }
    }

    fn dispatch(self: &mut Self, online_adjacent: &Vec<(usize, Prob)>) -> Option<(usize, Prob)> {
        let available_offline_nodes = get_available_offline_nodes_in_weighted_onlineadj(
            &self.offline_nodes_available,
            online_adjacent,
        );
        let largest_offline_node = available_offline_nodes
            .iter()
            .map(|x| {
                let i = x.0;
                let prob = x.1;
                let load = self.offline_nodes_loads[i];
                (i, f(prob, load), prob)
            })
            .max_by(|u1, u2| u1.1.partial_cmp(&u2.1).unwrap())
            .map(|u| (u.0, u.2));

        match largest_offline_node {
            Some(node) => {
                self.offline_nodes_loads[node.0] += node.1;
                Some(node)
            }
            None => None,
        }
    }

    fn query_success(self: &mut Self, offline_node: Option<(usize, Prob)>) -> Option<bool> {
        match offline_node {
            Some(adj_info) => {
                let mut rng = rand::thread_rng();
                let prob = adj_info.1;
                let result = rng.gen_bool(prob);
                if result {
                    self.offline_nodes_available[adj_info.0] = false;
                }
                Some(result)
            }
            None => None,
        }
    }

    fn alg_output(self: Self) -> f64 {
        self.offline_nodes_available
            .iter()
            .map(|&avail| match avail {
                true => 0,
                false => 1,
            })
            .sum::<i32>() as f64
    }
}

pub mod example {
    use crate::{papers::stochastic_reward::graph::StochasticReward, weightedbigraph::WBigraph};

    pub fn simplist(m: usize) -> StochasticReward<usize> {
        assert!(m > 0);
        let p = 1. / m as f64;
        let mut edges = Vec::new();
        for v in 0..m {
            edges.push(((0, v), p));
        }
        let wbigraph = WBigraph::from_edges(&edges);
        wbigraph.into_stochastic_reward()
    }

    pub fn gk(k: usize, m: usize) -> StochasticReward<usize> {
        assert!(k > 0 && m > 0);
        let p = 1. / m as f64;
        let mut edges = Vec::new();
        for u in 0..k {
            for v in 0..(u + 1) * m {
                edges.push(((u, v), p));
            }
        }
        let wbigraph = WBigraph::from_edges(&edges);
        wbigraph.into_stochastic_reward()
    }
}
