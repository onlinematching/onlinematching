use crate::papers::algorithm::algorithm::OnlineAlgorithm;
use std::default::Default;

use super::util::get_available_offline_nodes_in_weighted_onlineadj;

type OfflineInfo<Weight> = Vec<Weight>;

#[derive(Debug)]
pub struct Greddy<Weight> {
    offline_nodes_budgets: Vec<Weight>,
    offline_nodes_available: Vec<bool>,
    offline_nodes_loads: Vec<Weight>,
}

impl<Weight> OnlineAlgorithm<(usize, Weight), OfflineInfo<Weight>> for Greddy<Weight>
where
    Weight:
        Default + Into<f64> + Copy + std::cmp::PartialOrd + std::ops::AddAssign + std::fmt::Debug,
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
        self.offline_nodes_loads
            .iter()
            .map(|&x| x.into())
            .sum::<f64>()
    }
}

pub mod example {
    use crate::{papers::adwords::adwords::AdversarialAdwords, weightedbigraph::WBigraph};
    pub fn greedy_worst_case(n: usize) -> AdversarialAdwords<usize, f64> {
        let mut edges = Vec::new();
        for v in 0..n {
            edges.push(((0, v), 0.99));
        }
        for i in 0..2 * n {
            edges.push(((1, i), 1.));
        }
        let wbigraph = WBigraph::from_edges(&edges);
        let mut budgets = Vec::new();
        budgets.push(n as f64);
        budgets.push(n as f64);
        wbigraph.into_adwords(budgets)
    }
}
