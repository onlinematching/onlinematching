use crate::{papers::algorithm::algorithm::OnlineAlgorithm, weightedbigraph::WBigraph};

type OfflineInfo<Prob> = Vec<Prob>;
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

impl<'a, Key, Reward> StochasticReward<Key, Reward> {
    pub fn iter(self: &'a Self) -> StochasticRewardIter<'a> {
        StochasticRewardIter {
            online_adjacency_list: &self.weighted_bigraph.v_adjacency_list,
            online_index: 0,
        }
    }
}

pub struct StochasticRewardIter<'a> {
    pub online_adjacency_list: &'a Vec<Vec<(usize, Prob)>>,
    pub online_index: usize,
}

