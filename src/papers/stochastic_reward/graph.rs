use crate::weightedbigraph::WBigraph;

use self::algorithm::AdaptiveAlgorithm;

type OfflineInfo<Reward> = Vec<Reward>;
type Prob = f64;

impl<Key> WBigraph<Key, Prob> {
    pub fn into_stochastic_reward<Reward>(
        self: Self,
        rewards: Vec<Reward>,
    ) -> StochasticReward<Key, Reward> {
        assert!(rewards.len() == self.u_nodes.len());
        for edge in self.nodes_edges.iter() {
            let prob = edge.1;
            assert!(
                prob >= 0. && prob <= 1.,
                "prob = {}, Probility should be in [0, 1]",
                prob
            )
        }
        StochasticReward {
            online_rewards: rewards,
            weighted_bigraph: self,
        }
    }
}

#[derive(Debug)]
pub struct StochasticReward<Key, Reword> {
    pub online_rewards: Vec<Reword>,
    pub weighted_bigraph: WBigraph<Key, Prob>,
}

pub mod algorithm {
    pub trait AdaptiveAlgorithm<T, OfflineInfo>
    where
        Self: Sized,
    {
        fn init(offline_info: OfflineInfo) -> Self;

        fn dispatch(self: &mut Self, online_adjacent: &Vec<T>) -> Option<usize>;

        fn query_success(self: &mut Self, offline_node: Option<usize>) -> bool;

        fn alg_output(self: Self) -> f64;
    }
}

impl<'a, Key, Reward: Into<f64> + Copy> StochasticReward<Key, Reward> {
    pub fn iter(self: &'a Self) -> StochasticRewardIter<'a> {
        StochasticRewardIter {
            online_adjacency_list: &self.weighted_bigraph.v_adjacency_list,
            online_index: 0,
        }
    }

    #[allow(non_snake_case)]
    pub fn OPT(self: &Self) -> f64 {
        // temporary unsound
        self.online_rewards.iter().map(|&r| r.into()).sum()
    }

    #[allow(non_snake_case)]
    pub fn ALG<Alg: AdaptiveAlgorithm<(usize, Prob), OfflineInfo<Reward>>>(self: &Self) -> f64 {
        let mut alg = Alg::init(self.online_rewards.clone());
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
