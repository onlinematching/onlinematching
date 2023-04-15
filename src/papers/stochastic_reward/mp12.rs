use crate::papers::adwords::util::get_available_offline_nodes_in_weighted_onlineadj;

use super::graph::algorithm::AdaptiveAlgorithm;
use super::graph::OfflineInfo;
use super::graph::Prob;

pub struct Balance {
    offline_nodes_available: Vec<bool>,
    offline_nodes_loads: Vec<Prob>,
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

    fn dispatch(self: &mut Self, online_adjacent: &Vec<(usize, Prob)>) -> Option<usize> {
        let available_offline_nodes = get_available_offline_nodes_in_weighted_onlineadj(
            &self.offline_nodes_available,
            online_adjacent,
        );

        todo!()
    }

    fn query_success(self: &mut Self, offline_node: Option<usize>) -> bool {
        todo!()
    }

    fn alg_output(self: Self) -> f64 {
        todo!()
    }
}
