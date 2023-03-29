use self::algorithm::OnlineAlgorithm;
use crate::bigraph::Bigraph;

impl<Key> Bigraph<Key> {
    pub fn into_online(self: Self) -> OnlineAdversarialBigraph<Key> {
        let offline_size = self.u_nodes.len();
        let mut vec = Vec::with_capacity(offline_size);
        vec.resize(offline_size, true);
        OnlineAdversarialBigraph { bigraph: self }
    }
}

pub struct OnlineAdversarialBigraph<Key> {
    bigraph: Bigraph<Key>,
}

impl<'a, Key> OnlineAdversarialBigraph<Key> {
    pub fn iter(self: &'a Self) -> OnlineAdversarialBigraphIter<'a> {
        OnlineAdversarialBigraphIter {
            online_adjacency_list: &self.bigraph.v_adjacency_list,
            online_index: 0,
        }
    }

    #[allow(non_snake_case)]
    pub fn OPT(self: &Self) -> f64 {
        // temporary unsound
        self.bigraph.u_nodes.len() as f64
    }

    #[allow(non_snake_case)]
    pub fn ALG<Alg: OnlineAlgorithm<usize, usize>>(self: &Self) -> f64 {
        let mut alg = Alg::init(self.bigraph.u_nodes.len());
        for online_adj in self.iter() {
            // println!("{:?}", online_adj);
            let _alg_choose = alg.dispatch(online_adj);
        }
        alg.alg_output()
    }
}

pub struct OnlineAdversarialBigraphIter<'a> {
    online_adjacency_list: &'a Vec<Vec<usize>>,
    online_index: usize,
}

impl<'a> Iterator for OnlineAdversarialBigraphIter<'a> {
    type Item = &'a Vec<usize>;

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

pub mod algorithm {
    pub trait OnlineAlgorithm<T, OfflineInfo>
    where
        Self: Sized,
    {
        fn init(offline_info: OfflineInfo) -> Self;

        fn dispatch(self: &mut Self, online_adjacent: &Vec<T>) -> Option<usize>;

        fn alg_output(self: Self) -> f64;
    }
}
