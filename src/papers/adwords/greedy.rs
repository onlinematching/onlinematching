use crate::papers::algorithm::algorithm::OnlineAlgorithm;
use crate::weightedbigraph::WBigraph;
use rand::thread_rng;
use std::default::Default;

type OfflineInfo<Weight> = Vec<Weight>;

pub struct Greddy<Weight> {
    offline_nodes_budgets: Vec<Weight>,
    offline_nodes_available: Vec<bool>,
    offline_nodes_loads: Vec<Weight>,
    alg: Weight,
}

pub fn get_available_offline_nodes_in_weighted_onlineadj<Weight: Copy>(
    offline_nodes_available: &Vec<bool>,
    online_adjacent: &Vec<(usize, Weight)>,
) -> Vec<(usize, Weight)> {
    let mut vec = Vec::with_capacity(online_adjacent.len());
    for (offline_node, w) in online_adjacent.iter() {
        if offline_nodes_available[*offline_node] {
            vec.push((*offline_node, *w))
        }
    }
    vec
}

impl<Weight> OnlineAlgorithm<(usize, Weight), OfflineInfo<Weight>> for Greddy<Weight>
where
    Weight: Default + Into<f64> + Copy + std::cmp::PartialOrd + std::ops::AddAssign,
{
    fn init(offline_info: OfflineInfo<Weight>) -> Self {
        let l = offline_info.len();
        let offline_nodes_budgets = offline_info;
        let mut offline_nodes_loads: Vec<Weight> = Vec::with_capacity(l);
        let zero = Weight::default();
        offline_nodes_loads.resize(l, zero);
        let mut offline_nodes_available = Vec::with_capacity(l);
        offline_nodes_available.resize(l, true);
        Greddy {
            offline_nodes_budgets,
            offline_nodes_available,
            offline_nodes_loads,
            alg: zero,
        }
    }

    fn dispatch(self: &mut Self, online_adjacent: &Vec<(usize, Weight)>) -> Option<usize> {
        let available_offline_nodes = get_available_offline_nodes_in_weighted_onlineadj(
            &self.offline_nodes_available,
            online_adjacent,
        );
        let largest_offline_node = available_offline_nodes
            .iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        match largest_offline_node {
            Some(node) => {
                let i = node.0;
                let bid = node.1;
                let budget = self.offline_nodes_budgets[i];
                let mut load = self.offline_nodes_loads[i];
                load += bid;
                if load >= budget {
                    load = budget;
                    self.offline_nodes_available[i] = false;
                }
                self.offline_nodes_loads[i] = load;

                Some(i)
            }
            None => None,
        }
    }

    fn alg_output(self: Self) -> f64 {
        self.alg.into()
    }
}
