pub mod algorithm {
    use rand::{
        distributions::{Standard, Uniform},
        rngs::ThreadRng,
        thread_rng, Rng,
    };

    use crate::{bigraph::Dispatch, papers::util};

    pub struct Random {
        offline_nodes_available: Vec<bool>,
    }

    impl Dispatch for Random {
        fn init(offline_size: usize) -> Self {
            let mut vec = Vec::with_capacity(offline_size);
            vec.resize(offline_size, true);
            Random {
                offline_nodes_available: vec,
            }
        }

        fn dispatch(self: &mut Self, online_adjacent: &Vec<usize>) -> Option<usize> {
            let available_offline_nodes = util::get_available_offline_nodes_in_onlineadj(
                &self.offline_nodes_available,
                online_adjacent,
            );
            if available_offline_nodes.is_empty() {
                None
            } else {
                let mut rng = thread_rng();
                let val: usize = rng.sample(Uniform::new(0, available_offline_nodes.len()));
                Some(available_offline_nodes[val])
            }
        }
    }

    // pub struct Ranking {
    //     _offline_nodes_available: Vec<bool>,
    //     _rng: ThreadRng,
    // }

    // impl Dispatch for Ranking {
    //     fn init(offline_size: usize) -> Self {
    //         let mut rng = thread_rng();
    //         let val: f64 = rng.sample(Standard);
    //         todo!()
    //     }

    //     fn dispatch(self: &mut Self, v: &Vec<usize>) -> Option<usize> {
    //         todo!()
    //     }
    // }
}

pub mod example {
    use crate::bigraph::Bigraph;

    /// N means the size of Graph, |U| = |V| = 2 * N
    /// This is a “blown-up” version of the simple
    /// 2 × 2 example on the left.
    /// Each side of the bipartition has n vertices
    /// divided into two parts of size n/2 each (U = U1 \cup U2 and V = V1 \cup V2 )
    /// There is a perfect matching between U and V
    /// (the i'th vertex in U and V have an edge between them).
    /// There is also a bipartite clique between V1 and U2 .
    /// It can be shown that Random achieves a **ratio** of 1/2 + o(1)
    pub fn random_worst_case(n: usize) -> Bigraph<usize> {
        let mut edges = Vec::new();
        for i in 0..(2 * n) {
            edges.push((i, i))
        }
        for u in 0..n {
            for v in n..(2 * n) {
                edges.push((u, v))
            }
        }
        Bigraph::from_edges(&edges)
    }
}
