use crate::papers::adwords::util::get_available_offline_nodes_in_weighted_onlineadj;

use super::graph::algorithm::AdaptiveAlgorithm;
use super::graph::OfflineInfo;
use super::graph::Prob;
use rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct Ranking {
    offline_nodes_available: Vec<bool>,
    offline_nodes_rank: Vec<i32>,
}

impl AdaptiveAlgorithm<(usize, Prob), OfflineInfo> for Ranking {
    fn init(lenth: OfflineInfo) -> Self {
        use rand::seq::SliceRandom;
        let mut off_available = Vec::with_capacity(lenth);
        off_available.resize(lenth, true);
        let mut rank = Vec::with_capacity(lenth);
        for i in 0..lenth {
            rank.push(i as i32)
        }
        rank.shuffle(&mut thread_rng());
        Ranking {
            offline_nodes_available: off_available,
            offline_nodes_rank: rank,
        }
    }

    fn dispatch(
        self: &mut Self,
        online_adjacent: &Vec<(usize, Prob)>,
    ) -> Option<(usize, super::graph::Prob)> {
        let available_offline_nodes = get_available_offline_nodes_in_weighted_onlineadj(
            &self.offline_nodes_available,
            online_adjacent,
        );
        if available_offline_nodes.is_empty() {
            None
        } else {
            let mut min = i32::MAX;
            let mut index = None;
            for &off_node in available_offline_nodes.iter() {
                let off_node_rank = self.offline_nodes_rank[off_node.0];
                if off_node_rank < min {
                    min = off_node_rank;
                    index = Some(off_node);
                }
            }
            index
        }
    }

    fn query_success(
        self: &mut Self,
        offline_node: Option<(usize, super::graph::Prob)>,
    ) -> Option<bool> {
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
