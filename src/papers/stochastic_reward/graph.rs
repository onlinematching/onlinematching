use crate::{papers::algorithm::algorithm::OnlineAlgorithm, weightedbigraph::WBigraph};

type OfflineInfo<Prob> = Vec<Prob>;
type Prob = f64;

impl<Key> WBigraph<Key, f64> {
    pub fn into__online<Reward>(self: Self, rewards: Vec<Reward>) -> StochasticReward<Key, Reward> {
        assert_eq!(
            rewards.len(),
            self.u_nodes.len() // format!("budget = {:?},\n  graph = {:?}\n", budget, self)
        );
        StochasticReward {
            online_rewards: rewards,
            weighted_bigraph: self,
        }
    }
}

#[derive(Debug)]
pub struct StochasticReward<Key, Reword> {
    pub online_rewards: Vec<Reword>,
    pub weighted_bigraph: WBigraph<Key, f64>,
}
