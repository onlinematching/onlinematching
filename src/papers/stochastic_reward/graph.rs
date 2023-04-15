use crate::weightedbigraph::WBigraph;

use self::algorithm::AdaptiveAlgorithm;

pub type OfflineInfo = usize;
pub type Prob = f64;

impl<Key> WBigraph<Key, Prob> {
    pub fn into_stochastic_reward(self: Self) -> StochasticReward<Key> {
        for edge in self.nodes_edges.iter() {
            let prob = edge.1;
            assert!(
                prob >= 0. && prob <= 1.,
                "prob = {}, Probility should be in [0, 1]",
                prob
            )
        }
        StochasticReward {
            weighted_bigraph: self,
        }
    }
}

#[derive(Debug)]
pub struct StochasticReward<Key> {
    pub weighted_bigraph: WBigraph<Key, Prob>,
}

pub mod algorithm {
    pub trait AdaptiveAlgorithm<AdjType, OfflineInfo>
    where
        Self: Sized,
    {
        fn init(lenth: OfflineInfo) -> Self;

        fn dispatch(self: &mut Self, online_adjacent: &Vec<AdjType>) -> Option<usize>;

        fn query_success(self: &mut Self, offline_node: Option<usize>) -> bool;

        fn alg_output(self: Self) -> f64;
    }
}

impl<'a, Key> StochasticReward<Key> {
    pub fn iter(self: &'a Self) -> StochasticRewardIter<'a> {
        StochasticRewardIter {
            online_adjacency_list: &self.weighted_bigraph.v_adjacency_list,
            online_index: 0,
        }
    }

    #[allow(non_snake_case)]
    pub fn OPT(self: &Self) -> f64 {
        // temporary unsound
        self.weighted_bigraph.u_nodes.len() as f64
    }

    #[allow(non_snake_case)]
    pub fn ALG<Alg: AdaptiveAlgorithm<(usize, Prob), OfflineInfo>>(self: &Self) -> f64 {
        let mut alg = Alg::init(self.weighted_bigraph.u_nodes.len());
        for online_adj in self.iter() {
            let _alg_choose = alg.dispatch(online_adj);
        }
        alg.alg_output()
    }
}

pub struct StochasticRewardIter<'a> {
    pub online_adjacency_list: &'a Vec<Vec<(usize, Prob)>>,
    pub online_index: usize,
}

impl<'a> Iterator for StochasticRewardIter<'a> {
    type Item = &'a Vec<(usize, Prob)>;

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
