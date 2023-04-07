use crate::{papers::algorithm::algorithm::OnlineAlgorithm, weightedbigraph::WBigraph};

type OfflineInfo<Weight> = Vec<Weight>;

impl<Key, Weight> WBigraph<Key, Weight> {
    pub fn into_online(self: Self, budget: Vec<Weight>) -> OnlineAdversarialWBigraph<Key, Weight> {
        assert_eq!(
            budget.len(),
            self.u_nodes.len() // format!("budget = {:?},\n  graph = {:?}\n", budget, self)
        );
        OnlineAdversarialWBigraph {
            online_budget: budget,
            weighted_bigraph: self,
        }
    }
}

#[derive(Debug)]
pub struct OnlineAdversarialWBigraph<Key, Weight> {
    pub online_budget: Vec<Weight>,
    pub weighted_bigraph: WBigraph<Key, Weight>,
}

impl<'a, Key, Weight: Clone> OnlineAdversarialWBigraph<Key, Weight> {
    pub fn iter(self: &'a Self) -> OnlineAdversarialWBigraphIter<'a, Weight> {
        OnlineAdversarialWBigraphIter {
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
    pub fn ALG<Alg: OnlineAlgorithm<(usize, Weight), OfflineInfo<Weight>>>(
        self: &Self
    ) -> f64 {
        let mut alg = Alg::init(self.online_budget.clone());
        for online_adj in self.iter() {
            let _alg_choose = alg.dispatch(online_adj);
        }
        alg.alg_output()
    }
}

pub struct OnlineAdversarialWBigraphIter<'a, Weight> {
    pub online_adjacency_list: &'a Vec<Vec<(usize, Weight)>>,
    pub online_index: usize,
}

impl<'a, Weight> Iterator for OnlineAdversarialWBigraphIter<'a, Weight> {
    type Item = &'a Vec<(usize, Weight)>;

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
