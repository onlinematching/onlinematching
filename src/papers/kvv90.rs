pub mod algorithm {
    use rand::{
        distributions::{Standard, Uniform},
        thread_rng, Rng, rngs::ThreadRng,
    };

    use crate::bigraph::Dispatch;

    pub struct Random;
    impl Dispatch for Random {
        fn init() -> Self {
            Random
        }

        fn dispatch(self: &mut Self,v: &Vec<usize>) -> Option<usize> {
            if v.is_empty() {
                return None;
            }
            let mut rng = thread_rng();
            let val: usize = rng.sample(Uniform::new(0, v.len()));
            Some(v[val])
        }
    }

    pub struct Ranking {
        rng: ThreadRng
    }

    impl Dispatch for Ranking {
        fn init() -> Self {
            Ranking {
                rng: thread_rng()
            }
        }

        fn dispatch(self: &mut Self, v: &Vec<usize>) -> Option<usize> {
            let val: f64 = self.rng.sample(Standard);
            todo!()
        }
    }
}

pub mod example {}
