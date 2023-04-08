use crate::papers::algorithm::algorithm::OnlineAlgorithm;
use std::default::Default;

use super::util::get_available_offline_nodes_in_weighted_onlineadj;

type OfflineInfo<Weight> = Vec<Weight>;

pub struct MSVV<Weight> {
    offline_nodes_budgets: Vec<Weight>,
    offline_nodes_available: Vec<bool>,
    offline_nodes_fraction: Vec<f64>,
}

pub fn f<Weight: Into<f64>>(bid: Weight, x: f64) -> f64 {
    bid.into() * (1.0 - f64::exp(x - 1.0))
}

impl<Weight> OnlineAlgorithm<(usize, Weight), OfflineInfo<Weight>> for MSVV<Weight>
where
    Weight: Default
        + Into<f64>
        + From<f64>
        + Copy
        + std::cmp::PartialOrd
        + std::ops::AddAssign
        + std::ops::Mul<Output = Weight>,
{
    fn init(offline_info: OfflineInfo<Weight>) -> Self {
        let l = offline_info.len();
        let offline_nodes_budgets = offline_info;
        let mut offline_nodes_fraction: Vec<f64> = Vec::with_capacity(l);
        offline_nodes_fraction.resize(l, 0.);
        let mut offline_nodes_available = Vec::with_capacity(l);
        offline_nodes_available.resize(l, true);
        MSVV {
            offline_nodes_budgets,
            offline_nodes_available,
            offline_nodes_fraction,
        }
    }

    fn dispatch(self: &mut Self, online_adjacent: &Vec<(usize, Weight)>) -> Option<usize> {
        let available_offline_nodes = get_available_offline_nodes_in_weighted_onlineadj(
            &self.offline_nodes_available,
            online_adjacent,
        );
        let largest_offline_node = available_offline_nodes
            .iter()
            .map(|x| {
                let i = x.0;
                let bid = x.1;
                let x = self.offline_nodes_fraction[i];
                (i, f(bid, x), bid)
            })
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .map(|x| (x.0, x.2));
        match largest_offline_node {
            Some(node) => {
                let i = node.0;
                let bid = node.1;
                let budget = self.offline_nodes_budgets[i];
                let mut load = Weight::from(self.offline_nodes_fraction[i]) * budget;
                load += bid;
                if load >= budget {
                    load = budget;
                    self.offline_nodes_available[i] = false;
                }
                self.offline_nodes_fraction[i] = load.into() / budget.into();

                Some(i)
            }
            None => None,
        }
    }

    fn alg_output(self: Self) -> f64 {
        let l = self.offline_nodes_available.len();
        assert_eq!(self.offline_nodes_budgets.len(), l);
        assert_eq!(self.offline_nodes_fraction.len(), l);
        let mut ans = 0.;
        for i in 0..l {
            ans += self.offline_nodes_fraction[i] * self.offline_nodes_budgets[i].into();
        }
        ans
    }
}
